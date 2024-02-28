fn main() {
    let table = [1, 2, 3, 4, 5];


    clockwise(&table);
    anticlockwise(&table);
}

fn anticlockwise(table:&[usize]){

    let mut result = Vec::new();
    for index in 0..table.len() {
        result.push(table[table[index]%table.len()])
    }
    println!("Result = {:?}", result);

}
fn clockwise(table:&[usize]) {

    let mut result = Vec::new();
    for index in 0..table.len() {
        let new_index = (index + table.len() - 1) % table.len();
        result.push(table[new_index]);
    }
    println!("Result = {:?}", result);
}
