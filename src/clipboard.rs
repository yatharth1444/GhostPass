use clipboard::{ClipboardContext, ClipboardProvider};
use tokio::time::{sleep, Duration};
use anyhow::Result;

pub async fn shadow_clipboard(text: &str, duration_secs: u64) -> Result<()> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|_| anyhow::anyhow!("Failed to initialize clipboard context"))?;
    ctx.set_contents(text.to_owned())
        .map_err(|_| anyhow::anyhow!("Failed to set clipboard contents"))?;

    // Wait asynchronously and then clear clipboard
    sleep(Duration::from_secs(duration_secs)).await;

    ctx.set_contents("[REDACTED]".to_string())
        .map_err(|_| anyhow::anyhow!("Failed to clear clipboard"))?;
    Ok(())
}
