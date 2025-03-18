FROM --platform=$BUILDPLATFORM rust:1.85 AS base

ARG BUILDPLATFORM
ARG TARGETPLATFORM

RUN rustup target add $(uname -m)-unknown-linux-gnu

RUN cargo install sccache --version ^0.10 --target=$(uname -m)-unknown-linux-gnu

RUN cargo install cargo-chef --version ^0.1 --target=$(uname -m)-unknown-linux-gnu

ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM base AS planner

WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

FROM base AS builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --target=$(uname -m)-unknown-linux-gnu; \
    objcopy --compress-debug-sections target/$(uname -m)-unknown-linux-gnu/release/backend ./main

FROM builder AS runner

WORKDIR /app

## copy the main binary
COPY --from=builder app/main ./

## ensure the container listens globally
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=9000

EXPOSE 9000

CMD ["./main"]
