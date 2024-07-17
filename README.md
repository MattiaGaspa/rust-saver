# rust-saver

A basic and unsafe (no encryption 🤭) password saver (like BitWarden) written in Rust with actix and Yew with a Postgres database. 

## Run

After building the image with

``` bash
make build
```

Run 

``` bash
make up
```

for the unoptimized version.

For the optimized version (`--release`) run

``` bash
ENV="release" make up
```

## TODO

- Use websockets
- Add options to change ports
- Add remove features
- Login and encryption?