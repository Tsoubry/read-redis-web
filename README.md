# Read Redis Web

A simple http server in front of a Redis database for quickly getting values from keys. Only supports reading data (Redis GET command) and responds with raw bytes. There's an option to validate the json first before responding. It can also decompress the data from Redis if it's zipped with gzip.

For a more complete and user friendly Redis web front server, check out [Webdis](https://webd.is).


## Quickstart

With Docker: 
```sh
docker-compose up -d
```

Building with cargo:
```sh
cargo build --release
./target/release/read-redis-web
```

For more info, check the --help function.
