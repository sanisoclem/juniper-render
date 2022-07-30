use juniper::{graphql_object, EmptyMutation, EmptySubscription};
use warp::Filter;

mod schema;
use schema::*;

#[derive(Clone, Copy, Debug)]
struct Context;
impl juniper::Context for Context {}

#[derive(Clone, Copy, Debug)]
struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }
    async fn ledgers() -> Vec<Ledger> {
        vec![Ledger {
            id: "test".to_owned(),
            name: "test".to_owned(),
            owner: "test".to_owned(),
            created_at: "test".to_owned(),
            accounts_groups: vec![],
            transactions: vec![],
        }]
    }
}

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warp_async");
    env_logger::init();

    let log = warp::log("warp_server");

    let schema = Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    );

    log::info!("Listening on 127.0.0.1:8080");

    let state = warp::any().map(|| Context);
    let graphql_filter = juniper_warp::make_graphql_filter(schema, state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([0, 0, 0, 0], 8000))
    .await
}
