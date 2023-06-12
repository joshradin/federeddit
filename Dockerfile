FROM rust
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bins
