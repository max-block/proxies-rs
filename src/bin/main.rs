use proxies::server::run;

#[actix_web::main]
async fn main() {
    run().await.unwrap();
}
