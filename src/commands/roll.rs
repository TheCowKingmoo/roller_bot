
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
	let mut print_string: String = String::new();
	let mut a_flag = false;

    let tuple_return = parse::parse_roll_message(message_string);

    let num_roll = tuple_return.0;
    let dice_type = tuple_return.1;
    let add_on = tuple_return.2;
    let args = tuple_return.3;
    let err_string = tuple_return.4;

    if err_string != ""  {
        print_string = err_string;
    }  else  {
        for character in args  {
            if character == 'a'  {
                a_flag = true;
            }
        }

        if a_flag == true  {
            print_string = roller::avg_roller(num_roll, dice_type, add_on);
        }  else  {
          print_string = roller::print_all_rolls(num_roll, dice_type, add_on);
        }
        
    }
	
    msg.reply(ctx, print_string);
	
    Ok(())
}
