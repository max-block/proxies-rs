#[actix_web::main]
async fn main() -> std::io::Result<()> {
    proxies::run().await
}
