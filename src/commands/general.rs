
use serenity::model::channel::Message;
use serenity::prelude::{Context};
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group
    }
};


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