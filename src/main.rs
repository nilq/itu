mod itu;
use itu::*;

fn main() {
    let test = r#"
(a: i32): i128 ->
    (
        (a: i32): i128 -> a + 10
    )
    "#;

    let lexer = lexer(&mut test.chars());

    let traveler   = Traveler::new(lexer.collect());
    let mut parser = Parser::new(traveler);

    match parser.parse() {
        Err(why)  => println!("error: {}", why),
        Ok(stuff) => println!("{:#?}", stuff),
    }
}
