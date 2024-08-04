use mongodb::{Client, Database};
use todo_api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ** Connect to MongoDB
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .expect("Failed to connect to MongoDB");

    // ** Replace "mydb" with your database name
    let db: Database = client.database("mydb");

    println!("Server is running at http://127.0.0.1:8080");

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(db.clone()))
            .configure(routes::todo::todo_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
