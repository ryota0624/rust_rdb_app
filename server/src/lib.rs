use async_graphql::*;

#[derive(SimpleObject)]
pub struct User {
    name: String,
}

pub struct Query;

#[Object]
impl Query {
    async fn user(&self, username: String) -> Result<Option<User>> {
        Ok(Some(User { name: username }))
    }
}
pub struct Mutation;

#[Object]
impl Mutation {
    async fn register_user(&self, username: String) -> User {
        User { name: username }
    }
}

pub struct Schema(async_graphql::Schema<Query, Mutation, EmptySubscription>);

impl Schema {
    pub fn new(_a: String) -> Self {
        Schema(async_graphql::Schema::build(Query, Mutation, EmptySubscription).finish())
    }

    pub async fn execute(&self, query: String) -> Response {
        self.0.execute(query).await
    }
}

impl From<Schema> for async_graphql::Schema<Query, Mutation, EmptySubscription> {
    fn from(schema: Schema) -> Self {
        schema.0
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
