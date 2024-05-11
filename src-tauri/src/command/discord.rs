use crate::discord;
use crate::prelude::*;

#[tauri::command]
pub async fn connect_discord(app: AppHandle) -> Result<()> {
  debug!(command = "connect_discord");
  discord::connect(app).await;
  Ok(())
}
