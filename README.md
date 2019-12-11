# Splitterrust-Server

This server should serve all information about the game. This are classes,
abilities, items, spells, etc....

## Features

- [ ] Classes
- [ ] Abilities
- [ ] Items
- [ ] Spells
- [ ] Monsters/NPCs
- [ ] Resources

## Docker
As I am using a VPN on all my machines, docker has some issues with the
netowrk. To prevent this errors it is necessary to pre-create a docker network
when the VPN is down. This network is then specified as external network in
`docker-compose.yml`.

To run `docker-compose` you'll either need to create a network `my-network`
like this:

```
docker network create my-network --subnet 172.24.24.0/24
```

or remove the network definitions from `docker-compose`.
