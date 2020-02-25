# Splitterrust-Server

![Travis-CI][https://travis-ci.org/splitterrust/server.svg?branch=master]

This server should serve all information about the game. This are classes,
abilities, items, spells, etc....

## Features

- [ ] Classes
- [ ] Abilities
- [ ] Items
- [x] Spells
- [ ] Monsters/NPCs
- [ ] Resources

## Docker

### Building

To build the release version:
```
$ docker buid splitterrust_server:latest .
```

To run it:
```
$ docker run -p 8088:8088 -e \
    DATABASE_URL=postgres://splitterrust@localhost/splitterrust \
    splitterrust_server:latest
```

### Environment

#### `DATABASE_URL` (required)

The url for the database. Currently only Postgres is supported.

```
DATABASE_URL=postgres://splitterrust@localhost/splitterrust
```

#### `RUST_LOG`

Log level for the application.

Set everythig to one level:
```
RUST_LOG="debug"
```

Set just splitterrust_server to a level:

```
splitterrust_server=debug
```

Set multiple level:
```
splitterrust_server=debug,tokio_reactor=debug
```

### docker-compose

There is an example `docker-compose.yml` which will build a complete stack of
server + database + discord.

If you're like me and your running a VPN append this to the end of the
`docker-compose.yml`:

```
networks:
  default:
      external:
        name: my-network
```

And run the following with the VPN off:

```
docker network create my-network --subnet 172.24.24.0/24
```

Also specify the network for each service:

```
    networks:
      - default
```
