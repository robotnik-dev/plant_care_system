FROM rust:1.85 AS build

WORKDIR /build

COPY . .

RUN cargo build --release; \
    objcopy --compress-debug-sections target/release/backend ./main

FROM debian:bookworm-slim

WORKDIR /app

## copy the main binary
COPY --from=build /build/main ./

## ensure the container listens globally
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=9000

EXPOSE 9000

CMD ./main