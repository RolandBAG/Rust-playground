use aggregator::{Summary, Tweet};


// You can define a struct with generic traits



struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}
/*
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

 */
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}



fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

// You are able to define generic types that can work with multiple variables

/*
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/

/* Dont do this wont work

let wont_work = Point { x: 5, y: 4.0 };

*/


fn main() {
    let integer = Point { x: 5, y: 10 };
    let number_list = vec![34, 50, 25, 100, 65];

//    let result = largest(&number_list);
//    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

//    let result = largest(&char_list);
//    println!("The largest char is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());




 //   println!("integer.x = {}", integer.x());
}
