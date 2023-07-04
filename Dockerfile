FROM rust as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bins
RUN cargo install --path . --root /usr/local/federeddit/apps

FROM busybox
WORKDIR /bin
COPY --from=builder /usr/local/federeddit/apps .