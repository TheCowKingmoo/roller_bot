
use serenity::model::channel::Message;
use serenity::prelude::{Context};
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group
    }
};

use crate::core::*;


#[group]
#[commands(roll)]
pub struct Dice_Roll;

#[command]
fn roll(ctx: &mut Context, msg: &Message) -> CommandResult {
    let message_string  = msg.content.clone();

    let return_tuple = parse::parse_roll_message(message_string);
    let print_string = roller::any_roll(return_tuple.0, return_tuple.1);
    msg.reply(ctx, print_string);
    Ok(())
}
