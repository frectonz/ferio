FROM rust

WORKDIR /app

COPY . .

RUN cargo build --release --workspace --exclude ferio-cli

CMD ["./target/release/ferio-api"]

