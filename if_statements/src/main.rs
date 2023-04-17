fn main() {
    println!("===== TERNARY OPERATION =====\n");
    let condition = true;
    let mut number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    println!("===== LOOPING OPERATION =====\n");
    loop {
        println!("Looping at number: {number}");
        if number == 7 {
            break;
        }
        number += 1;
    }
//  Assigning variable with loop
    println!("Assigning variable with loop:");
    let mut counter = 0;
    let mut result = loop {
        counter += 1;

        println!("Counter is: {counter}");

        if counter == 3 {
            break counter * 2
        }
    };
    println!("Result is {result}");

    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if counter == 8 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");

    // Testing Fahrenheit to Celsius

    // FTC first:

    let mut x = 100.0;
    let mut y = FahrenheitToCelsius(true, x);
    println!("{x} in Celsius is {y}");
    x = 30.0;
    y = FahrenheitToCelsius(false, x);
    println!("{x} in Fahrenheit is {y}");
}

/* 
    FahrenheitToCelsius

    Converts Fahrenheit to Celsius or Celsius to Fahrenheit

    * `scale` - Determines what scale it's converting to 
        (true for Fahrenheit to Celsius, false for Celsius to Fahrenheit)
    * `x` - The input number to be converted based on scale variable. 
*/

fn FahrenheitToCelsius(scale: bool, x: f32) {
    if scale == true {
        let y = x - 32.0;
        y * 0.5556;
    }
    else {
        let y = x * 18.0;
        y + 32.0;
    }
}
