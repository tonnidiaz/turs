use turs::{actix_web::{Responder, Scope, get, web}, log, tokio};

#[get("/")]
async fn index() -> impl Responder{
    "Hello world!"
}
fn router() -> Scope {
    web::scope("")
        .service(index)
    
}
#[tokio::main]
async fn main() {
    log!("Hello, world!");
    turs::Server::new(router, 5001).await.expect("Failed to init server");
}
 