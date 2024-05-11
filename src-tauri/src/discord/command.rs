use super::prelude::*;
use crate::prelude::*;

#[poise::command(prefix_command)]
pub(super) async fn ping(ctx: Context<'_>) -> Result<()> {
  ctx.reply("pong!").await?;
  Ok(())
}
