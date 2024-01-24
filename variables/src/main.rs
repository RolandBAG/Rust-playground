fn main() {
    println!("======== TESTING VARIABLE MUTABILITY =========\n");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("\n\n======== TESTING VARIABLE SHADOWING AND SCOPE =========\n");
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("Inner scope value: {y}");
    }

    println!("Outer scope value: {y}");

    let x = five();
    println!("\n\n====== TESTING FUNCTION RETURN =======\nThe value of x is: {x}");
    let x = plus_one(x);
    println!("Now x has 1 added to it: {x}");
}


fn five() -> i32 {
    5
}

// NOTE: This statement would fail if line 36 was i + 1; because the semicolon
//          makes it a return, not a statement


fn plus_one(x: i32) -> i32 {
    x + 1
}