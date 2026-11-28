-- Role and permission catalog.
--
-- Permission slugs here must match the constants in
-- src/application/shared/permissions.rs. SQL seeds cannot reference Rust
-- constants, so a new permission requires a matching entry here.

BEGIN;

INSERT INTO roles (slug, name, description, is_default)
VALUES
  ('super', 'Super Administrator', 'Unrestricted access to every operation, including destroying roles and API keys.', FALSE),
  ('admin', 'Administrator', 'Manages users, roles, catalog, inventory, and API keys, but cannot destroy roles or API keys.', FALSE),
  ('user', 'User', 'Read access to the catalog and stock, plus self-service profile management.', TRUE);

INSERT INTO permissions (slug, name, description)
VALUES
  ('role:create', 'Create roles', 'Create new roles.'),
  ('role:update', 'Update roles', 'Update role metadata and the default flag.'),
  ('role:delete', 'Delete roles', 'Delete roles.'),
  ('role:list', 'List roles', 'View roles and their granted permissions.'),
  ('role:grant', 'Grant role permissions', 'Grant and revoke permissions on roles.'),

  ('permission:create', 'Create permissions', 'Create new permissions.'),
  ('permission:list', 'List permissions', 'View permissions.'),

  ('user:create', 'Create users', 'Create users together with their credentials.'),
  ('user:update', 'Update users', 'Update user details, role, and account state.'),
  ('user:delete', 'Delete users', 'Delete users and their credentials.'),
  ('user:list', 'List users', 'View users.'),
  ('user:get', 'View user', 'View a single user.'),

  ('product:create', 'Create products', 'Create products.'),
  ('product:update', 'Update products', 'Update products.'),
  ('product:delete', 'Delete products', 'Delete products and their dependent records.'),
  ('product:list', 'List products', 'View products.'),
  ('product:get', 'View product', 'View a single product.'),

  ('variant:manage', 'Manage variants', 'Create, update, and delete product variants.'),
  ('selection:manage', 'Manage variant selections', 'Create, update, and delete variant selections.'),

  ('sku:update', 'Update SKUs', 'Change SKU codes.'),
  ('sku:delete', 'Delete SKUs', 'Delete SKUs and their prices and stock.'),
  ('sku:regenerate', 'Regenerate SKUs', 'Reconcile a product SKUs with its variant design.'),
  ('sku:list', 'List SKUs', 'View product SKUs.'),

  ('price:set', 'Set prices', 'Set global and per-location prices.'),
  ('price:remove', 'Remove prices', 'Remove prices.'),
  ('price:list', 'List prices', 'View prices.'),

  ('stock:adjust', 'Adjust stock', 'Apply stock adjustments.'),
  ('stock:list', 'List stock', 'View stock levels.'),
  ('movement:transfer', 'Transfer stock', 'Transfer stock between locations.'),
  ('movement:list', 'List stock movements', 'View the stock movement ledger.'),

  ('apikey:list', 'List API keys', 'View API keys.'),
  ('apikey:create', 'Create API keys', 'Create API keys.'),
  ('apikey:rotate', 'Rotate API keys', 'Rotate API keys.'),
  ('apikey:update', 'Update API keys', 'Enable or disable API keys.'),
  ('apikey:delete', 'Delete API keys', 'Delete API keys.'),

  ('location:create', 'Create locations', 'Create storage locations.'),
  ('location:update', 'Update locations', 'Update and deactivate locations.'),
  ('location:delete', 'Delete locations', 'Delete locations and their stock and prices.'),
  ('location:list', 'List locations', 'View locations.'),
  ('location:get', 'View location', 'View a single location.');

-- super holds every permission, including ones added later.
INSERT INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.slug = 'super';

-- admin holds everything except destroying roles and API keys.
INSERT INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.slug = 'admin'
  AND p.slug NOT IN ('role:delete', 'apikey:delete');

-- user holds read-only catalog and stock access.
INSERT INTO role_permission (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.slug = 'user'
  AND p.slug IN (
    'product:get', 'product:list',
    'sku:list',
    'price:list',
    'stock:list',
    'movement:list',
    'location:get', 'location:list'
  );

COMMIT;
