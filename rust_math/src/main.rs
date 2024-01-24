use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut user_input = String::new();
    stdin.read_line(&mut user_input)?;
    println!("Input: {} ", user_input);
    Ok(())
}

fn vertex_form() {

}