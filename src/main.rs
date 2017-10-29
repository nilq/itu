mod itu;
use itu::*;

fn main() {
    let test = r#"
a: [i128] = {2^128-1}
    "#;

    let lexer = lexer(&mut test.chars());

    let traveler   = Traveler::new(lexer.collect());
    let mut parser = Parser::new(traveler);

    match parser.parse() {
        Err(err)  => match err {
            ParserError {ref value, ref position} => {
                match *position {
                    Some(ref pos) => {
                        let mut lines = test.lines();

                        for _ in 0 .. pos.line - 1 {
                            lines.next();
                        }
                        
                        let source_pos = format!("ln {}, cl {}| ", pos.line, pos.col);

                        match lines.next() {
                            Some(line) => println!("{}{}", source_pos, line),
                            None       => unreachable!(),
                        }

                        let mut error = String::from("");
                        
                        for _ in 0 .. pos.col + source_pos.len() {
                            error.push_str(" ")
                        }
                        
                        error.push_str("^ ");

                        
                        match *value {
                            ParserErrorValue::Constant(ref a) => error.push_str(a),
                        }
                        
                        println!("{}", error);
                        
                    },
                    
                    None => (),
                }
            },
        },
        Ok(stuff) => println!("{:#?}", stuff),
    }
}
