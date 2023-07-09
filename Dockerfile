FROM rust:1.67

WORKDIR /usr/src/connect4_rust
COPY . .

RUN cargo install --path .
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
CMD ["connect4_rust", "--port", "8081"]