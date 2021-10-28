fn fizzbag(number: u8){    
    match number {
       number if number%3==0 && number%5==0 => println!("Fizzbag"),
       number if number%3 == 0 => println!("Fizz"),
       number if number%5==0 => println!("Bag"),
        _ => println!("{}", number), 
}

fn main() {
    
    for number in 1..101{
        fizzbag(number)
    }

}