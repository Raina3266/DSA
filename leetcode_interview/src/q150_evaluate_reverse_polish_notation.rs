enum Operators {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operators {
    fn parse(input: &str) -> Option<Self> {
        match input {
            "+" => Some(Self::Add),
            "-" => Some(Self::Sub),
            "*" => Some(Self::Mul),
            "/" => Some(Self::Div),
            _ => None,
        }
    }
    fn calculate(&self, num1: i32, num2: i32) -> i32 {
        match self {
            Operators::Add => num1 + num2,
            Operators::Sub => num1 - num2,
            Operators::Mul => num1 * num2,
            Operators::Div => num1 / num2,
        }
    }
}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    eval(tokens).0
}

// assume already reversed: + 1 2
fn eval(mut tokens: Vec<String>) -> (i32, Vec<String>) {
    if let Some(first) = tokens.pop() {
        if let Some(op) = Operators::parse(&first) {
            // recursive case
            let (a, rest) = eval(tokens);
            let (b, rest) = eval(rest);
            (op.calculate(b, a), rest)
        } else {
            // base case - it's just an integer
            let first = first.parse().unwrap();
            return (first, tokens);
        }
    } else {
        // Empty input — only reachable for malformed expressions.
        (0, tokens)
    }
}
