mod stack;
use regex::Regex;
use stack::Stack;

fn main() {
    let s = "42 * (543+43)"; 

    let n = expression(s);
    println!("{n}");
}
fn expression(expr: &str) -> u32 {
    let mut v: Vec<Token> = Vec::new();
    let _v = tokenize(expr, &mut v);
    let ex = parse_expression(_v);
    rpn(ex.vec)
}

// Tokenize the expression
fn tokenize<'a>(expr: &str, vec: &'a mut Vec<Token>) -> &'a Vec<Token> {
    let re_numbers = Regex::new(r"\d+").unwrap();
    let re_arithmetic = Regex::new(r"[-+/*]").unwrap();
    let re_parentheses = Regex::new(r"[()]").unwrap();
    let mut rec: bool = false;
    for i in expr.chars() {
        if re_numbers.is_match(&i.to_string()) && !rec {
            vec.push(Token::Number(i.to_digit(10).unwrap()));
            rec = true;
        } else if re_numbers.is_match(&i.to_string()) && rec {
            let t = match vec.pop().unwrap() {
                Token::Number(n) => 10 * n,
                _ => 0,
            };
            println!("{t}");
            vec.push(Token::Number(i.to_digit(10).unwrap() + t));
        }
        if re_arithmetic.is_match(&i.to_string()) {
            vec.push(Token::Operator(i));
            rec = false;
        }
        if re_parentheses.is_match(&i.to_string()) {
            vec.push(Token::Parenthesis(i));
            rec = false;
        }
    }
    vec
}

fn parse_expression(tokens: &Vec<Token>) -> Stack<Token> {
    //shunting rail algorithm
    let mut num_stack: Stack<Token> = Stack::new();
    let mut oper_stack: Stack<Token> = Stack::new();

    for t in tokens {
        match t {
            Token::Number(_) => num_stack.push(t.clone()),
            Token::Operator(op) => {
                while let Some(Token::Operator(top_op)) = oper_stack.peek() {
                    if precedence(top_op) >= precedence(&op) {
                        num_stack.push(oper_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                oper_stack.push(t.clone());
            }
            Token::Parenthesis('(') => oper_stack.push(t.clone()),
            Token::Parenthesis(')') => {
                //go through opstack till "(/" found  pop operators and push them to the num stack)
                for i in oper_stack.vec.clone().into_iter().rev() {
                    match i {
                        Token::Parenthesis('(') => {
                            let g = oper_stack.pop();
                        }
                        Token::Operator(_) => {
                            num_stack.push(oper_stack.pop().unwrap());
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }
    while !oper_stack.is_empty() {
        num_stack.push(oper_stack.pop().unwrap());
    }
    num_stack
}
fn precedence(c: &char) -> u32 {
    match c {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 69,
    }
}

fn rpn(tokens: Vec<Token>) -> u32 {
    let mut ans : Stack<u32> = Stack::new();

    for t in tokens {
        match t {
            Token::Number(n) => ans.push(n),
            Token::Operator(op) => {
                let b = ans.pop().expect("stack underflow");
                let a = ans.pop().expect("stack underflow");

                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => panic!("Unexpected operator"),
                };
                ans.push(result);
            }
            _ => panic!("unknown toeken")
        }
    }
    ans.pop().unwrap()
}
    

#[derive(Clone, Debug)]
enum Token {
    Number(u32),
    Operator(char),
    Parenthesis(char),
}
