//Random alphabets plus numbers.
use rand::{seq::SliceRandom, Rng};
use std::io::stdin;

const ALPHABETS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn main() {
    let name = get_user_name();
    let length_of_password = get_password_length();
    let val_digit = number_of_digits();
    create_password(length_of_password, name, val_digit)
}

fn create_password(length_of_password: u32, name: String, val_digit: u32) {

    if val_digit > length_of_password {
        println!("The digits should not have a higher number than the password length.");
        return
    }

    let mut vals: Vec<String> = Vec::new();

    //creating a for loop to help achive the password length specified by the username
    for i in 1..=length_of_password {

        let random_letter = ALPHABETS.choose(&mut rand::thread_rng()).unwrap().to_string();
        let to_uppercase = rand::thread_rng().gen::<bool>(); //we are just using this line to randomly get a boolen value.

        if to_uppercase {
            vals.push(random_letter.to_uppercase());
        } else {
            vals.push(random_letter);
        }
    }

    //Note the vals is still a vector, to convert it to a string we have to call the join method on the vector and pass the empty string as a separator.

    let random_password: String = vals.join("");
    println!("{} This is your password: {}", name, random_password);
}

fn get_user_name() -> String {
    println!("What is your name?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    println!("what password length do you want {}", name);
    name
}

fn get_password_length() -> u32 {
    //using built in loop and pattern matching to prevent the program from crashing once the parse fails
    let pass_length = loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u32>() {
            Ok(n) => break n,
            _ => println!("The length is either invalid or must be a number"),
        }
    };
    return pass_length;
}

fn number_of_digits() -> u32 {
    let digit_num = loop {
        println!("Enter the number of digits you want in your password");
        let mut digits = String::new();
        stdin().read_line(&mut digits).unwrap();
        match digits.trim().parse::<u32>() {
            Ok(n) => break n,
            _=> println!("Digits must be a valid number"),
        }
    };
    digit_num
}

// //Random alphabets (lowercase and uppercase) and code refactoring.
// use rand::{seq::SliceRandom, Rng};
// use std::io::stdin;

// const ALPHABETS: [char; 26] = [
//     'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
//     't', 'u', 'v', 'w', 'x', 'y', 'z',
// ];

// fn main() {
//     let name = get_user_name();
//     let length_of_password = get_password_length();
//     create_password(length_of_password, name)
// }

// fn create_password(length_of_password: u32, name: String) {
//     let mut vals: Vec<String> = Vec::new();

//     //creating a for loop to help achive the password length specified by the username
//     for i in 1..=length_of_password {

//         let random_letter = ALPHABETS.choose(&mut rand::thread_rng()).unwrap().to_string();
//         let to_uppercase = rand::thread_rng().gen::<bool>(); //we are just using this line to randomly get a boolen value.

//         if to_uppercase {
//             vals.push(random_letter.to_uppercase());
//         } else {
//             vals.push(random_letter);
//         }
//     }

//     //Note the vals is still a vector, to convert it to a string we have to call the join method on the vector and pass the empty string as a separator.

//     let random_password: String = vals.join("");
//     println!("{} This is your password: {}", name, random_password);
// }

// fn get_user_name() -> String {
//     println!("What is your name?");
//     let mut name = String::new();
//     stdin().read_line(&mut name).unwrap();
//     println!("what password length do you want {}", name);
//     name
// }

// fn get_password_length() -> u32 {
//     //using built in loop and pattern matching to prevent the program from crashing once the parse fails
//     let pass_length = loop {
//         let mut input = String::new();
//         stdin().read_line(&mut input).unwrap();
//         match input.trim().parse::<u32>() {
//             Ok(n) => break n,
//             _ => println!("The length is either invalid or must be a number"),
//         }
//     };
//     return pass_length;
// }


//Version 2
// use rand::{seq::SliceRandom, Rng};
// use std::io::stdin;

// const ALPHABETS: [char; 26] = [
//     'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
//     't', 'u', 'v', 'w', 'x', 'y', 'z',
// ];

// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     stdin().read_line(&mut name).unwrap();
//     println!("what password length do you want {}", name);

//     //using built in loop and pattern matching to prevent the program from crashing once the parse fails
//     let length_of_password = loop {
//         let mut input = String::new();
//         stdin().read_line(&mut input).unwrap();
//         match input.trim().parse::<u32>() {
//             Ok(n) => break n,
//             _ => println!("The length is either invalid or must be a number"),
//         }
//     };

//     create_password(length_of_password, name)
// }

// fn create_password(length_of_password: u32, name: String) {
//     let mut vals: Vec<String> = Vec::new();

//     //creating a for loop to help achive the password length specified by the username
//     for i in 1..=length_of_password {
//         //choose is from the rand lib
//         //The ALPHABETS is from the array of char we defined initially, to_string is how we convert chars to string so we can push it into the vals vector we do this conversion cos in rust chars and strings arev two different datatypes
//         vals.push(
//             ALPHABETS
//                 .choose(&mut rand::thread_rng())
//                 .unwrap()
//                 .to_string(),
//         );
//     }

//     //Note the vals is still a vector, to convert it to a string we have to call the join method on the vector and pass the empty string as a separator.

//     let random_password: String = vals.join("");
//     println!("{} This is your password: {}", name, random_password);
// }

//version 1 of the password generator
// use rand::{seq::SliceRandom, Rng};
// use std::io::stdin;

// const ALPHABETS: [char; 26] = [
//     'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
//     't', 'u', 'v', 'w', 'x', 'y', 'z',
// ];

// fn main() {
//     println!("What is your name?");
//     let mut name = String::new();
//     stdin().read_line(&mut name).unwrap();
//     println!("what password length do you want {}", name);

//     let mut length_of_password = String::new();
//     stdin().read_line(&mut length_of_password).unwrap();

//     //because we want the length of the password to be a number and not a string, we parse the number and crash the program if the parsing fails.

    // let length_of_password = length_of_password
    //     .trim()
    //     .parse::<u32>()
    //     .expect("The length is either invalid or must be a number");

//     create_password(length_of_password, name)
// }

// fn create_password(length_of_password: u32, name: String) {
//     let mut vals: Vec<String> = Vec::new();

//     //creating a for loop to help achive the password length specified by the username
//     for i in 1..=length_of_password {
//         //choose is from the rand lib
//         //The ALPHABETS is from the array of char we defined initially, to_string is how we convert chars to string so we can push it into the vals vector we do this conversion cos in rust chars and strings arev two different datatypes
//         vals.push(
//             ALPHABETS
//                 .choose(&mut rand::thread_rng())
//                 .unwrap()
//                 .to_string(),
//         );
//     }

//     //Note the vals is still a vector, to convert it to a string we have to call the join method on the vector and pass the empty string as a separator.

//     let random_password: String = vals.join("");
//     println!("{} This is your password: {}", name, random_password);

// }
