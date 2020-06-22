

const ARG_CHAR: char = '-';
const ADD_CHAR: char = '+';
const ARG_AVERAGE_CHAR: char = 'a';
const DICE_CHAR_LOWER: char = 'd';
const DICE_CHAR_UPPER: char = 'D';
const EMPTY_STRING: &str = "";
const E_TOO_SMALL_INPUT: &str = "input is too small";
const E_MORE_THAN_ONE_ARG_CHAR: &str = "more than one arg character was detected";
const E_NON_INT_BEFORE_D: &str = "got a non-integer before D";
const E_NON_INT_AFTER_D: &str = "got non-integer after D";
const E_MINUS_IN_WRONG_SPOT: &str = "detected minus in wrong location";
const E_SIGN_NO_ROOM: &str = "detected operation but nothing behind it";
const E_MORE_THAN_ONE_ATTEMPT_TO_ADD_MINUS: &str = "detected more than one attempt to add and or minus";
const E_DEFAULT: &str = "default error catcher";
const E_ADD_SUB_NON_DIGIT: &str = "error - attempting to add/sub a non digit";

const E_MULTIPLE_SAME_ARG: &str = "Error - more than one of the same arg detected";
const E_NO_MATCH_ARG: &str = "Error - no matching arguments";
const ASCII_FIX: u32 = 48;


pub fn parse_roll_message(message_string: String) -> (u32, u32, i32, Vec<char>, String)  {

    let mut arg_char_vector = Vec::new();             // Char Vector that will hold arg flags 
    let mut arg_str_return: String = String::new();   // String that will be filled with error message if error happens

    //Check if the given string is too small to actually be a command
    if message_string.len() < 9  {                    // value = min length for prefix + roll + whitespace + 1D1 - usize made making this a const a pain
                                                      // eg: ~roll 1D1 = smallest command possible
      arg_str_return.push_str(E_TOO_SMALL_INPUT);     
      return (0, 0, 0, arg_char_vector, arg_str_return);
    }

    let mut number_of_dice = Vec::new();              // will have the input string characters representing number of dice
    let mut dice_type = Vec::new();                   // will have the input string characters representing dice type
    let mut extra: i32 = 0;
    

    //Flags used to tell if we have already parsed parts already
    let mut dice_num_flag = false;
    let mut d_char_flag = false;
    let mut operator_flag = false;
    let mut add_minus_flag = false;

    // splits the input string by whitespace
    let split = message_string.split_whitespace();

    let mut i = 0;  //used to tell what string idx we are at

    for chunk_string in split  {

      // skip the first loop - always equal to ~roll
      if i == 0  {
        i += 1;
        continue;
      }

      // now we break down the string into characters
      let characters: Vec<char> = chunk_string.chars().collect();
      for j in 0..characters.len()  {
        let character: char = characters[j];

        // Check for args / minus
        if character == ARG_CHAR  {
          // cannot have - in any location but the first
          if j != 0  {
            arg_str_return.push_str(E_MINUS_IN_WRONG_SPOT);
            return(0, 0, 0, arg_char_vector, arg_str_return);
          }
          // check if anything exists past
          if j >= characters.len()+1  {
            arg_str_return.push_str(E_SIGN_NO_ROOM);
            return(0, 0, 0, arg_char_vector, arg_str_return);
            // check next char - if digit then we know this is a minus
          }  else if  characters[j+1].is_ascii_digit()  {
            // we have already tried to minus
            if add_minus_flag == true  {
              arg_str_return.push_str(E_MORE_THAN_ONE_ATTEMPT_TO_ADD_MINUS);
              return (0, 0, 0, arg_char_vector, arg_str_return);
            }
            add_minus_flag = true;
            let sub_tuple = add_or_subtract(chunk_string);
            
            extra = sub_tuple.0 as i32;
            extra = extra * -1;

            arg_str_return = sub_tuple.1;

            if arg_str_return != EMPTY_STRING  {
              return (0, 0, 0, arg_char_vector, arg_str_return);
            }

            break; //we break as the add_or_sub function parse that whole string

          }  else if operator_flag == true  {
            arg_str_return.push_str(E_MORE_THAN_ONE_ARG_CHAR);
            return (0, 0, 0, arg_char_vector, arg_str_return);
          }  else  {
            operator_flag = true;
            let arg_return_tuple = break_up_arg(chunk_string);
            arg_char_vector = arg_return_tuple.0;
            arg_str_return = arg_return_tuple.1;
            if arg_str_return != EMPTY_STRING  {
              return (0, 0, 0, arg_char_vector, arg_str_return);
            }
            break;
          }

        }  else if character == ADD_CHAR   {

          if j >= characters.len()+1  {
            arg_str_return.push_str(E_SIGN_NO_ROOM);
            return(0, 0, 0, arg_char_vector, arg_str_return);
            // check next char - if digit then we know this is a minus
          }

          if add_minus_flag == true   {
            arg_str_return.push_str(E_MORE_THAN_ONE_ATTEMPT_TO_ADD_MINUS);
            return (0, 0, 0, arg_char_vector, arg_str_return);
          }
          
          add_minus_flag = true;
          let sub_tuple = add_or_subtract(chunk_string);
          
          extra = sub_tuple.0 as i32;

          arg_str_return = sub_tuple.1;

          if arg_str_return != EMPTY_STRING  {
            return (0, 0, 0, arg_char_vector, arg_str_return);
          }

          break; //we break as the add_or_sub function parse that whole string

        } else if character.is_alphabetic()  {

          if number_of_dice.len() < 1  {
            arg_str_return.push_str(E_NON_INT_BEFORE_D);
            return (0, 0, 0, arg_char_vector, arg_str_return);
          }

          if dice_type.len() > 1  {
            arg_str_return.push_str(E_NON_INT_AFTER_D);
            return (0, 0, 0, arg_char_vector, arg_str_return);
          }

          //check if this is our single D character
          if d_char_flag == false && (character == DICE_CHAR_LOWER || character == DICE_CHAR_UPPER) {
            d_char_flag = true;
            dice_num_flag = true;
          } 

        } else if character.is_ascii_digit()  {

          if dice_num_flag == false {

            number_of_dice.push(u32::from(character) - ASCII_FIX);

          //this is the second set of numbers so we add to dice_type
          }  else  {
            
              dice_type.push(u32::from(character) - ASCII_FIX);
          }
        }  else  {
          arg_str_return.push_str(E_DEFAULT);
          return (0, 0, 0, arg_char_vector, arg_str_return);
        }
      }  //end for inner loop  
    }  //end outer for loop
    return (convert_vector_of_u32_to_single_u32(number_of_dice.as_mut_slice()), convert_vector_of_u32_to_single_u32(dice_type.as_mut_slice()), extra, arg_char_vector, arg_str_return);
}  //end parse roll

fn break_up_arg(arg_string: &str) -> (Vec<char>, String)  {
  let mut i = 0;
  let mut return_vector = Vec::new();
  let mut a_flag = false;
  let mut return_str = EMPTY_STRING;
  let mut return_string = String::new();

  for character in arg_string.chars()  {
    if i == 0  {
      i += 1;
      continue;
    }

    let lower_character = character.to_ascii_lowercase();

    match lower_character  {
      // a = avg
      ARG_AVERAGE_CHAR => if a_flag == false  {
        a_flag = true;
        return_vector.push(character);
      }  else  {
        return_str = E_MULTIPLE_SAME_ARG;
        break;
      },
      // default catch all
      _ => return_str = E_NO_MATCH_ARG
    }
    
    i+=1;
  }
  return_string.push_str(return_str);
  return (return_vector, return_string);
}


fn add_or_subtract(input_string: &str) -> (u32, String)  {
  let mut return_string = String::new();
  let mut numbers = Vec::new();
  let mut i = 0;
  for character in input_string.chars()  {
    if i == 0  {
      i+=1;
      continue;
    }
    if character.is_ascii_digit() == false  {
      return_string.push_str(E_ADD_SUB_NON_DIGIT);
      return (0,return_string);
    }
    numbers.push(u32::from(character) - ASCII_FIX);
  }

  let number = convert_vector_of_u32_to_single_u32(numbers.as_mut_slice());
  return (number, return_string);
}


fn convert_vector_of_u32_to_single_u32(input_vector: &mut [u32]) -> u32  {
    let mut return_value: u32 = 0;
    let mut i = 1;
  
    let l: u32 =  input_vector.len() as u32;
  
    for input in input_vector  {
      return_value += *input * 10u32.pow(l - i);
      i += 1;
    }
    return return_value;
  }
  