FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /app

RUN cargo install sccache --version ^0.10

ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM chef AS planner

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json;

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --bin backend;

FROM debian:bookworm-slim AS runner

## copy the main binary
COPY --from=builder app/target/release/backend ./
COPY --from=builder app/backend/Rocket.toml ./static

EXPOSE 9000

CMD ["./backend"]