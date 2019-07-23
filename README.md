# Rust Juniper Rocket GraphQL Example

Are you trying to figure out how to use Juniper with Rocket?

Clone this project and follow the instructions below and start playing today! Unlike the [Rocket example](https://github.com/graphql-rust/juniper/blob/master/juniper_rocket/examples/rocket_server.rs) in the Juniper repo, it can be built without relying on types defined in Juniper tests.

## Running the project

Since Rocket uses the latest and greatest features of Rust, you need to use a nightly version of rust. As per the [Rocket documentation](https://rocket.rs/guide/getting-started/), you can do this using

```bash
$ rustup default nightly
```

or per project basis

```bash
$ rustup override set nightly
```

Lastly, you can run the server using

```bash
$ cargo run
```

If all goes well, you will be able to visit http://localhost:8000/ with a graphiql interface.

## Attributions

I found [this example](https://github.com/rofrol/rust-juniper-example) by Roman Frołow to helpful.
