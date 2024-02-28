pub fn play(name:String){
    println!("Playing movie {} :movies-app",name);
 }

 pub fn typename(){
    let mut line = String::new();
    println!("Enter your name :");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);
    println!("no of bytes read , {}", b1);
 }