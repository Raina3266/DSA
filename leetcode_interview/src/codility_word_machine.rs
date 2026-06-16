/*
A word machine is a system that performs a sequence of simple operations on a stack of integers.

Initially the stack is empty.

The sequence of operations is given as a string.

Operations are separated by single spaces.

The following operations may be specified:

an integer X (from 0 to 2²⁰ − 1) → the machine pushes X onto the stack;

"POP" → the machine removes the topmost number from the stack;

"DUP" → the machine pushes a duplicate of the topmost number onto the stack;

"+" → the machine pops the two topmost elements from the stack, adds them together and pushes the sum onto the stack;

"-" → the machine pops the two topmost elements from the stack, subtracts the second one from the first (topmost) one and pushes the difference onto the stack.

After processing all the operations, the machine returns the topmost value from the stack.

The machine processes 20-bit unsigned integers (numbers from 0 to 2²⁰ − 1).

An overflow in addition or underflow in subtraction causes an error.

The machine also reports an error when it tries to perform an operation that expects more numbers on the stack than the stack actually contains.

Also, if after performing all the operations, the stack is empty, the machine reports an error.
*/

fn word_machine(input: &str) -> Result<u32, MachineError> {
    let mut stack: Vec<u32> = vec![];
    for i in input.split_whitespace() {
        match i {
            "DUP" => {
                let Some(last) = stack.pop() else {
                    return Err(MachineError::EmptyStack);
                };
                stack.push(last);
                stack.push(last);
            }
            "POP" => {
                let Some(_last) = stack.pop() else {
                    return Err(MachineError::EmptyStack);
                };
            }
            "+" => {
                let Some(first) = stack.pop() else {
                    return Err(MachineError::EmptyStack);
                };
                let Some(second) = stack.pop() else {
                    return Err(MachineError::EmptyStack);
                };
                if first + second < 2u32.pow(20) {
                    stack.push(first + second);
                } else {
                    return Err(MachineError::Overflow);
                }
            }
            "-" => {
                let Some(first) = stack.pop() else {
                    return Err(MachineError::EmptyStack);
                };
                let Some(second) = stack.pop() else {
                    return Err(MachineError::EmptyStack);
                };
                if first >= second {
                    stack.push(first - second);
                } else {
                    return Err(MachineError::Underflow);
                }
            }
            _ => {
                let Ok(num) = i.parse::<u32>() else {
                    return Err(MachineError::Overflow);
                };
                if num > 2u32.pow(20) - 1 {
                    return Err(MachineError::Overflow);
                }
                stack.push(num);
            }
        }
    }
    if let Some(result) = stack.pop() {
        Ok(result)
    } else {
        Err(MachineError::EmptyStack)
    }
}

#[derive(Debug, PartialEq)]
enum MachineError {
    EmptyStack,
    Overflow,
    Underflow,
}

#[test]
fn test() {
    let input1 = "4 5 6 - 7 +";
    assert_eq!(word_machine(input1), Ok(8));
    let input2 = "13 DUP 4 POP 5 DUP + DUP + -";
    assert_eq!(word_machine(input2), Ok(7));
    let input3 = "5 6 + -";
    assert_eq!(word_machine(input3), Err(MachineError::EmptyStack));
    let input4 = "3 DUP 5 - -";
    assert_eq!(word_machine(input4), Err(MachineError::Underflow));
}
