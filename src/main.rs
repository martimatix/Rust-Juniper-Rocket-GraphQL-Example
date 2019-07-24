#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate juniper;

mod model;
mod schema;

use juniper::{EmptyMutation, RootNode};
use model::Database;
use rocket::response::content;
use rocket::State;
use schema::Query;

type Schema = RootNode<'static, Query, EmptyMutation<Database>>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(Database::new())
        .manage(Schema::new(Query, EmptyMutation::<Database>::new()))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
