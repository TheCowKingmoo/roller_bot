
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
	
	if tuple_return.3 != ""  {
        print_string = tuple_return.3;
    }  else  {
        for character in tuple_return.2  {
            if character == 'a'  {
                a_flag = true;
            }
        }

        if a_flag == true  {
            println!("avg");
            print_string = roller::avg_roller(tuple_return.0, tuple_return.1);
        }  else  {
          print_string = roller::print_all_rolls(tuple_return.0, tuple_return.1);
        }
        
    }
	
	
    msg.reply(ctx, print_string);
	
	
	
	
	
    Ok(())
}
