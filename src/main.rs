use actix_web::{web, App, HttpServer};
use url_shortener::*;
use dotenv::dotenv;
// use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables
    // Connecting to mongodb and setting it up
    let client_db: mongodb::Client = connect_db().await;
    create_index_url(&client_db).await;
    // build our application
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client_db.clone()))
            .service(add_url)
            .service(get_url)
            .service(get_root)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}