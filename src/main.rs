use std::net::TcpListener;
use ctclsite::start;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();
	
	let listener = TcpListener::bind("127.0.0.1:5000")?;
	start_blog(listener)?.await?;
	Ok(())
}
