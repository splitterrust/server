FROM rust:1.41 as builder
WORKDIR /usr/src/splitterrust_server
COPY . .

RUN apt-get update && \
    apt-get install -y postgresql postgresql-client

RUN cargo install --path .

FROM debian:buster-slim
RUN groupadd -r splitterrust && useradd -r -s /bin/false -g splitterrust splitterrust
RUN apt-get update && apt-get install -y libpq5

ENV SERVER /usr/local/bin/splitterrust_server

COPY --from=builder /usr/local/cargo/bin/splitterrust_server $SERVER
COPY docker_entrypoint.sh /user/local/bin/

RUN ln -s usr/local/bin/docker_entrypoint.sh / # backwards compat

EXPOSE 8088

#ENTRYPOINT ["docker_entrypoint.sh"]

CMD ["splitterrust_server"]