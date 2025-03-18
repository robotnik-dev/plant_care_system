FROM --platform=$BUILDPLATFORM lukemathwalker/cargo-chef:latest-rust-1 AS chef

ARG TARGETPLATFORM
ARG BUILDPLATFORM

WORKDIR /app

RUN cargo install sccache --version ^0.10 --target $(uname -m)-unknown-linux-gnu

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
    cargo chef cook --release --recipe-path recipe.json --target $(uname -m)-unknown-linux-gnu;

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --bin backend --target $(uname -m)-unknown-linux-gnu; \
    objcopy --compress-debug-sections ../target/$(uname -m)-unknown-linux-gnu/release/backend ./main;

FROM debian:bookworm-slim AS runner

## copy the main binary
COPY --from=builder app/main /usr/local/bin

## ensure the container listens globally
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=9000

EXPOSE 9000

ENTRYPOINT ["./main"]
