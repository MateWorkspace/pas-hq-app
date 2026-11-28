FROM rust:1.99-slim-bookworm AS build

RUN apt-get update \
    && apt-get install --yes --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src \
    && printf 'fn main() {}\n' > src/main.rs \
    && printf 'pub fn placeholder() {}\n' > src/lib.rs \
    && cargo build --release \
    && rm -rf src

COPY database ./database
COPY src ./src
RUN touch src/main.rs src/lib.rs \
    && cargo build --release \
    && strip target/release/pas-hq-app

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home --shell /usr/sbin/nologin pas-hq

COPY --from=build /app/target/release/pas-hq-app /usr/local/bin/pas-hq-app

USER pas-hq

ENV PASHQ_HTTP_ADDR=0.0.0.0:8080
EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/pas-hq-app"]
