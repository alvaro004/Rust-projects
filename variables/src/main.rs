/*
fn main() {
    let mut x = 5;
    let an_string
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


}
*/

/*
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
fn main() {
    let mut x: u32 = 1;
    {
        let mut x = "hola";
    }
    println!("{x}");
}
*/

// fn main() {
//     //declaring integer

//     let number1: i32 = -1202992727;
//     let number2: u8 = 110;

//     println!("{} {}", number1, number2);

//     //declaring float

//     let floatin_num: f64 = -2.221;
//     println!("{}", floatin_num);

//     // declaring boolean

//     let is_boolean: bool = true;
//     println!("{}", is_boolean);

//     //touples

//     let _tup1: (i32, f64, u8) = (500, 6.4, 1);
//     let _tup2: (&str, &str, u8, f64) = ("Alvaro", "Llano", 23, 69.6);
//     println!(
//         "Your name is {} {}, and your age is {}, your weight is {}",
//         _tup2.0, _tup2.1, _tup2.2, _tup2.3
//     );

//     //Arrays

//     let _array1: [u32; 4] = [12, 14, 15, 16];

//     for n in _array1 {
//         println!("Element {} ", n)
//     }

//     //read the user input and and store it in a variable then print it

//     let mut _user_input = String::new();
//     println!("Please enter your name: ");
//     std::io::stdin()
//         .read_line(&mut _user_input)
//         .expect("Failed to read line");
//     println!("Your name is {}", _user_input);
// }

// fn main() {
//     //write a function that takes a number and look inside a predifine array and notify to the user if the number it is or not

//     let _array_numbers: [i32; 5] = [12, 9, 4, 2, 15];

//     loop {
//         println!("Can you guess the number inside the array?\nPlease enter a number: ");
//         let mut _user_input_number = String::new();

//         std::io::stdin()
//             .read_line(&mut _user_input_number)
//             .expect("Failed to read line");

//         let _user_input_number: i32 = match _user_input_number.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("\nPlease enter a valid number\n");
//                 continue;
//             }
//         };

//         for (index, &_each_element) in _array_numbers.iter().enumerate() {
//             if _each_element == _user_input_number {
//                 println!(
//                     "your number {} it is inside the array in the position {}",
//                     _user_input_number, index
//                 );
//                 break;
//             } else {
//                 println!(
//                     "your number {} it is not inside the array",
//                     _user_input_number
//                 );
//             }
//         }
//         println!();
//         break;
//     }
// }

fn search_number() {
    let numbers = [10, 20, 30, 40, 50];

    loop {
        // Read input from the user
        println!("Enter a number (or 'q' to quit):");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        // Check if user wants to quit
        if input.trim() == "q" {
            println!("Exiting program...");
            break;
        }

        // Parse user input into an integer
        let search_number: i32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        // Search for the number in the array
        let result = numbers.iter().position(|&x| x == search_number);

        match result {
            Some(index) => println!("Number found at index: {}", index),
            None => println!("Number not found"),
        }
    }
}

fn main() {
    search_number();
}
