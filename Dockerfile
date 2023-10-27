# FROM rust:1.73 as builder
FROM rust:1.73
WORKDIR /usr/src/site
COPY . .
RUN cargo install --path .
CMD ["site"]

# FROM debian:bullseye-slim
# RUN apt update -y && apt upgrade -y && apt install build-essential libc-bin libc6 -y && apt clean && apt autoremove
# COPY --from=builder /usr/local/cargo/bin/site /usr/local/bin/site
# CMD ["site"]
