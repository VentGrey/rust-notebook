use std::io;
use std::f64::consts::PI;
use rand::Rng;

fn main() {
    //-- MENU
    println!("Please select an exercise to execute:");
    println!("1 - Basic operations");
    println!("2 - Circle");
    println!("3 - Currency exchange");

    //-- Menu processing
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    let option: u32 = input.trim().parse().unwrap();

    match option {
        1 => basic(),
        2 => circle(),
        3 => exchange(),
        4 => measurements(),
        _ => {
            println!("Option not available! Returning to main...");
            main();
        },
    }
}

fn basic() {
    println!("Please input a first number");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Could not read from stdin");

    let num1: f64 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),
    };

    println!("Please input another number");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Could not read from stdin");

    let num2: f64 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),
    };

    println!("The sum of the numbers equals to: {:.2}", num1 + num2);
    println!("The diff of the numbers equals to: {:.2}", num1 - num2);
    println!("The product of the numbers equals to: {:.2}", num1 * num2);
    println!("The division of the numbers equals to: {:.2}", num1 / num2);
}

fn circle() {
    println!("Please input your circle's radius");
    let mut rad = String::new();
    io::stdin()
        .read_line(&mut rad)
        .expect("Could not read from stdin");

    let rad: f64 = match rad.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),
    };

    println!("The area of the circle is equal to: {:.2}", rad * PI * PI);
    println!("The perimeter of the circle is equal to: {:.2}", (2.0 * rad) * PI);
}

fn exchange() {
    // Generate the random "dollar" value
    let mut rng = rand::thread_rng();
    let d_value = rng.gen_range(17.0, 20.0);

    println!("Please input the amount of USD to convert");
    let mut mxn = String::new();
    io::stdin()
        .read_line(&mut mxn)
        .expect("Failed to read from stdin");

    let mxn: f64 = match mxn.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),
    };

    println!("You possess {:.2} mxn", mxn * d_value);
}

fn measurements() {
    const HECT_VAL:f64 = 0.404686;

    let mut acres = String::new();
    println!("Input the number of acres you wish to convert");
    io::stdin()
        .read_line(&mut acres)
        .expect("Failed to read from stdin");

    let acres: f64 = match acres.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),
    };

    println!("You possess {:.2} hectares", acres * HECT_VAL);
}
