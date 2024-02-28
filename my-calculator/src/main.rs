use std::env;

fn main() {
    let cmd_line: Vec<String> = env::args().collect();
    println!("No of elements in arguments is: {}", cmd_line.len());
    println!("The whole list: {:?}", cmd_line);

    let mut result = 0;
    let mut current_operator = "";
    let mut has_read_first_arg = false;

    for arg in &cmd_line {
        if has_read_first_arg {
            // Skip the first argument since it is the executable file name

            if ["+", "-", "*", "/"].contains(&arg.as_str()) {
                // Set the current operator
                current_operator = arg.as_str();
                println!(" Operator: {}", current_operator);
            } else {
                if let Ok(num) = arg.parse::<i32>() {
                    match current_operator {
                        "+" => result += num,
                        "-" => result -= num,
                        "*" => result *= num,
                        "/" => result /= num,
                        _ => {
                            if current_operator == "" {
                                result = num;
                            } else {
                                println!("Invalid operator: {}", current_operator);
                                return;
                            }
                        }
                    }
                    println!(" Number: {}", num);
                } else {
                    println!(" Not a valid number: {}", arg);
                    return;
                }
            }
        }
        has_read_first_arg = true;
    }
    println!("Result is {}", result);
}
