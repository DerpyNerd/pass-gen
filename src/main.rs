use std::io;
use rand::Rng;

fn generate(properties: &mut [bool; 4], size: &mut u8) {

    let small_set = String::from("abcdefghijklmnopqrstuvwxyz");
    let cap_set = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let num_set = String::from("0123456789");
    let special_set = String::from("~!@#$%^&*+_`-+");
    let mut password = String::new();

    let mut i: u8 = 0;
    while i < *size {

        let random = rand::thread_rng().gen_range(0..4);
        if properties[random] == true {
            let pass: char = match random {
                0 => small_set.chars().nth(rand::thread_rng().gen_range(0..26)).unwrap(),
                1 => cap_set.chars().nth(rand::thread_rng().gen_range(0..26)).unwrap(),
                2 => num_set.chars().nth(rand::thread_rng().gen_range(0..10)).unwrap(),
                3 => special_set.chars().nth(rand::thread_rng().gen_range(0..14)).unwrap(),
                _ => ' ',
            };
            password.push(pass);
            i += 1;
        }
        else {
            continue;
        }
    }

    println!("Generated password: {}", password);
}

fn main() {

    println!("--------- Password Generator v1.0.0 ---------");
    println!("Input a 4-bit number to choose a setting");
    println!("Uppercase [] - Lowercase [] - Numbers [] - Special Characters []");

    //Read in settings and store appropriately into properties array
    let mut properties: [bool; 4] = [false; 4]; 
    let mut setting = String::new();
    io::stdin()
        .read_line(&mut setting)
        .expect("Failed to read properties");
    for (index, property) in setting.trim().chars().enumerate() {
        properties[index] = match property {
            '0' => false,
            '1' => true,
            _ => {
                println!("Not a valid property");
                break;
            },
        };
    }

    println!("Lowercase: {}   Uppercase: {}   Numbers: {}   Special-Characters: {}",
                properties[0], properties[1], properties[2], properties[3]);

    println!("Enter size of password: ");

    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read size");
    let mut size = size.trim().parse().expect("Not a valid size");

    generate(&mut properties, &mut size);
   
}