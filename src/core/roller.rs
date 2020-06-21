use rand::Rng;

pub fn any_roll(number_of_rolls: u32, dice_type: u32) -> String {

    let START_PRINT = "Roll Results For ";
    let ERR_PRINT = "There was an error";

    let mut result_string: String = String::new();
    let mut rng = rand::thread_rng();

    if number_of_rolls == 0 || dice_type == 0  {
        result_string.push_str(ERR_PRINT);
    }  else  {
        result_string.push_str(START_PRINT);
        result_string.push_str(&number_of_rolls.to_string());
        result_string.push_str("D");
        result_string.push_str(&dice_type.to_string());
        result_string.push_str("\n");

        for i in 0..number_of_rolls  {
            result_string.push_str("Roll ");
            result_string.push_str(&((i+1).to_string()));
            result_string.push_str(" = ");
            result_string.push_str(&rng.gen_range(1, dice_type + 1).to_string());
            result_string.push_str("\n");

        }
    }
        
    return result_string;

}  //end any_roll
