FROM rust:1.56 as builder
WORKDIR /usr/src/rr_reloaded
COPY Cargo.lock Cargo.toml sqlx-data.json /usr/src/rr_reloaded/
COPY src/ /usr/src/rr_reloaded/src/
RUN cargo install --path .

FROM node:16.11.1 as npm_builder
WORKDIR /app
COPY frontend/ /app/
RUN npm run build

FROM debian:buster-slim
RUN apt-get update && apt-get install libssl-dev -y && rm -rf /var/lib/apt/lists/*
COPY --from=npm_builder /app/build /rr_reloaded/build
ENV FRONTEND_DIR /rr_reloaded/build
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/rr_reloaded
CMD ["rr_reloaded"]