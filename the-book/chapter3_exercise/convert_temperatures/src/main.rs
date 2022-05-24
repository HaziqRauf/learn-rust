use std::io;

fn main() {
    
    println!("Please input the temperature in fahrenheit.");

    loop {
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }

}
