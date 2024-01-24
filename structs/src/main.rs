struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("roland"),
        email: String::from("rol@rol.com.au"),
        sign_in_count: 1,
    };

    user1.email = String::from("rolandbeowulf");

    let mut user2 = build_user(String::from("Jackjake"), String::from("jack@jake.com.au"));

    let user3 = User {
        active: user1.active,
        username: user2.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
/*
    let user4 = User {
        email: String::from("anotheranother@example.com"),
        ..user2
    };
*/
    let black = Color(0, 0, 0);
    let subject = AlwaysEqual;

    let rect = (30, 50);

    let rect1: (u32, u32) = (30, 50);

    println!("The area of defined rectangle is {} square pixels", area(rect1));

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle Area: {} Square Pixels", rect1.area());
    if rect1.width() {
        println!("Rectangle with nonzero width: {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 70,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}