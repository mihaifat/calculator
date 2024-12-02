use calculator_module::calculator::{ReversePolishNotationConverter, ReversePolishNotationParser};

#[test]
fn test_formula_1() {
    let formula = "( 2 + 3 ) ^ ( 2 + 1 )".split(" ").collect();
    let converter: ReversePolishNotationConverter<'_> = ReversePolishNotationConverter::new();
    let reverse_polish_notation = converter.convert(formula).expect("Conversion failed");
    let mut parser = ReversePolishNotationParser::new();
    let result = parser.parse(&reverse_polish_notation).expect("Parsing failed");
    assert_eq!(reverse_polish_notation, "2 3 + 2 1 + ^");
    assert_eq!(result, 125.0);
}

#[test]
fn test_formula_2() {
    let formula = "sin ( 2 )".split(" ").collect();
    let converter = ReversePolishNotationConverter::new();
    let reverse_polish_notation = converter.convert(formula).expect("Conversion failed");
    let mut parser = ReversePolishNotationParser::new();
    let result = parser.parse(&reverse_polish_notation).expect("Parsing failed");
    assert_eq!(reverse_polish_notation, "2 sin");
    assert_eq!(result, f64::sin(2.0));
}