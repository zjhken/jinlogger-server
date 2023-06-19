use seafloor::{
	anyhow::Result,
	application::{self, App, HttpResult},
	smol::{future::FutureExt, process::Command},
};
use serde::Deserialize;
use tracing::info;

pub fn setup_server() -> Result<App> {
	let mut app = App::new();

	app.setFunc("/", |ctx| async move { html(ctx).await }.boxed());

	app.setFunc("/api/dsl", |ctx| async move { dsl(ctx).await }.boxed());

	app.listenAddress(([0, 0, 0, 0], 7777));
	return Ok(app);
}

async fn html(ctx: &mut application::Context) -> HttpResult {
	ctx.response.set_body(include_str!("ui.html"));
	Ok(())
}

async fn dsl(ctx: &mut application::Context) -> HttpResult {
	#[derive(Deserialize)]
	struct ProcessLogInput {
		file: String,
		dsl: String,
	}
	info!("enter");
	let input = ctx.request.query::<ProcessLogInput>().unwrap();
	let out = Command::new("cat")
		.arg(input.file)
		.arg("|")
		.arg(input.dsl)
		.output()
		.await?;
	ctx.response.set_body(out.stdout);
	return Ok(());
}
