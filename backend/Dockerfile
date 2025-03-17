FROM rust:1.85 AS planner

RUN cargo install cargo-chef

WORKDIR /app

# Copy the whole project
COPY . .

# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.85 AS builder

RUN cargo install cargo-chef

# Copy the build plan from the previous Docker stage
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --recipe-path recipe.json

COPY . .

RUN cargo build --release; \
    objcopy --compress-debug-sections target/release/backend ./main

FROM debian:bookworm-slim AS runner

WORKDIR /app

## copy the main binary
COPY --from=builder ./main ./

## ensure the container listens globally
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=9000

EXPOSE 9000

CMD ./main