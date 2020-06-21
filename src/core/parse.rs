


pub fn parse_roll_message(message_string: String) -> (u32, u32)  {

    const ASCIIFIX: u32 = 48;

    if message_string.len() < 9  { //prefix + roll + whitespace + 1D1
        return (0, 0);
    }

    let mut number_of_dice = Vec::new();
    let mut dice_type = Vec::new();

    
    let mut dice_num_flag = false;
    let mut d_char_flag = false;

    
    let split = message_string.split_whitespace();

    let mut i = 0;
    for chunk_string in split  {


      if i != 0  {
        for character in chunk_string.chars()  {

          let converted: u32 = u32::from(character) - ASCIIFIX;
  
          //check if char is an integer - no need to check lower as we are using u32
          if converted > 9 {
              //check if we got a number first
              if number_of_dice.len() < 1  {
                  return (0, 0);
    
             // triggers if we get non number input after we get dice type
              }  else if dice_type.len() > 1  {
                println!("error - too much input");
                return (0, 0);
              }
    
              //check if this is our single D character
              if d_char_flag == false && (character == 'd' || character == 'D') {
                  d_char_flag = true;
                  dice_num_flag = true;
              }  else  {
                  println!("error - bad input");
                  return (0, 0);
              }
    
          //this is still the first set of numbers so we add to numDice
          }  else if dice_num_flag == false {
            number_of_dice.push(converted);
        //this is the second set of numbers so we add to dice_type
          }  else  {
              dice_type.push(converted);
          }  
  
        } 

      }

      i += 1;

    }
    return (convert_vector_of_u32_to_single_u32(number_of_dice.as_mut_slice()), convert_vector_of_u32_to_single_u32(dice_type.as_mut_slice()));

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
  