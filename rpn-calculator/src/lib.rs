#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(val) => stack.push(*val),
            _ => match (stack.pop(), stack.pop()) {
                (Some(x), Some(y)) => stack.push(get_operation(input)(y, x)),
                _ => break,
            },
        }
    }
    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}

fn get_operation(operator: &CalculatorInput) -> fn(i32, i32) -> i32 {
    match operator {
        CalculatorInput::Add => |a, b| -> i32 { a + b },
        CalculatorInput::Subtract => |a, b| -> i32 { a - b },
        CalculatorInput::Multiply => |a, b| -> i32 { a * b },
        CalculatorInput::Divide => |a, b| -> i32 { a / b },
        CalculatorInput::Value(_) => panic!("operator should not be value"),
    }
}
