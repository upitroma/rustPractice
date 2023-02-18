use std::io;
/// this is documentation
// this is a comment
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    // let mut guess = input();
    
    println!("You guessed: {}", input());

}

fn input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input;
}
