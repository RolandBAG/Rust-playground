use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let x = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("Third element: {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("Third Element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = v.get(100);
    does_it_exist(does_not_exist);
    let does_exist = v.get(3);
    does_it_exist(does_exist);
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // This Vector will be dropped once out of scope
    {
        let not_long_for_this_world = vec![1, 2, 3, 4];
    }

    // Note: You can't iterate over strings, do it this way

    println!("Testing iteration through string:");
    for c in "Test".chars() {
        print!("{c} ");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    
    // Overwriting a value

    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Add key if not present already

    scores.entry(String::from("Yellow")).or_insert(60);
    scores.entry(String::from("Blue")).or_insert(76);
    scores.entry(String::from("Green")).or_insert(100);
    println!("Adding new scores\nYellow: 60, Blue: 76, Green: 100\n{:?}", scores);

    let text = "hello world wonderful world blood whistle hear its cry";

    let mut text_map = HashMap::new();

    // Updating hash values based on existing values

    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word count in the text 'hello world wonderful world blood whistle hear its cry'\n{:?}", text_map);

    // Get median and mode of list of integers

    let mut num_list_1 = vec![ 1, 2, 3, 4, 5, 6, 4, 3, 2, 2, 2, 5, 6, 1, 3, 2, 4, 5 ];
    let mut num_list_2 = vec![ 1, 2, 2, 4, 3, 4, 4];
    num_list_1.sort();
    num_list_2.sort();
    println!("Median for even Number List: {}", median(&num_list_1));
    println!("Median for odd Number List: {}", median(&num_list_2));
    println!("Mean: {}", mean(&num_list_1));
    println!("Mode: {}", mode(&num_list_1));

}

fn median(array : &Vec<i32>) -> i32 {
    let median_num = (array.len() / 2) as f64;
    println!("Array size is: {}", array.len());
    let mut median_var = 0;
    println!("{:?}", array);

    if median_num % 2.0 == 0.0 {
        let lower_num_med = median_num.floor();
        median_var = (array[lower_num_med as usize] + array[(lower_num_med+1.0) as usize]) / 2;
    }
    else {
        median_var = array[median_num as usize];
    }
    median_var

}

fn mean(array : &Vec<i32>) -> f32 {
    let mut sum =0;
    for i in array {
        sum+=i;
    }
    sum as f32 / array.len() as f32
}

fn mode(array : &Vec<i32>) -> i32 {
    let mut count = HashMap::new();
    for i in array {
        count.entry(i).or_insert(1);
    }
    let mut mode_val = 0;
    for (k, v) in count.iter() {
        if *v > mode_val {
            mode_val = *v;
        }
    }
    mode_val
}

fn does_it_exist( val : Option<&i32>) {
    match val {
        Some(v) => println!("The value is: {}", v),
        None => println!("This value is nonexistent."),
    }
}
//    let does_not
