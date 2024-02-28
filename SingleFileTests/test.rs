fn main() {
    let cmd_line = std::env::args();
    println!(
        "No of elements in arguments is :{}",cmd_line.len());

    let mut sum = 0;
    let mut has_read_first_arg = false;

    for arg in cmd_line {
        if has_read_first_arg {
            //skip the first argument since it is the exe file name
            sum += arg.parse::<i32>().unwrap();
        }
        has_read_first_arg = true;
    }
    println!("sum is {}", sum);
}
