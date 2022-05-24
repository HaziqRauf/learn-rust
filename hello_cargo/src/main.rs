fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    println!("{}",s1);
    // println!("{}",s2);

    let s3 = takes_and_gives_back(s2);

    println!("{}",s3);
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String{
    a_string
}
