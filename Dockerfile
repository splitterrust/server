FROM rust:1.41 as builder

WORKDIR /usr/src/

# create blank project
RUN USER=root cargo new splitterrust_server

# Install deps, this will cache them so we can
# build the container faster for development.
COPY Cargo.toml /usr/src/splitterrust_server/

WORKDIR /usr/src/splitterrust_server

# Remove last line, which is our local library (for the moment)
RUN sed -i '$ d' Cargo.toml

RUN cargo build --release

# Now copy source files and install the application.
# It's important, that Cargo.toml will be pulled again
# so the last line is present again.
COPY . .

# This is the actual build.
RUN cargo build --release

# The smaller image with only the binary and all needed deps
FROM debian:buster-slim
RUN groupadd -r splitterrust && useradd -r -s /bin/false -g splitterrust splitterrust
RUN apt-get update && apt-get install -y libpq5

ENV SERVER /usr/local/bin/splitterrust_server

COPY --from=builder /usr/src/splitterrust_server/target/release/splitterrust_server $SERVER
COPY docker_entrypoint.sh /usr/local/bin/

RUN ln -s /usr/local/bin/docker_entrypoint.sh / # backwards compat

EXPOSE 8088

ENTRYPOINT ["docker_entrypoint.sh"]

CMD ["splitterrust_server"]
