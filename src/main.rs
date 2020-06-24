use serenity::client::Client;
use serenity::prelude::{EventHandler};

use serenity::framework::standard::{
    StandardFramework
  };

use crate::commands::*;


pub mod commands;
pub mod core;

use futures::executor::block_on;
use std::env;


impl EventHandler for Handler {}

struct Handler;

async fn async_main()  {
    let _c = construct().await;
}

async fn construct()  {
        // Login with a bot token from the environment
        let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&general::GENERAL_GROUP)
        .group(&roll::DICEROLL_GROUP)
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}




fn main() {
    block_on(async_main());
}