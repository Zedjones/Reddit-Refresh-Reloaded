use crate::db::Search;
use async_graphql::{EmptyMutation, EmptySubscription};

pub(crate) type Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub(crate) fn schema() -> Schema {
    async_graphql::Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}

pub(crate) struct Query;

#[async_graphql::Object]
impl Query {
    async fn get_searches(&self) -> Vec<Search> {
        todo!()
    }
}
