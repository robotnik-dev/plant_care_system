FROM rust:1.85 AS build

WORKDIR /build

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

COPY . .

RUN cargo install --path .

RUN trunk build --release

FROM nginx:1.27-bookworm

COPY --from=build /build/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]