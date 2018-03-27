# Rust Juniper Rocket GraphQL Example

Are you trying to figure out how to use Juniper with Rocket?

I created this repo because at the time of writing (2018-03-28), the [Juniper book](http://juniper.graphql.rs/) section on Rocket is yet to be written and I found the (example in Juniper Rocket)[https://github.com/graphql-rust/juniper/tree/master/juniper_rocket/examples] to not work for me.

I hope this changes in the future.

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
