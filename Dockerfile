FROM rust:1.41 as builder
WORKDIR /usr/src/splitterrust_server
COPY . .

RUN apt-get update && \
    apt-get install -y postgresql postgresql-client

RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq5
COPY --from=builder /usr/local/cargo/bin/splitterrust_server /usr/local/bin/splitterrust_server
CMD ["splitterrust_server"]
