mod calculator_module;

use calculator_module::calculator::{ReversePolishNotationConverter, ReversePolishNotationParser};

fn main() {

    let formula = "sin ( 45 * 2 )".split(" ").collect();
    let converter = ReversePolishNotationConverter::new();
    let reverse_polish_notation = converter.convert(formula);

    match reverse_polish_notation {
        Ok(value) => {
            let mut parser = ReversePolishNotationParser::new();
            println!("Reverse polish notation formula: {:?}", value);
            let result: Result<f64, String> = parser.parse(&value);
            match result {
                Ok(value) => println!("Hello, world!, {:?}", value),
                Err(error) => println!("{:?}", error),
            }
        },
        Err(error) => println!("Could not convert the formula due to error: {:?}", error)
    }
    
}
