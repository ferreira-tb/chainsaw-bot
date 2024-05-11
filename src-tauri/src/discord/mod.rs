mod command;

mod prelude {
  pub(super) use super::Context;
}

use crate::prelude::*;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::GatewayIntents;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};

type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
  #[allow(dead_code)]
  pub app: AppHandle,
}

impl Data {
  pub fn new(app: AppHandle) -> Self {
    Self { app }
  }
}

pub async fn connect(app: AppHandle) {
  thread::spawn(move || {
    block_on(async move {
      let prefix_options = PrefixFrameworkOptions {
        prefix: Some("!".into()),
        mention_as_prefix: true,
        case_insensitive_commands: true,
        ..Default::default()
      };

      let options = FrameworkOptions {
        prefix_options,
        commands: vec![command::ping()],
        ..Default::default()
      };

      let _: Result<()> = try {
        let data = Data::new(app);
        let builder = Framework::builder()
          .options(options)
          .setup(move |ctx, _, framework| {
            Box::pin(async move {
              let commands = &framework.options().commands;
              poise::builtins::register_globally(ctx, commands).await?;
              Ok(data)
            })
          });

        let intents = GatewayIntents::non_privileged() | GatewayIntents::privileged();
        let mut client = serenity::ClientBuilder::new("token", intents)
          .framework(builder.build())
          .await?;

        client.start().await?;
      };
    });
  });
}
