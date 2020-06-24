use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;

#[group]
#[commands(ping, help)]
pub struct General;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "No help for you!")?;

    Ok(())
}
