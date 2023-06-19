use std::collections::HashMap;

use seafloor::anyhow;
use seafloor::smol;
use std::process::Command;
use tracing::info;
use warp::Filter;

use crate::logger::setup_logger;
use crate::server::setup_server;
use warp::http::Response;

mod logger;
mod server;
fn main() -> anyhow::Result<()> {
	println!("Hello, world!");
	setup_logger()?;
	let app = setup_server()?;
	info!("Starting server...");
	return app.start();
}
