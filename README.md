# Rust GraphQL Server

## Prerequisites

`Rust` and `Cargo`
- [installation](https://doc.rust-lang.org/cargo/getting-started/installation.html#install-rust-and-cargo) 
- [more info](https://www.rust-lang.org/tools/install)

## Start server

```sh
# Start API Server with debug log level.
RUST_LOG=debug cargo run

# => server will start on port 3030
```

***

## Routes

Once the server is running, the following routes can be used.

### Playground

This route links to the interactive GraphQL playground.

- [Development URL](http://localhost:3030/playground)

### Api Endpoint

This route accepts GraphQL API requests.

- [Development URL](http://localhost:3030/graphql)


TODO: 

- [x] `query` example
- [ ] `mutation` example
- [ ] `subscription` example

### Health Check

This route can be used to confirm the server is alive.

- [Development URL](http://localhost:3030)

**More info**

-  [`src/server/routes.rs`](https://github.com/dankreiger/Rust-GraphQL-API-Server/blob/main/src/server/routes.rs)


