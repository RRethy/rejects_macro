extern crate rejects_macro;

use rejects_macro::rejects;

fn main() {
    let re = rejects! { r"\d\d\d" };
    println!("{}", re.find_end("123ds"));
    println!("Hello, world!");
}
