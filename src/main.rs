mod itu;
use itu::*;

fn main() {
    let test = r#"
a := 123
b: str = r"hey"
c: char
    "#;
    
    let lexer = lexer(&mut test.chars());

    let traveler   = Traveler::new(lexer.collect());
    let mut parser = Parser::new(traveler);

    match parser.parse() {
        Err(why)  => println!("error: {}", why),
        Ok(stuff) => println!("{:#?}", stuff),
    }
}
