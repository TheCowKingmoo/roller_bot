const ARG_CHAR: char = '-';
const ADD_CHAR: char = '+';
const ARG_AVERAGE_CHAR: char = 'a';
const DICE_CHAR_LOWER: char = 'd';
const DICE_CHAR_UPPER: char = 'D';

const ASCII_DECIMAL_SHIFT: u32 = 48;
const ASCII_ALPHA_SHIFT_IDX: u32 = 97;

use crate::core::*;

#[derive(Debug)]
pub enum ParseError {
  NoDecimalBeforeD,
  NonDecimalBeforeD,
  NonDecimalAfterD,
  NonDigit,
  SameArg,
  NoMatchArg,
  InputTooSmall,
  SplitStringTooSmall,
  FirstCharAlphabet,
  AlreadyRolledDice,
  NoDFound,
  NoDecimalAfterD,
  CatchAll,
  AddOrSubBeforeDice,
}
pub type DiceParseResult = Result<(u32, u32), ParseError>;

pub type ModifyParseResult = Result<i32, ParseError>;

pub type ArgParseResult = Result<Vec<char>, ParseError>;

pub type ParseResult = Result<(Vec<dice::Dice>, Vec<char>), ParseError>;

pub fn parse_roll_message(message_string: String) -> ParseResult {
  let mut arg_char_vector: Vec<char> = vec!['.'; 26]; // Char Vector that will hold arg flags
  let mut dice_vector: Vec<dice::Dice> = Vec::new();

  //Check if the given string is too small to actually be a command
  if message_string.len() < 9 {
    // value = min length for prefix + roll + whitespace + 1D1 - usize made making this a const a pain
    // eg: ~roll 1D1 = smallest command possible
    return Err(ParseError::InputTooSmall);
  }

  let mut current_dice: dice::Dice = dice::Dice {
    number_rolls: 0,
    dice_type: 0,
    modifier: 0,
  };

  //Flags used to tell if we have already parsed parts already
  let mut dice_flag = false;

  // splits the input string by whitespace
  let split = message_string.split_whitespace();

  let mut i = 0; //used to tell what string idx we are at

  for chunk_string in split {
    // skip the first loop - always equal to ~roll
    if i == 0 {
      i += 1;
      continue;
    }

    // now we break down the string into characters
    let characters: Vec<char> = chunk_string.chars().collect();

    if characters.len() < 2 {
      println!("SplitStringTooSmall");
      return Err(ParseError::SplitStringTooSmall);
    }

    let current_char = characters[0];
    let next_char = characters[1];

    if current_char.is_alphabetic() {
      println!("FirstCharAlphabet");
      return Err(ParseError::FirstCharAlphabet);
    }

    if current_char.is_ascii_digit() {
      let r_tuple = parse_dice(chunk_string)?;

      if dice_flag {
        dice_vector.push(current_dice);
        current_dice = dice::Dice {
          number_rolls: 0,
          dice_type: 0,
          modifier: 0,
        };
      }
      dice_flag = true;
      current_dice.number_rolls = r_tuple.0;
      current_dice.dice_type = r_tuple.1;
    } else if current_char == ARG_CHAR {
      if next_char.is_ascii_digit() {
        if !dice_flag {
          return Err(ParseError::AddOrSubBeforeDice);
        }
        current_dice.modifier += modify_operation(chunk_string)?;
      } else {
        let temp_vec = break_up_arg(chunk_string)?;
        for character in temp_vec {
          let s: usize = (character as u32 - ASCII_ALPHA_SHIFT_IDX) as usize;
          arg_char_vector[s] = character;
        }
      }
    } else if current_char == ADD_CHAR {
      if !dice_flag {
        return Err(ParseError::AddOrSubBeforeDice);
      }
      current_dice.modifier += modify_operation(chunk_string)?;
    } else {
      println!("CatchAll");
      return Err(ParseError::CatchAll);
    }
  }

  dice_vector.push(current_dice);
  Ok((dice_vector, arg_char_vector))
}

// will parse a string in a format like 1D20, 100D100, num...numDnum...num
// the numbers before D are used as number of dice to roll, after D represents the type of dice
fn parse_dice(dice_string: &str) -> DiceParseResult {
  let mut number_of_dice_vec = Vec::new();
  let mut dice_type_vec = Vec::new();
  let mut d_flag = false; //lets us know what half od the string we are on
                          // split string into a vec of chars
  let characters: Vec<char> = dice_string.chars().collect();

  for character in characters {
    if !d_flag {
      if character.is_ascii_digit() {
        // push the found char in its u32 decimal form
        number_of_dice_vec.push(u32::from(character) - ASCII_DECIMAL_SHIFT);
      } else if character == DICE_CHAR_LOWER || character == DICE_CHAR_UPPER {
        if number_of_dice_vec.is_empty() {
          println!("NoDecimalBeforeD");
          return Err(ParseError::NoDecimalBeforeD);
        }
        d_flag = true;
      } else {
        //got a non number before D
        println!("NonDecimalBeforeD");
        return Err(ParseError::NonDecimalBeforeD);
      }
    } else if character.is_ascii_digit() {
      // push the found char in its u32 decimal form
      dice_type_vec.push(u32::from(character) - ASCII_DECIMAL_SHIFT);
    } else {
      println!("NonDecimalAfterD");
      return Err(ParseError::NonDecimalAfterD);
    }
  }

  if !d_flag {
    return Err(ParseError::NoDFound);
  }

  if dice_type_vec.is_empty() {
    return Err(ParseError::NoDecimalAfterD);
  }

  // convert collected numbers into a singular u32
  let number_of_dice: u32 = convert_vector_of_u32_to_single_u32(number_of_dice_vec.as_mut_slice());
  let dice_type: u32 = convert_vector_of_u32_to_single_u32(dice_type_vec.as_mut_slice());
  let return_tuple = (number_of_dice, dice_type);
  Ok(return_tuple)
}

fn break_up_arg(arg_string: &str) -> ArgParseResult {
  let mut return_vector = Vec::new();
  let mut a_flag = false;

  let mut i = 0;
  for character in arg_string.chars() {
    if i == 0 {
      i += 1;
      continue;
    }
    let lower_character = character.to_ascii_lowercase();
    if lower_character == ARG_AVERAGE_CHAR {
      if !a_flag {
        a_flag = true;
        return_vector.push(character);
      } else {
        println!("SameArg");
        return Err(ParseError::SameArg);
      }
    } else {
      println!("NoMatchArg");
      return Err(ParseError::NoMatchArg);
    }
    i += 1;
  }
  Ok(return_vector)
}

fn modify_operation(input_string: &str) -> ModifyParseResult {
  let mut numbers = Vec::new();
  let mut sign: i32 = 1;
  let mut i = 0;
  for character in input_string.chars() {
    if i == 0 {
      if character == ARG_CHAR {
        sign = -1;
      }
      i += 1;
      continue;
    } else if !character.is_ascii_digit() {
      println!("NonDigit");
      return Err(ParseError::NonDigit);
    }
    numbers.push(u32::from(character) - ASCII_DECIMAL_SHIFT);
  }

  let number = sign * convert_vector_of_u32_to_single_u32(numbers.as_mut_slice()) as i32;
  Ok(number)
}

fn convert_vector_of_u32_to_single_u32(input_vector: &mut [u32]) -> u32 {
  let mut final_number: u32 = 0;
  let mut i = 1;

  let l: u32 = input_vector.len() as u32;

  for input in input_vector {
    final_number += *input * 10u32.pow(l - i);
    i += 1;
  }
  final_number
}

#[test]
fn test_vector_of_u32_to_single_u32() {
  let mut input = Vec::new();
  input.push(1);
  input.push(0);
  input.push(0);
  input.push(1);
  let actual = 1001;
  let return_value = convert_vector_of_u32_to_single_u32(input.as_mut_slice());
  assert_eq!(actual, return_value);
}

#[test]
fn test_arg_average() {
  let input = "-a";
  let expected = vec!['a'];
  let return_value = break_up_arg(input);
  assert_eq!(expected, return_value.ok().unwrap());
}

#[test]
fn test_arg_dne() {
  let input = "-.";
  let expected = ParseError::NoMatchArg;
  let return_value = break_up_arg(input);
  assert_eq!(
    format!("{:?}", expected),
    format!("{:?}", return_value.err().unwrap())
  );
}

#[test]
fn test_mod_add() {
  let input = "+100";
  let expected: i32 = 100;
  let return_value = modify_operation(input);
  assert_eq!(expected, return_value.ok().unwrap());
}

#[test]
fn test_mod_sub() {
  let input = "-100";
  let expected: i32 = -100;
  let return_value = modify_operation(input);
  assert_eq!(expected, return_value.ok().unwrap());
}

#[test]
fn test_mod_non_digit() {
  let input = "+1a1";
  let expected = ParseError::NonDigit;
  let return_value = modify_operation(input);
  assert_eq!(
    format!("{:?}", expected),
    format!("{:?}", return_value.err().unwrap())
  );
}

#[test]
fn test_simple_parse_dice() {
  let input = "1D20";
  let expected: (u32, u32) = (1, 20);
  let return_value = parse_dice(input);
  assert_eq!(expected, return_value.ok().unwrap());
}

#[test]
fn test_multi_parse_dice() {
  let input = "70D100";
  let expected: (u32, u32) = (70, 100);
  let return_value = parse_dice(input);
  assert_eq!(expected, return_value.ok().unwrap());
}

#[test]
fn test_parse_dice_no_d() {
  let input = "100";
  let expected = ParseError::NoDFound;
  let return_value = parse_dice(input);
  assert_eq!(
    format!("{:?}", expected),
    format!("{:?}", return_value.err().unwrap())
  );
}

#[test]
fn test_parse_dice_no_digit_before_d() {
  let input = "D100";
  let expected = ParseError::NoDecimalBeforeD;
  let return_value = parse_dice(input);
  assert_eq!(
    format!("{:?}", expected),
    format!("{:?}", return_value.err().unwrap())
  );
}

#[test]
fn test_parse_dice_no_digit_after_d() {
  let input = "100D";
  let expected = ParseError::NoDecimalAfterD;
  let return_value = parse_dice(input);
  assert_eq!(
    format!("{:?}", expected),
    format!("{:?}", return_value.err().unwrap())
  );
}

#[test]
fn test_parse_roll_message_too_small() {
  let input = "~roll 1D".to_owned();
  let expected = ParseError::InputTooSmall;
  let return_value = parse_roll_message(input);
  assert_eq!(
    format!("{:?}", expected),
    format!("{:?}", return_value.err().unwrap())
  );
}

#[test]
fn test_parse_roll_message_simple() {
  let input = "~roll 1D20".to_owned();
  let c_dice = dice::Dice {
    number_rolls: 1,
    dice_type: 20,
    modifier: 0,
  };
  let return_value = parse_roll_message(input);
  let r_d = return_value.ok().unwrap().0[0];
  assert_eq!(c_dice.number_rolls, r_d.number_rolls);
  assert_eq!(c_dice.dice_type, r_d.dice_type);
}

#[test]
fn test_parse_roll_message_modify() {
  let input = "~roll 1D20 +7 -100".to_owned();
  let c_dice = dice::Dice {
    number_rolls: 1,
    dice_type: 20,
    modifier: -93,
  };
  let return_value = parse_roll_message(input);
  let r_d = return_value.ok().unwrap().0[0];
  assert_eq!(c_dice.number_rolls, r_d.number_rolls);
  assert_eq!(c_dice.dice_type, r_d.dice_type);
  assert_eq!(c_dice.modifier, r_d.modifier);
}

#[test]
fn test_parse_roll_message_arg() {
  let input = "~roll 1D20 -a".to_owned();
  let mut vec = vec!['.'; 26];
  vec[0] = 'a';
  let return_value = parse_roll_message(input);
  assert_eq!(vec[0], return_value.ok().unwrap().1[0]);
}
