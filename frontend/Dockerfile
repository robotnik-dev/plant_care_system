FROM --platform=$BUILDPLATFORM rust:1.85 AS base

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN rustup target add wasm32-unknown-unknown
RUN rustup target add $(uname -m)-unknown-linux-gnu

RUN cargo install sccache --version ^0.10  --target=$(uname -m)-unknown-linux-gnu
RUN cargo install cargo-chef --version ^0.1  --target=$(uname -m)-unknown-linux-gnu
RUN cargo install --locked trunk --version ^0.21 --target=$(uname -m)-unknown-linux-gnu

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
    trunk build --release;

FROM nginx:1.27-bookworm AS runner

COPY --from=builder app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]