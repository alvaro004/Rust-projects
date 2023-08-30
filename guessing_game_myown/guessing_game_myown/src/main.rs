use rand::{thread_rng, Rng};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    //create a random number
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..101);
    print!("Guess number {}\n", n);

    //ask user the number
    print!("What's your number? \n");

    // storage number
    let mut number = String::new();
    std::io::stdin()
        .read_line(&mut number)
        .expect("Error, please write a number");

    // convert number from String to Integer
    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("please input a number ({})", e);
            return;
        }
    };

    // compare the number with the actual values
    print_type_of(&number);

    //show result

    // make a loop the game
}
