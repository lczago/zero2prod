FROM rust:1.93

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["./target/release/zero2prod"]