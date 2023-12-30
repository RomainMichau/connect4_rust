FROM rust:1.68

WORKDIR /usr/src/connect4_rust

COPY . .

RUN cargo install --path .

CMD ["connect4_rust", "--port", "8081"]
