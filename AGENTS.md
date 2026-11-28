# pas-hq-app Agentic Guidance

## About This Repository

`pas-hq-app` is the central backend for a point-of-sale and inventory platform.
A single installation aggregates many physical sites (outlets and warehouses),
the products those sites sell, the stock they hold, and the audits that explain
how that stock moved. It owns identity too: users, roles, permissions, and the
credentials those users authenticate with.

The service is a Rust 2024 crate (binary + library) built on Axum, SQLx and
PostgreSQL. It exposes one HTTP surface under `/v1` and nothing else yet — there
is no gRPC listener, no cron worker, no message broker, and no background queue.

What the service does today:

- **Identity and access.** Username/password sign-in issuing an HS256 access and
  refresh token pair, plus long-lived API keys (`pas-…`) that resolve to the
  same claims. Roles hold permissions through a grant table. Every
  administrative operation is authorized by a permission check against the
  caller's role.
- **Catalog.** Products, variant dimensions (Color, Storage), the selectable
  values of each dimension, and SKUs — the concrete sellable combination of one
  selection per variant. SKUs are *generated*, never authored: a product's SKU
  set is reconciled against its variant design by an explicit regeneration
  operation.
- **Pricing.** One price row per SKU and currency, optionally scoped to a
  location. A `NULL` location is the global price; lookups fall back from the
  outlet override to the global row.
- **Inventory.** Current stock per site and SKU, plus an append-only movement
  ledger that records transfers, sales, adjustments, and write-offs. Movements
  survive the deletion of the SKU or location they refer to.
- **Sites.** Locations with a `kind` of `store` or `warehouse`.

Design posture worth knowing before changing anything:

- Authorization is **data**, not control flow. There is no role-name bypass, no
  wildcard, and no `super` short-circuit in code. The `super` role is powerful
  only because the seeds grant it every permission.
- The database is deliberately permissive about deletion of business data
  (cascades); the application owns the guard, the user warning, and any
  write-off. Referential integrity protects *identity* and *audit history*, not
  stock.
- Permissions are never embedded in tokens. A token carries identity and role;
  the permission set is resolved from the role at authorization time and cached.
- `Session::whoami` re-reads the user and role from storage on every call, so a
  deactivation, suspension, or role change takes effect on the caller's next
  request instead of at access-token expiry. Sign-in, refresh, and `whoami`
  return the same `{ user, permissions }` shape, which lets a client resolve a
  session in one call. `whoami` deliberately avoids the `profile` name, which
  belongs to the self-service usecase group.

## Database Schema

Read the ERD before changing persistence. It is modular DBML:

```
docs/erd/main.dbml           Project block, `use * from` imports, and every Ref (FK)
docs/erd/01_auths.dbml       roles, permissions, role_permission, users, base_auths, api_keys
docs/erd/02_catalog.dbml     products, variants, selections, skus, sku_selections, prices
docs/erd/03_inventory.dbml   locations, inventories, stock_movements
```

`main.dbml` is the only file that declares relationships; the numbered files
declare tables and indexes only. Notes on a table record the behavioural rules
that the schema cannot express. Keep them accurate — several of them (SKU
identity, price fallback, movement snapshots, cascade semantics) are
load-bearing.

Fifteen tables, in dependency order:

| Table | Purpose |
|---|---|
| `roles` | Role identities. `slug` is the stable key; at most one row has `is_default = TRUE` (partial unique index). |
| `permissions` | Stable authorization actions such as `product:create`. Created, never updated or deleted. |
| `users` | Application identities. Exactly one `role_id`. `is_active` gates login; `suspended_until` suspends without deactivating. |
| `role_permission` | The only source of effective authorization. Unique per `(role_id, permission_id)`. |
| `base_auths` | Username/password credentials, one per user. Only the bcrypt hash is stored. |
| `api_keys` | Long-lived credentials. Stores a SHA-256 digest, a display-redacted preview, and `is_active`. Multiple per user, unique `(user_id, name)`. |
| `locations` | Sites that hold stock: `kind` is `store` or `warehouse`. |
| `products` | Product model identified by `(brand, model, manufacturer)`. |
| `product_variants` | Variant dimensions of a product; unique per `(product_id, name)`. |
| `product_variant_selections` | Selectable values of a dimension; unique per `(product_variant_id, name)`. |
| `product_skus` | The sellable unit. `selection_key` is its deterministic identity. |
| `product_sku_selections` | Binds one selection to one SKU. A SKU carries exactly one selection per variant; the application enforces that. |
| `prices` | `amount` is exact `NUMERIC`. Two partial unique indexes implement scoping. |
| `inventories` | Running balance per `(location_id, sku_id)`, unique. |
| `stock_movements` | Append-only ledger. `quantity` is a positive magnitude; direction comes from which location is set. |

### Rules the schema encodes

- **One default role.** `uq_roles_is_default` is a partial unique index on
  `is_default` `WHERE is_default = TRUE`.
- **Price scoping.** `NULL location_id` is the global price. Two partial unique
  indexes keep the scopes apart — `(sku_id, currency)` where the location is
  null, and `(sku_id, currency, location_id)` where it is not. A plain unique
  index would allow unlimited duplicate global rows because `NULL` is distinct
  from `NULL`. Reads resolve the outlet override first, then the global row
  (`ORDER BY location_id NULLS LAST`).
- **SKU identity.** `selection_key` is the sorted selection identifiers joined
  with `-` (empty for a product with no variants), unique per product. Because
  it is deterministic, a regeneration is a *diff*: unchanged keys keep their
  prices and stock, missing keys are inserted, and keys that disappeared are
  deleted. Renaming a variant therefore never destroys stock.
- **SKU regeneration is application work.** No foreign key can delete a SKU
  because its child count dropped, so deleting a variant or selection leaves
  SKUs in place. The application must recompute the valid key set and delete the
  SKUs whose key is not in it, inside the same transaction as the design change.
- **Movement snapshots.** `stock_movements` references are `ON DELETE SET NULL`
  and the row carries `sku_label`, `src_location_name`, and `dest_location_name`
  captured at write time, so history stays readable after its SKU or location is
  deleted. The application must populate those columns; they cannot be recovered
  later.
- **Cascades.** Deleting a product cascades variants, selections, SKUs, SKU
  selections, prices, and inventories. Deleting a location cascades its prices
  and stock. Deleting a SKU cascades its prices and stock. `stock_movements` is
  the only table that survives. `base_auths.user_id` and `api_keys.user_id` are
  `RESTRICT`, so a user with credentials or keys cannot be deleted implicitly.

### Migrations and seeds

`database/migrations/` holds the schema; `database/seeds/` holds ordered static
data. Both use SQLx's migrator and therefore the same versioning, checksum, and
locking semantics — but **separate version tables**:

| | directory | table |
|---|---|---|
| schema | `database/migrations/` | `_sqlx_migrations` (SQLx default) |
| data | `database/seeds/` | `schema_seeds` (set with `dangerous_set_table_name`) |

Seeds run *after* migrations because they depend on the schema. Never edit an
applied migration or seed: the checksum is recorded and SQLx will refuse to run.

Seed files are plain SQL wrapped in `BEGIN;`/`COMMIT;`, using ordinary `INSERT`
(a conflict must fail the version rather than silently skip) and resolving
foreign keys through natural-key subqueries, never hardcoded ids. Each `down`
deletes exactly the rows its `up` created, in reverse foreign-key order.

Current seeds:

- `20261129000000_seed_roles_and_permissions` — the `super`, `admin`, and `user`
  roles (`user` is the default), all 40 permissions, and the grants. `super`
  receives every permission through a `CROSS JOIN`, so a permission added later
  is covered automatically; `admin` receives everything except `role:delete` and
  `apikey:delete`; `user` receives read-only catalog and stock permissions.
- `20261129000001_seed_default_users` — the `super`, `admin`, and `user`
  accounts. Each uses an independently salted bcrypt hash. The deployment
  password is documented in the file header and **must be changed immediately
  after deployment**.

## Code Architecture and Conventions

### Clean Architecture Layout

Six top-level modules, one per layer, declared in `src/lib.rs`:

```
src/
  domain/          vocabulary + contracts; depends on nothing
  infrastructure/  contract implementations over drivers
  application/     usecase implementations; depends on domain only
  presentation/    HTTP adapters; depends on domain usecases only
  composition/     wiring; the only module that knows every layer
  config/          typed settings loaded from the environment
  main.rs          two lines: delegate to composition::launch
```

Dependency direction is one-way and enforced by review:

```
domain  <-  infrastructure
   ^            ^
application ----+        (application sees contracts, never adapters)
   ^
presentation  ->  only domain usecases

composition  ->  everything, once, in order
```

Rules that follow from this:

- Domain must not import `infrastructure`, `application`, `presentation`, or
  `composition`.
- Application must not import concrete adapters. It receives `Arc<dyn
  Contract>`.
- Presentation must not call repository or utility contracts, nor construct an
  application implementation.
- Composition is the only place that names concrete types, and the only place
  allowed to know the construction order.

Dependencies are injected by constructor. There is no service locator, no
global, no `lazy_static`. Each struct stores exactly the `Arc<dyn …>` it needs,
and composition builds them in dependency order.

### Domain

`src/domain/` has three concerns: `models/`, `contracts/`, `usecases/`.

**Models** — plain data plus classification.

- `models/entity/` — one file per table plus `claims.rs`. Structs are public
  with public fields and derive `Clone, Debug, PartialEq` (add `Eq` only when no
  `f64` or `BigDecimal` field is present). Audit columns are flattened onto the
  entity (`created_at`, `updated_at`, `created_by`, `updated_by`) rather than
  nested.
- Secrets never appear in `Debug`. `BaseAuth` and `ApiKey` hand-write `Debug` to
  omit `password_hash`/`hashed_key` and finish with `finish_non_exhaustive()`.
- Enumerations that map to a `TEXT` column implement `as_str()`, `Display`, and
  `FromStr` with a dedicated `Parse*Error` implementing `Error`. Examples:
  `Level`, `Format`, `LocationKind`, `MovementReason`, `CacheBackend`.
- `models/app/` — `AppContext`, `AppEnv`, `AppInfo`.
- `models/error/` — one file per error family. Every enum exposes `code()`
  returning a SCREAMING_SNAKE_CASE string, implements `Display` with a
  diagnostic sentence, and implements `StdError`. `UsecaseError` aggregates the
  other families and delegates `code()`.

**Contracts** — traits only, no implementations.

- `contracts/repository/` — one file per aggregate, plus `future.rs`
  (`RepositoryFuture<'a, T>` = boxed `Future<Output = Result<T,
  RepositoryError>>`) and `Pagination`. Every method takes `&AppContext` first,
  then `Option<&'a mut dyn DbExecutor>`, then its arguments. Boxed futures keep
  the traits object-safe so composition can store `Arc<dyn Repository>`.
- `contracts/utility/` — `Logger`, `Password`, `Token`, `ApiKey`,
  `AppContextFactory`, and the transactor (`DbExecutor` + `Transactor`).
- `contracts/caching/` — `Cache` (byte-oriented, async) and `ApiKeyClaimsCache`
  (typed on `AccessClaims`).
- `contracts/validator/` — one trait per group: `AuthValidator`,
  `RoleValidator`, `PermissionValidator`, `UserValidator`, `ProductValidator`,
  `LocationValidator`, `ApiKeyValidator`. Each method is `(&AppContext, &str) ->
  Result<(), ValidatorError>`.

**Usecases** — the application-facing surface. One trait per capability,
grouped:

| Group | Traits |
|---|---|
| `auth` | `Session` (sign_in, validate_access, refresh, profile), `ApiKeyAuth` (validate_api_key) |
| `access` | `ManageRole`, `ManagePermission`, `ManageUser`, `ManageApiKey` |
| `profile` | `ManageAccount`, `ChangePassword`, `ManageOwnApiKey` |
| `catalog` | `ManageProduct`, `ManageVariant`, `ManageVariantSelection`, `ManageSku`, `RegenerateProductSkus`, `ManagePrice` |
| `inventory` | `GetStock`, `AdjustStock`, `ListStock`, `ListMovements`, `RecordMovement` |
| `location` | `ManageLocation` |
| `misc` | `CreateAppContext` |
| `shared` | `PermissionGuard`, `UsecaseFuture`, `Page` |

Each trait is followed by its request/result structs. Requests express domain
operations: they omit server-owned ids on create and immutable fields (slugs) on
update. Results contain only values suitable for presentation mapping. All
traits are object-safe and context-first.

`CreateAppContext` shapes the rest of the request pipeline:

- `create(actor, trace, claims)` — explicit.
- `create_open(trace)` — actor `open`, no claims. Used for sign-in and refresh.
- `create_system(trace)` — actor `system`, no claims. Used while validating a
  credential, before an identity exists.
- `create_authenticated(claims, trace)` — actor becomes the user's **name**,
  claims attached. This is the single place that turns validated claims into a
  user context, so the convention cannot drift between the bearer and API-key
  paths.

### Infrastructure

`src/infrastructure/` implements the domain contracts over concrete libraries.
It must not contain business rules and must not know about HTTP.

**Repositories** — `repository/<entity>/{mod,postgres,postgres_query}.rs`:

- `postgres_query.rs` builds SQL with `sqlx::QueryBuilder` and never executes
  it. A `const COLUMNS` and a `select()` helper head each file; list filters use
  the shared `push_name_contains` / `push_pagination` helpers.
- `postgres.rs` holds `struct PostgresX { database: PgPool, logger: Arc<dyn
  Logger> }` and implements the contract. Every method follows the same shape:

  ```rust
  Box::pin(async move {
      let mut pool_executor = PoolExecutor::new(self.database.clone());
      let executor = match executor {
          Some(executor) => &mut *executor,
          None => &mut pool_executor,  // NULL means the repository's own pool
      };
      let result: Result<T, OperationError> = async { /* work */ }.await;
      finish(self.logger.as_ref(), context, "repository/x/y", result,
             || RepositoryError::XNotFound)
  })
  ```

  `finish` logs `sql_state`, `constraint`, and `error_code` once and maps the
  driver error to a classified `RepositoryError`. `require_affected` turns a
  zero-row `execute` into the aggregate's not-found error.
- `shared/error.rs` is the only place that translates SQLSTATE. A unique
  violation maps by constraint name, a foreign-key violation maps to the missing
  parent, `23001` (RESTRICT) and `23503` both surface as conflicts or not-found
  as appropriate, `23502/23514/22P02/…` are `BadArgs`, pool timeouts are
  `Timeout`, everything else is `Failure`. When adding a constraint, add its
  mapping here or it becomes a plain `Conflict`/`Failure`.
- `shared/scan/<entity>.rs` decodes a `PgRow` into an entity with `try_get`.
  Enums are read as `String` and parsed, with an unknown value surfacing as
  `sqlx::Error::Decode`.
- `shared/cache.rs` provides `CacheNamespace` + `InvalidationGuard`, a
  generation-based namespace used by the cache-aside decorator.

**Caching** — `caching/`:

- `cache/moka.rs` and `cache/redis.rs` both implement `Cache`. Moka is
  in-process; Redis is out-of-process with per-kind TTL (`SET … EX`). Cache
  failures are logged and treated as misses, never as domain errors.
- `api_key.rs` wraps any `Cache` to store `AccessClaims` under the key digest.
  Entries are removed by exact key, so no namespace is needed.

**Utilities** — `utility/`:

- `logger/` — `WriterLogger::new(writer, format, level)`. One line per call,
  JSON or plain, always `time_unix_ms`, `level`, `msg`, `tag`, `actor`,
  `execution_id`, `trace_id`, `meta`. Absent context values become empty
  strings; absent metadata becomes `{}`. Writes are synchronous and may block;
  failures are silent.
- `password/` — `BcryptPassword::new(cost)`. Rejects passwords past bcrypt's
  72-byte boundary instead of truncating; separates `Mismatch` from
  `CompareFailed`.
- `token/` — `JwtToken::new(secret, access_ttl, refresh_ttl)`. HS256 for both
  token kinds, separated by a `token_type` claim, with `exp`/`nbf` required and
  no leeway.
- `api_key/` — `Sha256ApiKey`. Keys are `pas-` plus 32 random bytes in URL-safe
  base64; the digest is SHA-256 hex (deterministic, because it is both the
  lookup key and the cache key); the preview keeps the first 8 and last 4
  characters.
- `transactor/` — `DbExecutor` is the domain trait; `PoolExecutor` and
  `TransactionExecutor` implement it; `PgTransactor::with_tx` begins a
  transaction, runs the callback, commits on `Ok` and rolls back on `Err`,
  preserving the callback error.
- `validator/` — `RegexAuthValidator`, `RegexRoleValidator`,
  `RegexPermissionValidator`, `RegexUserValidator`, `RegexProductValidator`,
  `RegexLocationValidator`, `RegexApiKeyValidator`. Limits live in
  `validator/shared.rs`. A rejection logs once at ERROR with `error_code` and
  `reason`; valid input never touches the logger.
- `app_context/` — `UuidAppContextFactory`, which generates a fresh execution
  id.

### Application

`src/application/<group>/<capability>.rs`, mirroring the usecase groups. One
struct per usecase trait, named `<Capability>Management` or
`<Capability>Usecase`, holding only `Arc<dyn …>` fields and implementing the
domain trait.

Every method follows the same five steps, in this order:

1. **Authorize.** Call
   `self.permission_guard.require(context, &[PERMISSION_CONST]).await?` with a
   hard-typed permission from `application/shared/permissions.rs`. Profile
   operations (own account, own password, own API keys) have no guard — the
   target is always `context.actor_id()`, and ownership is re-checked before any
   mutation.
2. **Validate.** Call the relevant validator method, log the failure, and return
   `UsecaseError::Validation`.
3. **Write.** Repository calls pass `None` for the executor, or `Some(executor)`
   inside a transaction.
4. **Log the outcome.** `info` for every create/update that succeeded (with the
new
   id in metadata), `warn` where something destructive or best-effort happened,
   `error` on every failure.
5. **Map errors.** `RepositoryError`/`PasswordError`/`TokenError`/`ApiKeyError`
are
   widened into `UsecaseError` so presentation can classify by `code()`.

Hard rules:

- **No logging helper builds a message.** Tags and sentences are written out at
  each call site, e.g. `self.logger.info(context,
  "application/catalog/product/create", "product created", Some(&meta))`. Only
  the `error_code` metadata is shared.
- **Multi-write operations go through the transactor.** `create_user`,
  `delete_user`, `adjust_stock`, `transfer_stock`, and `regenerate_product_skus`
  open one transaction and pass `Some(executor)` into every repository call.
- **SKU regeneration owns the diff.** It computes the valid selection-key set,
  logs the plan (`created_count`, `removed_count`, `destroys_stock`), returns
  early on `dry_run`, warns when stock would be destroyed, then deletes the
  disappeared SKUs and inserts the new ones in one transaction. Variant and
  selection usecases deliberately do *not* regenerate; they log that
  regeneration is now required.
- **Cache invalidation is explicit.** Rotating, disabling, or deleting an API
  key removes its digest entry. Deleting a user removes every key of that user
  inside the transaction. Updating a user's role, active state, or suspension
  invalidates all of their keys, so a role change takes effect immediately
  instead of after the TTL.
- **Transactions own rollback.** Never swallow an error inside a `with_tx`
  callback; the callback's error type is `UsecaseError`, which implements
  `From<sqlx::Error>` so begin and commit failures surface safely.

### Presentation

`src/presentation/http/` is an Axum adapter. It parses transport input, calls
one usecase, and maps the result. No business rules, no repository access, no
duplicate logging.

```
routes.rs            Router assembly, public vs protected groups, 404/405
mod.rs               Dependencies (one Arc<dyn Usecase> per trait)
middleware/
  header.rs          single_header: at most once, non-blank ASCII
  trace_id.rs        x-trace-id -> Option<Uuid>
  bearer.rs          Authorization: Bearer <token>
  api_key.rs         x-api-key
  context.rs         open() / authenticated() composition + RequestContext
dto/request/         per-group query/body DTOs, flat PageQuery primitives
dto/response/        per-group wire models, error.rs (ApiError), page.rs, model.rs
handler/<group>/     <resource>.rs + <resource>_errmap.rs
```

**Request pipeline.**

- Public routes (`/v1/auth/sign-in`, `/v1/auth/refresh`, `/v1/version`) run
  `open()`: read `x-trace-id`, build a context with actor `open`, insert it into
  request extensions. `/v1/auth/me` is **protected**: it resolves the caller's
  live profile, so it needs validated claims.
- Every other route runs `authenticated()`: read `x-trace-id`, read the bearer
  token, read `x-api-key`, then **prefer the bearer token** when both are
  present. Claims are resolved under a *system* context, because no identity
  exists until validation succeeds, then `create_authenticated` builds the user
  context.
- `x-trace-id` is optional but validated when present: a single non-zero UUID.
- The middleware inserts `AppContext` into `Request::extensions`. Handlers
  extract it with `RequestContext(pub AppContext)`, which implements
  `FromRequestParts`. A missing context is a loud 500, not an empty context.
- A panic anywhere in a handler becomes a safe 500 through `CatchPanicLayer`.
  The body limit is 1 MiB.

**DTOs.** Request structs derive `Deserialize`, reject unknown fields where
possible, and implement `From` into the usecase request. Response structs derive
`Serialize` and are built by an explicit mapper function; entities are never
serialized directly. Conventions:

- Lists return `{ "data": [...], "page": { "limit", "offset" } }`.
- Creates return `201`, updates `200`, deletes `204`.
- Timestamps are RFC 3339 strings.
- Price amounts travel as text in both directions and are parsed into
  `BigDecimal`, so no precision is lost.
- Enums travel as their stored strings (`store`, `write_off`) and are parsed,
  producing a 400 with a specific message on an unknown value.
- List paging is declared as flat `limit`/`offset` fields on each list DTO and
  funneled through `page_from(limit, offset)`, which defaults to 10 and clamps
  to 100. Do **not** reintroduce `#[serde(flatten)]` for paging: the query
  extractor uses `serde_urlencoded`, which cannot deserialize flattened structs,
  and the endpoint will reject every query string.
  `tests/unit/presentation/query.rs` guards this.

**Errors.** There is no central status table. Each handler file has a sibling
`*_errmap.rs` with **one function per handler function**, e.g.
`create_product_error(UsecaseError) -> ApiError`. Each function matches the
specific `ValidatorError`/`RepositoryError`/`PasswordError`/`TokenError`
variants it can produce and returns an `ApiError` carrying a status, a stable
code, and a complete human-readable sentence. The catch-all arm returns a
generic 500 message with a fixed code; source chains never reach the client.
This keeps every mapping traceable to a single call site.

`ApiError` itself only holds `{ status, code, message }` and implements
`IntoResponse`; it also converts the Axum `Json`/`Path`/`Query` rejections into
400s.

**Current routes** (all under `/v1`):

```
GET    /version                      (public: build name and version only)

POST   /auth/sign-in                 POST   /auth/refresh
GET    /auth/me                      (protected: live user + permissions)

GET    /profile                      PATCH  /profile
PUT    /profile/password
GET    /profile/api-keys             POST   /profile/api-keys
PATCH  /profile/api-keys/{id}        DELETE /profile/api-keys/{id}
POST   /profile/api-keys/{id}/rotate

GET    /roles                        POST   /roles
PATCH  /roles/{id}                   DELETE /roles/{id}
GET    /roles/{id}/permissions       POST   /roles/{id}/permissions
DELETE /roles/{id}/permissions/{permission_id}
GET    /permissions                  POST   /permissions
GET    /users                        POST   /users
GET    /users/{id}                   PATCH  /users/{id}
DELETE /users/{id}
GET    /users/{id}/api-keys
GET    /api-keys                     POST   /api-keys
PATCH  /api-keys/{id}                DELETE /api-keys/{id}
POST   /api-keys/{id}/rotate

GET    /products                     POST   /products
GET    /products/{id}                PATCH  /products/{id}
DELETE /products/{id}
GET    /products/{id}/variants
GET    /products/{id}/skus           POST   /products/{id}/skus/regenerate
POST   /variants                     PATCH  /variants/{id}
DELETE /variants/{id}
GET    /variants/{id}/selections
POST   /selections                   PATCH  /selections/{id}
DELETE /selections/{id}
PATCH  /skus/{id}                    DELETE /skus/{id}
GET    /skus/{id}/prices
POST   /prices                       DELETE /prices
GET    /prices/effective

POST   /stock/adjustments            POST   /stock/transfers
GET    /stock/by-location/{id}       GET    /stock/by-sku/{id}
GET    /stock/movements/by-location/{id}
GET    /stock/movements/by-sku/{id}

GET    /locations                    POST   /locations
GET    /locations/{id}               PATCH  /locations/{id}
DELETE /locations/{id}
POST   /locations/{id}/deactivate
```

`/products/{id}/skus/regenerate` accepts `?dry_run=true` to return the plan
without writing.

### Composition

`src/composition/main/` is the only module that constructs concrete types. It is
seven focused files, wired strictly in order:

```
driver.rs          stdout WriterLogger; PgPool from AppEnv (PgConnectOptions,
                   application_name from CARGO_PKG_NAME, statement logging off)
infrastructure.rs  cache backend (Moka or Redis), every Postgres* repository,
                   CachedRolePermission, CachedApiKeyClaims, all validators
application.rs     every usecase implementation, in dependency order
presentation.rs    the Router, from filled http::Dependencies
launcher.rs        driver -> infrastructure -> application -> presentation
runner.rs          settings, migrations, seeds, listener, signals, pool cleanup
mod.rs             module declarations + public re-exports
```

`src/main.rs` does nothing but delegate:

```rust
#[tokio::main]
async fn main() -> ExitCode {
    pas_hq_app::composition::launch().await
}
```

Startup sequence in `runner.rs`:

1. Load `AppEnv` (`config::env::load`) and `AppInfo` (`config::app::load`). A
   configuration failure is reported to stderr and exits non-zero, before a
   logger exists.
2. Build the logger from settings and log `service starting` with name, version,
   and address.
3. Build drivers (pool), then infrastructure, application, and presentation. 4.
Apply migrations (`_sqlx_migrations`), then seeds (`schema_seeds`). 5. Bind
`PASHQ_HTTP_ADDR`, log `http listener bound` with the resolved address. 6. Serve
with graceful shutdown on SIGINT/SIGTERM. 7. Drain within
`PASHQ_SHUTDOWN_TIMEOUT_MS`, then close the pool with the same
   deadline. Cleanup also runs after a failure, without replacing the original
   error.

`launch_with(env, shutdown)` accepts an injected shutdown future for tests.

`LaunchError` classifies `Signal`, `Database`, `Cache`, `Migration`, `Seed`,
`Bind`, `Serve`, `ServerStopped`, and `ShutdownTimeout`. A normal signal-driven
shutdown exits zero; every failure exits non-zero. Lifecycle logs never dump
settings, passwords, connection strings, or request data.

### Environment Variables

Configuration is read in exactly one place: `src/config/env.rs`. Nothing else
may call `env::var` or read `.env`.

Flow:

1. `config::env::load()` reads an optional `.env` from the working directory;
   `load_from_path(path)` takes an explicit path for tooling.
2. Process environment variables take precedence over `.env` values. 3.
`from_sources` applies a typed default, validates, and builds `AppEnv`. 4.
Values are then injected into constructors by composition. Layers never
   re-read the environment.

`AppEnv` deliberately does not derive `Debug`, because it holds secrets.
`LoadError` never carries a rejected value or a raw `.env` line — only the
variable name and a safe reason — so configuration errors can be logged safely.

All variables use the `PASHQ_` prefix. See `.env.example` for the authoritative
list.

| Variable | Default | Notes |
|---|---|---|
| `PASHQ_LOGGER_FORMAT` | `plain` | `plain` or `json` |
| `PASHQ_LOGGER_LEVEL` | `INFO` | `NONE`, `ERROR`, `WARN`, `INFO`, `DEBUG` |
| `PASHQ_POSTGRES_HOST` | `127.0.0.1` | |
| `PASHQ_POSTGRES_PORT` | `5432` | 1–65535 |
| `PASHQ_POSTGRES_USERNAME` | — | required |
| `PASHQ_POSTGRES_PASSWORD` | — | required; preserved byte-for-byte |
| `PASHQ_POSTGRES_DATABASE` | `pas_hq` | |
| `PASHQ_POSTGRES_SSL_MODE` | `prefer` | `disable`, `allow`, `prefer`, `require`, `verify-ca`, `verify-full` |
| `PASHQ_POSTGRES_MAX_CONNECTIONS` | `10` | positive |
| `PASHQ_POSTGRES_CONNECT_TIMEOUT_MS` | `10000` | positive milliseconds |
| `PASHQ_CACHE_BACKEND` | `moka` | `moka` or `redis` |
| `PASHQ_CACHE_MAX_CAPACITY` | `10000` | Moka only |
| `PASHQ_CACHE_INDIVIDUAL_TTL_MS` | `60000` | |
| `PASHQ_CACHE_LIST_TTL_MS` | `10000` | |
| `PASHQ_REDIS_URL` | `redis://127.0.0.1:6379` | must be non-blank when the backend is redis |
| `PASHQ_AUTH_ACCESS_SECRET` | — | required, non-blank; signs both token kinds |
| `PASHQ_AUTH_ACCESS_TTL_MS` | `900000` | 15 minutes |
| `PASHQ_AUTH_REFRESH_TTL_MS` | `86400000` | 24 hours |
| `PASHQ_AUTH_BCRYPT_COST` | `12` | out-of-range falls back to bcrypt's default |
| `PASHQ_AUTH_API_KEY_CACHE_TTL_MS` | `60000` | how long resolved key claims stay cached |
| `PASHQ_HTTP_ADDR` | `0.0.0.0:8080` | socket address |
| `PASHQ_SHUTDOWN_TIMEOUT_MS` | `15000` | positive milliseconds |

### Versioning Convention

`Cargo.toml`'s `version` tracks the deployment stage:

- `X.Y.Z-dev.N` — development builds. The current version is `0.1.0-dev.1`.
- `X.Y.Z-stg.N` — staging builds.
- `X.Y.Z` — production. No suffix at all.

`config::app::load()` reads `CARGO_PKG_NAME` and `CARGO_PKG_VERSION`, so the
running version is whatever `Cargo.toml` says at build time. It is logged at
startup and must never be hardcoded anywhere else.

## Tests

All tests live under `tests/unit/`, compiled as one binary declared in
`Cargo.toml` (`autotests = false`, `[[test]] name = "unit"`). Organize files to
mirror `src/`:

```
tests/unit/main.rs          mod application; mod config; mod domain;
                            mod infrastructure; mod presentation;
tests/unit/domain/          models, error codes, cache contract,
                            repository and usecase object safety
tests/unit/infrastructure/  logger, password, token, transactor, validators,
                            cache, app context, repository decorator
tests/unit/application/     permission guard, api key authentication
tests/unit/presentation/    router behavior, query parsing
tests/unit/config/          placeholder: config behavior is covered inline
```

Conventions:

- Tests are black-box: they use `pas_hq_app::…` public paths, never `crate::`.
- Doubles are local to the test file. There is no shared fake library; each file
  defines the stubs it needs (see `tests/unit/presentation/router.rs` for a full
  set of usecase stubs).
- Object safety is asserted directly, e.g. `let _: Option<&dyn RoleRepository> =
  None;` — this fails to compile if a contract stops being dyn-compatible.
- Private behavior (SQLSTATE classification, cache key generation, logger
  expiry) is covered by inline `#[cfg(test)] mod tests` inside `src/`, so
  private items need no widening.
- The Redis adapter has an `#[ignore]`d round-trip test gated on
  `PAS_HQ_APP_TEST_REDIS_URL`; it is skipped when no Redis is available. Run it
  with `PAS_HQ_APP_TEST_REDIS_URL=redis://127.0.0.1:6379 cargo test --
  --ignored`. No test requires a database: repositories are exercised through
  their decorators and the router through stubs.

Run from this directory:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`cargo test` runs the inline library tests and the `unit` binary. Nothing else
is required to verify a change. If you touch the schema, also apply the
migrations and seeds against a disposable PostgreSQL and confirm both version
tables:

```sh
docker run --detach --rm --name pas-hq-test \
  -e POSTGRES_PASSWORD=test-only -e POSTGRES_DB=pas_hq \
  -p 127.0.0.1:35432:5432 postgres:18

# boot the binary against it, then check:
#   SELECT count(*) FROM _sqlx_migrations;
#   SELECT count(*) FROM schema_seeds;
```

## Deployment

There is currently **no Dockerfile, compose file, or CI pipeline** in this
repository. Do not assume one exists. Deployment today is: build the binary,
provide the environment, run it against a prepared PostgreSQL, and terminate it
with SIGTERM.

Build and run:

```sh
cargo build --release
cp .env.example .env          # then fill in the required secrets
./target/release/pas-hq-app
```

The binary:

- reads `.env` from the working directory (optional) and then the process
  environment, which wins;
- applies pending migrations and then pending seeds automatically — no separate
  migrate step is required or provided;
- binds `PASHQ_HTTP_ADDR` and logs the resolved address;
- exits zero on SIGINT/SIGTERM after draining, non-zero on any failure.

A container deployment would need, at minimum: a multi-stage build producing the
release binary, the `database/` directory available at build time (both
`sqlx::migrate!` calls are compile-time macros that embed the migration and seed
directories), no writable filesystem, a non-root user, and the same environment
variables passed at runtime. Never bake `.env`, `PASHQ_AUTH_ACCESS_SECRET`, or
database credentials into an image.

## Working on this project

When changing something, walk the layers in order and keep the contract surface
honest:

1. **Schema** — update the DBML in `docs/erd/`, then add a new migration (never
   edit an applied one). Record behavioural rules in the table note.
2. **Domain** — entity, error variants and `code()`s,
repository/validator/usecase
   contracts, request/result types.
3. **Infrastructure** — query builders, adapter, scanner, and the SQLSTATE
mapping
   for any new constraint.
4. **Application** — the usecase implementation: guard, validate, write, log,
map.
   Add any new permission slug to `application/shared/permissions.rs`.
5. **Presentation** — request DTO, response model, handler, and the matching
   `*_errmap.rs` function.
6. **Composition** — wire the new dependency through `infrastructure.rs`,
   `application.rs`, `presentation.rs`, and update `http::Dependencies`.
7. **Seeds** — if the change introduces permissions, extend the roles seed so
the
   catalog stays complete.
8. **Tests** — cover the new behaviour, keep object-safety assertions current.
9. **Verify** — `fmt`, `clippy -D warnings`, `test`.

Things that are deliberately not here yet, so do not invent them by analogy:

- No gRPC, no cron, no queue, no WebSocket, no broker.
- No Docker or CI assets.
- No integration test harness against a real database.
- No refresh-token revocation list and no token version column; disabling a user
  takes effect on the next sign-in, refresh, or API-key validation, not on an
  issued access token.
- No per-key scopes: an API key always carries its owner's current role.
- No audit log table for administrative actions; application logs are the audit
  trail.
- The `role_permission` cache invalidates in-process only; a multi-instance
  deployment relies on the TTL to bound staleness.
