FROM rust:1.45.2

WORKDIR /usr/src/myapp
RUN mkdir cards
RUN mkdir hands
COPY hands/. hands/
COPY cards/. cards/
COPY Cargo.toml ./
COPY src/. ./src/

RUN cargo build --release

CMD ["./target/release/randomhand"]
