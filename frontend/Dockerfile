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

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN rustup target add wasm32-unknown-unknown

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo install --locked trunk --version ^0.21

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json;

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cd frontend; \
    trunk build --release;

FROM nginx:1.27-bookworm AS runner

COPY --from=builder app/frontend/dist /usr/share/nginx/html
COPY frontend/nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]