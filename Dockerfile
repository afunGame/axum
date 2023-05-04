FROM rust:latest as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

WORKDIR /app

ADD . .


RUN cargo build --target x86_64-unknown-linux-musl --release

FROM busybox

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/axum .

CMD ["/app/axum"]
