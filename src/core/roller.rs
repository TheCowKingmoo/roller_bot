use rand::Rng;

const START_PRINT: &str = "Roll Results For ";
const ERR_PRINT: &str = "There was an error";
const D_CHAR: &str = "D";
const NEWLINE: &str = "\n";
const EQUAL: &str = " = ";
const PLUS: &str = " + ";
const HEADER_REPEAT: &str = "Roll ";
const TOTAL_RESULT: &str = "Total = ";
const AVERAGE_RESULT: &str = "Average = ";

pub fn print_all_rolls(number_of_rolls: u32, dice_type: u32, add_on: i32) -> String {

    let mut result_string: String = String::new();
    let mut rng = rand::thread_rng();

    if number_of_rolls == 0 || dice_type == 0  {
        result_string.push_str(ERR_PRINT);
    }  else  {
        result_string.push_str(START_PRINT);
        result_string.push_str(&number_of_rolls.to_string());
        result_string.push_str(D_CHAR);
        result_string.push_str(&dice_type.to_string());
        result_string.push_str(NEWLINE);

        for i in 0..number_of_rolls  {
            result_string.push_str(HEADER_REPEAT);
            result_string.push_str(&((i+1).to_string()));
            result_string.push_str(EQUAL);
            let roll = rng.gen_range(1, dice_type + 1);
            result_string.push_str(&roll.to_string());
            if add_on != 0  {
                result_string.push_str(PLUS);
                result_string.push_str(&add_on.to_string());
                result_string.push_str(EQUAL);
                result_string.push_str(&(roll as i32 + add_on).to_string());
            } 
            result_string.push_str(NEWLINE);

        }
    }
        
    result_string

}  //end print_all_rolls

pub fn avg_roller(number_of_rolls: u32, dice_type: u32, add_on: i32) -> String  {

    let mut result_string: String = String::new();
    let mut rng = rand::thread_rng();

    if number_of_rolls == 0 || dice_type == 0  {
        result_string.push_str(ERR_PRINT);
        return result_string;
    }

    result_string.push_str(START_PRINT);
    result_string.push_str(&number_of_rolls.to_string());
    result_string.push_str(D_CHAR);
    result_string.push_str(&dice_type.to_string());
    result_string.push_str(NEWLINE);

    let mut total = 0;
    let mut i = 0;
    while i < number_of_rolls  {
      total += rng.gen_range(1, dice_type + 1) as i32 + add_on;
      i += 1;
    }
    result_string.push_str(TOTAL_RESULT);
    result_string.push_str(&(total.to_string()));
    result_string.push_str(NEWLINE);
    result_string.push_str(AVERAGE_RESULT);
    result_string.push_str(&((total / i as i32).to_string()));
    result_string.push_str(NEWLINE);

    result_string
    
}
