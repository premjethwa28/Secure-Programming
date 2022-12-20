//Name : Prem Atul Jethwa
//UTA ID : 1001861810
//References -
//https://www.rust-lang.org/
//https://www.geeksforgeeks.org/introduction-to-rust-programming-language/
//https://towardsdatascience.com/a-comprehensive-tutorial-to-rust-operators-for-beginners-11554b2c64d4
//http://web.mit.edu/rust-lang_v1.26.0/arch/amd64_ubuntu1404/share/doc/rust/html/std/ops/index.html
//The rust calculator created is Post Fix Calculator i.e. input numbers first and then operator. 
//For eg: 2 2 + ; will put 4 in stack.

use std::io::{self, BufRead, Write};
use std::collections::VecDeque;
use std::ops::{Add, Sub, Mul, Div, Neg, Not, Rem, BitAnd, BitOr, Shl, Shr};

type Stack = VecDeque<f64>;

// Available operation in the calculator
enum StackOperator {
    Add,       // The addition operator +
    Sub,       // The subtraction operator -
    Mul,       // The multiplication operator *
    Div,       // The division operator /
    Shl,       // The left shift operator <<
    Shr,       // The right shift operator >>
    Rem,       // The remainder operator % or modulus
    BitAnd,    // The bitwise AND operator &
    BitOr,     // The bitwise OR operator |
    Not,       // The unary logical negation operator !
    Neg,       // The unary negation operator -
    Sum,       // Sums the entire stack
    Prod,      // Multiplies the entire stack
    Pop,       // Pops an item off the stack
    Clear,     // Clears the stack
    NoOp,      // No operation (in case of error)
    Num(f64),  // A number
}

// Prints the list of available operators to the terminal
fn print_list() -> StackOperator {
    println!("List of available operators: ");
    println!("help, ? : print this help");
    println!("<number> : Pushes a number to the stack");
    println!("+, -, *, / : Applies the respective binary operation");
    println!("neg : Negates the last number");
    println!("! : Not the number");
    println!("% : The remainder of last 2 numbers");
    println!("sum : Add the entire stack together");
    println!("prod : Multiplies the entire stack together");
    println!("& : Bitwise AND the last 2 numbers");
    println!("| : Bitwise OR the last 2 numbers");
    println!(">> : Right Shift the stack");
    println!("<< : Left Shift the stack");
    println!("pop : Removes the topmost number");
    println!("clear : Clears the stack");
    println!("exit : Exit the code");
    println!("Enter number and hit return");
    StackOperator::NoOp
}

// Parses a string and returns a stack-operator
fn parse_string(input: &str) -> StackOperator {
    use StackOperator::*;

    match input.trim() {
        // Binary operations
        "+" | "add" | "addition" => Add,
        "-" | "sub" | "subtract" => Sub,
        "*" | "mul" | "multiply" => Mul,
        "/" | "div" | "divide"   => Div,
        "%" | "rem" => Rem,
        // Unary operations
        "neg" | "negate" | "~" => Neg,
        "!" | "not"  => Not,
        // Bitwise operations
        "&" | "bitand" => BitAnd,
        "|" | "bitor" => BitOr,
        // Shift operations
        ">>" | "shr" => Shr,
        "<<" | "shl" => Shl,
        // Stack operations
        "sum"  => Sum,
        "prod" => Prod,
        "pop"  => Pop,
        "clear" | "cls" => Clear,
        // Other
        "list" | "?" => print_list(),
        "exit" | "q" | "end" => {
            println!("To exit, press ctrl+c");
            NoOp
        },
        // Number
        str => {
            if let Ok(num) = str.parse::<f64>() {
                Num(num)
            } else {
                println!("Error! Couldn't parse {}", str);
                NoOp
            }
        }
    }
}

// Prompts the user for an input from the console
fn get_input() -> io::Result<StackOperator> {
    let mut buff: String = String::new();
    let stdin = io::stdin();

    print!("--> ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut buff)?;
    buff = buff.to_lowercase(); // ensure lowercase

    Ok(parse_string(&buff))
}

// So if we push 2 1 - the operation becomes 2 - 1, not 1 - 2
// For floating values
fn eval_binoperator<F>(stack: &mut Stack, fun: F)
where
    F: FnOnce(f64, f64) -> f64,
{
    if stack.len() >= 2 {
        let a = stack.pop_back().unwrap();
        let b = stack.pop_back().unwrap();
        stack.push_back(fun(b, a));
    }
}

// For integer values
fn eval_binoperatorint<F>(stack: &mut Stack, fun: F)
where
    F: FnOnce(i64, i64) -> i64,
{
    if stack.len() >= 2 {
        let a = stack.pop_back().unwrap() as i64;
        let b = stack.pop_back().unwrap() as i64;
        stack.push_back(fun(b, a) as f64);
    }
}

// Applies a unary operation if the stack has enough elements
// For floating values
fn eval_unoperator<F>(stack: &mut Stack, fun: F)
where
    F: FnOnce(f64) -> f64,
{
    if let Some(a) = stack.pop_back() {
        stack.push_back(fun(a));
    }
}

//For integer values
fn eval_unoperatorint<F>(stack: &mut Stack, fun: F)
where
    F: FnOnce(i64) -> i64,
{
    if let Some(a) = stack.pop_back() {
        stack.push_back(fun(a as i64) as f64);
    }
}

// Folds the stack over fun, then pushes the result
fn eval_stackoperator<F>(stack: &mut Stack, start: f64, fun: F)
where
    F: FnMut(f64, &f64) -> f64,
{
    let result = stack.iter().fold(start, fun);
    stack.clear();
    stack.push_back(result);
}

// Determines what to do given a StackOp, and applies its effect to the stack
fn eval(stack: &mut Stack, last_op: StackOperator) {
    use StackOperator::*;

    match last_op {
        // Binary operators
        Add => eval_binoperator(stack, f64::add),
        Sub => eval_binoperator(stack, f64::sub),
        Mul => eval_binoperator(stack, f64::mul),
        Div => eval_binoperator(stack, f64::div),
        Rem  => eval_binoperatorint(stack, i64::rem),
        //Bitwise operators
        BitAnd => eval_binoperatorint(stack, i64::bitand),
        BitOr  => eval_binoperatorint(stack, i64::bitor),
        // Unary operators
        Neg  => eval_unoperator(stack, f64::neg),
        Not  => eval_unoperatorint(stack, i64::not),
        // Shift operators
        Shr  => eval_binoperatorint(stack, i64::shr),
        Shl  => eval_binoperatorint(stack, i64::shl),
        // Stack operations
        Sum   => eval_stackoperator(stack, 0.0, |acc, x| acc + x),
        Prod  => eval_stackoperator(stack, 1.0, |acc, x| acc * x),
        Pop   => { stack.pop_back(); }, 
        Clear => stack.clear(),
        // Number
        Num(n) => stack.push_back(n),
        // Other
        NoOp => return,
        BitAnd => todo!(),
        BitOr => todo!(),
        Shl => todo!(),
        Shr => todo!(),  // do nothing
    }
}

// Main Function
fn main() -> io::Result<()> {
    println!("Welcome to my Rust Calculator!");
    println!("Type \"list\" and hit return to view available operators.");
    let mut stack = VecDeque::new();
    loop {
        let input = get_input()?;
        eval(&mut stack, input);

        if stack.len() >= 1 {
            println!("Stack: {:.2?}", stack);
        }
    }
}

