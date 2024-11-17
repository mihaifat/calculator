pub mod calculator {

    use std::collections::{HashMap, VecDeque};

    const OPERATORS: [&str; 5] = ["+", "-", "/", "*", "^"];

    const FUNCTIONS: [&str; 1] = ["sin"];


    #[derive(PartialEq)]
    enum Associativity {
        LEFT,
        RIGHT,
    }

    struct Function {
        symbol: &'static str
    }

    impl Function {
        const SIN: Function = Function {
            symbol: "sin"
        };
    }

    struct Operator {
        symbol: &'static str,
        associativity: Associativity,
        precedence: i8
    }

    impl Operator {
        const ADDITION: Operator = Operator {
            symbol: "+",
            associativity: Associativity::LEFT,
            precedence: 0
        };

        const SUBTRACTION: Operator = Operator {
            symbol: "-",
            associativity: Associativity::RIGHT,
            precedence: 0,
        };

        const DIVISION: Operator = Operator {
            symbol: "/",
            associativity: Associativity::LEFT,
            precedence: 5,
        };

        const MULTIPLICATION: Operator = Operator {
            symbol: "*",
            associativity: Associativity::LEFT,
            precedence: 5,
        };

        const MODULUS: Operator = Operator {
            symbol: "%",
            associativity: Associativity::LEFT,
            precedence: 5,
        };

        const POWER: Operator = Operator {
            symbol: "^",
            associativity: Associativity::RIGHT,
            precedence: 10,
        };
    }


    pub struct ReversePolishNotationConverter<'a> {
        operator_map: HashMap<&'a str, &'a Operator>,
        function_map: HashMap<&'a str, &'a Function>
    }

    impl ReversePolishNotationConverter<'_> {

        pub fn new() -> Self {
            let mut operator_map = HashMap::new();
            operator_map.insert(Operator::ADDITION.symbol, &Operator::ADDITION);
            operator_map.insert(Operator::SUBTRACTION.symbol, &Operator::SUBTRACTION);
            operator_map.insert(Operator::DIVISION.symbol, &Operator::DIVISION);
            operator_map.insert(Operator::MULTIPLICATION.symbol, &Operator::MULTIPLICATION);
            operator_map.insert(Operator::MODULUS.symbol, &Operator::MODULUS);
            operator_map.insert(Operator::POWER.symbol, &Operator::POWER);

            let mut function_map = HashMap::new();
            function_map.insert(Function::SIN.symbol, &Function::SIN);
            
            Self {
                operator_map,
                function_map
            }
        }

        pub fn convert(&self, tokens: Vec<&str>) -> Result<String, String> {
            let mut stack: Vec<&str> = Vec::new();
            let mut output: VecDeque<&str> = VecDeque::new();

            let is_operator = |token| -> bool {
                self.operator_map.contains_key(token)
            };

            let is_function = |token: &str| -> bool {
                self.function_map.contains_key(token)
            };

            for token in tokens {
                if is_function(token) {
                    stack.push(token);
                } else if is_operator(token) {
                    while !stack.is_empty() && self.operator_map.contains_key(stack.last().ok_or_else(|| "Cannot fetch last element from stack")?) {
                        let current_operator = self.operator_map.get(token).ok_or_else(|| "Cannot fetch current operator".to_string())?;
                        let top_stack_element = stack.last().ok_or_else(|| "Empty stack".to_string())?;
                        let top_operator = self.operator_map.get(top_stack_element).ok_or("Cannot fetch top operator".to_string())?;
                        if (current_operator.associativity == Associativity::LEFT && current_operator.precedence <= top_operator.precedence) ||
                        (current_operator.associativity == Associativity::RIGHT && current_operator.precedence < top_operator.precedence) {
                            let element = stack.pop().ok_or_else(|| "Could not fetch element from stack")?;
                            output.push_back(element);
                            continue;
                        }
                        break;
                    }
                    stack.push(token)
                } else if token.eq("(") {
                    stack.push(token);
                } else if token.eq(")") {
                    while !stack.is_empty() {
                        let element = stack.pop().ok_or_else(|| "Could not pop from stack")?;
                        if !element.eq("(") {
                            output.push_back(element);
                        }
                        if is_function(stack.last().ok_or_else(|| "Could not parse last token for is_function check")?) {
                            output.push_back(stack.pop().ok_or_else(|| "Could not pop from stack to send it to output queue")?);
                        }
                    }
                    stack.pop();
                } else {
                    output.push_back(token);
                }
            }

            while !stack.is_empty() {
                let stack_element = stack.pop().ok_or_else(|| "Could not fetch element from stack")?;
                output.push_back(stack_element);
            }

            let output_string = Vec::from(output).join(" ");
            Ok(output_string)
        }

    }

    pub struct ReversePolishNotationParser {
        operand_stack: Vec<f64>

    }

    impl ReversePolishNotationParser {

        pub fn new() -> Self {
            Self { 
                operand_stack: Vec::new()
            }
        }

        pub fn parse(&mut self, expression: &str) -> Result<f64, String> {
            let token_list = expression.split(" ");
            for token in token_list {
                if OPERATORS.contains(&token) {
                    let operand2 = self.operand_stack.pop().ok_or_else(|| "Could not fetch operand2")?;
                    let operand1 = self.operand_stack.pop().ok_or_else(|| "Could not fetch operand1")?;
                    let result = self.calculate(token, operand1, operand2);
                    match result {
                        Ok(value) => self.operand_stack.push(value),
                        Err(error) => return Err(error),
                    }
                } else if FUNCTIONS.contains(&token) {
                    let operand = self.operand_stack.pop().ok_or_else(|| "Could not fetch operand")?;
                    let result = self.apply_function(token, operand);
                    match result {
                        Ok(value) => self.operand_stack.push(value),
                        Err(error) => return Err(error),
                    }
                } else {
                    let result = token.parse::<f64>().map_err(|_| "Could not parse operand".to_string());
                    match result {
                        Ok(value) => self.operand_stack.push(value),
                        Err(error) => return Err(error),
                    }
                }
            }
            let result = self.operand_stack.pop();
            match result {
                Some(value) => Ok(value),
                None => Ok(0f64) 
            }
        }

        pub fn calculate(&self, operator: &str, operand1: f64, operand2: f64) -> Result<f64, String> {
            match operator {
                "+" => Ok(operand1 + operand2),
                "-" => Ok(operand1 - operand2),
                "*" => Ok(operand1 * operand2),
                "/" => Ok(operand1 / operand2),
                "^" => Ok(operand1.powf(operand2)),
                _ => Err("Operator ".to_string() + operator + " is not handled")
            }
        }

        fn apply_function(&self, function: &str, operand: f64) -> Result<f64, String> {
            match function {
                "sin" => Ok(f64::sin(operand)),
                _ => Err("Function ".to_string() + function + " is not handled")
            }
        }

    }
}