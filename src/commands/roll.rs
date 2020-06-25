use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::channel::Message;
use serenity::prelude::Context;

use crate::core::*;

const MAX_PRINT_LINE_NUM: u32 = 128;
const MAX_PRINT_LINE_MSG: &str = "Result exceeded max print number. Averaging result instead.\n";

#[group]
#[commands(roll)]
pub struct DiceRoll;

#[command]
fn roll(ctx: &mut Context, msg: &Message) -> CommandResult {
    let message_string = msg.content.clone();
    let mut print_string: String = String::new();
    let mut a_flag = false;

    //let mut tuple_return: (u32, u32, i32, Vec<char>) = (0, 0, 0, Vec::new());

    print_string.push_str("");

    let r = parse::parse_roll_message(message_string);
    if r.is_err() {
        print_string = format!("{:?}", r.err().unwrap());
    } else {
        let tuple_return = r.ok().unwrap();
        let dice_vector = tuple_return.0;
        let arg_vector = tuple_return.1;

        for character in arg_vector {
            if character == 'a' {
                a_flag = true;
            }
        }

        println!("{}", dice_vector.len());

        for dice in dice_vector {
            if a_flag {
                print_string.push_str(&roller::avg_roller(
                    dice.number_rolls,
                    dice.dice_type,
                    dice.modifier,
                ));
            } else if dice.number_rolls > MAX_PRINT_LINE_NUM {
                print_string.push_str(&format!(
                    "{}{}",
                    MAX_PRINT_LINE_MSG.to_string(),
                    roller::avg_roller(dice.number_rolls, dice.dice_type, dice.modifier)
                ));
            } else {
                print_string.push_str(&roller::print_all_rolls(
                    dice.number_rolls,
                    dice.dice_type,
                    dice.modifier,
                ));
            }
        }
    }

    println!("{}", print_string);
    msg.reply(ctx, print_string)?;
    Ok(())
}
