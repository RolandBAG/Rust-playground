fn main() {
    let x = 5;
    let y = x;
    let mut s = String::from("hello");

/* Code that doesnt work - demonstrates borrowing */
//    change(&s);

// This will work on the other hand because it's assigning a mutable reference. 
    change(&mut s);
    println!("{}", s);

/*  This code won't compile because you can't have two rerferences to the same variable at the same time
    let r1 = &mut s;
    let r2 = &mut s;

    However you can make two references if one is in scope: IE
*/
    {
        let r1 = &mut s;
        println!("Printing two references using scope: \n{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);
//  DO NOT COMBINE MUTABLE AND IMMUTABLE REFERENCES IE:
//  let r3 = &s
//  This would cause a failed compile

    let mut s = String::from("Built by Dr Light");
    let fw = first_word(&s);
    println!("{}", fw);

}

/*
------- Function that doesn't compile
        Tries to change a borrowed string 

fn change(some_string: &String) {
//    some_string.push_str(", world");
} */
/*
------- Function that compiles
        Changes a mutable reference to a borrowed value

        NOTE: Can only have reference to one borrowed value at a time
*/

fn change(some_string: &mut String){
    some_string.push_str(" world!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len();
}