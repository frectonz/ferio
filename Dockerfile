FROM rust

WORKDIR /app

COPY . .

RUN cargo build --release --workspace --exclude ferio-cli

EXPOSE $PORT

CMD ["./target/release/ferio-api"]

