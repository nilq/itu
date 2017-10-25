mod itu;
use itu::*;

fn main() {
    let test = r#"
a := 123
b: str  = r"hey"
c: char = 'a'
    "#;
    
    let lexer = lexer(&mut test.chars());
    
    for token in lexer {
        println!("{:#?}", token)
    }
}
