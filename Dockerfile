FROM rust as builder
WORKDIR /usr/src/app
COPY Cargo.toml .
COPY Cargo.lock .
COPY crates crates
RUN --mount=type=cache,target=/usr/local/cargo \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release --bins
RUN cargo install --path crates/coordinator --root /usr/local/federeddit/
RUN cargo install --path crates/users-service --root /usr/local/federeddit/

FROM busybox
WORKDIR /federeddit-apps/
COPY --from=builder /usr/local/federeddit/bin .
RUN ls