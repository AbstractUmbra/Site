FROM rust:1.71 as builder
WORKDIR /usr/src/site
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/site /usr/local/bin/site
CMD ["site"]
