use server::Schema;

#[tokio::main]
async fn main() {
    let schema = async_graphql::Schema::from(Schema::new("server".to_string()));
    println!("{}", schema.sdl());
}
