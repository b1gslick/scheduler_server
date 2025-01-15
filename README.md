# Scheduler server

it is backend for pomidoro/todo app

## Local start

Start dev compose

```bash
docker-compose -f docker-compose-dev.yml up -d
```

provide some env, for example in .env

```bash
PASETO_KEY="RANDOM WORDS WINTER MACINTOSH PC"
PORT=8080
DATABASE_PASSWORD="scheduler"
DATABASE_USER="scheduler"
DATABASE_PORT=5432
DATABASE_DB="schedulerdb"
DATABASE_HOST="localhost"
```

Run server

```bash

make
```
