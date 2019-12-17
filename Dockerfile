ARG BASE_CONTAINER=splitterrust_db:latest
FROM $BASE_CONTAINER

WORKDIR /usr/src/server

COPY . .

RUN cargo install --path .

CMD ["splitterrust_server"]
