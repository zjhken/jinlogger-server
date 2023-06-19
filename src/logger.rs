use tracing::Level;

use anyhow_ext::Context;
use seafloor::anyhow::Result;

pub fn setup_logger() -> Result<()> {
	let subscriber = tracing_subscriber::FmtSubscriber::builder()
		// all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
		// will be written to stdout.
		.with_max_level(Level::INFO)
		// builds the subscriber.
		.finish();

	tracing::subscriber::set_global_default(subscriber)
		.context("setting default subscriber failed")?;
	return Ok(());
}
