FROM rust:1.56 as builder
WORKDIR /usr/src/myapp
COPY .. .
RUN cargo install --path .

FROM node:16:11:1
WORKDIR /app
COPY ../frontend /app/
RUN npm run build

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/rr_reloaded
CMD ["rr_reloaded"]