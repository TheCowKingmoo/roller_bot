
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


const MAX_PRINT_LINE_NUM: u32 = 128;
const MAX_PRINT_LINE_MSG: &str = "Result exceeded max print number. Averaging result instead.\n";

#[group]
#[commands(roll)]
pub struct DiceRoll;

#[command]
fn roll(ctx: &mut Context, msg: &Message) -> CommandResult {
    let message_string  = msg.content.clone();
	let mut print_string: String = String::new();
	let mut a_flag = false;

    //let mut tuple_return: (u32, u32, i32, Vec<char>) = (0, 0, 0, Vec::new());

    print_string.push_str("");

    let r = parse::parse_roll_message(message_string);
    if r.is_err()  {
        print_string = format!("{:?}", r.err().unwrap());
    }  else  {
        let tuple_return = r.ok().unwrap();
        
        let num_roll = tuple_return.0;
        let dice_type = tuple_return.1;
        let add_on = tuple_return.2;
        let args = tuple_return.3;


        println!("{}", num_roll);
    
        for character in args  {
            if character == 'a'  {
                a_flag = true;
            }
        }
    
        if a_flag == true {
            print_string = roller::avg_roller(num_roll, dice_type, add_on);
        }  else if num_roll > MAX_PRINT_LINE_NUM   {
            print_string =  format!("{}{}", MAX_PRINT_LINE_MSG.to_string(), roller::avg_roller(num_roll, dice_type, add_on));
        } else  {
            print_string = roller::print_all_rolls(num_roll, dice_type, add_on);
        }    


    }


    

    println!("{}", print_string);
	
    msg.reply(ctx, print_string)?;
	
    Ok(())
}
