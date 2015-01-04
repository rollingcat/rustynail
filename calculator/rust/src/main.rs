
use std::char::UnicodeChar;

fn main() {
    println!("Calculator");

    let tests = vec!["3 + 4 * 2 - ( 1 - 5 ) + 7",
    				 "123",
    				 "3+4 * 2 *(5 - 3 )",
    				 "(((((((1+2+3*(4 + 5))))))",
    				 "a - (b + c+d * 4)!",
    				 "3 + 4 * 2 ( 1 - 5 ) + 7",
    				 "3 + 4 * 2 - * ( 1 - 5 ) + 7"];

    for test in tests.iter() {
    	println!("infix: {}", test);

  		let tokens = tokenize(test.as_slice());

		print!("tokenize: ");
  		for token in tokens.iter() {
  			print!("{} ", token);
  		}
  		println!("");

  		let postfix = convert_to_postfix(tokens);
  		print!("postfix: ");
  		if postfix.is_empty() {
  			println!("failed");
  		} else {
	  		for token in postfix.iter() {
	  			print!("{} ", token);
	  		}
	  		println!("");
	  	}

	  	let result = rpn(postfix);
	  	match result {
	  		Some(x) => println!("result: {}", x),
	  		None => println!("None"),
	  	}
    }
}

fn rpn(input: Vec<&str>) -> Option<int> {
	let mut stack: Vec<int> = Vec::new();

	for token in input.iter() {
		let ch = token.char_at(0);
		if token.len() == 1 && !UnicodeChar::is_numeric(ch) {
			if stack.len() < 2 {
				println!("Invalid postfix notation");
				return None;
			}
			let second_num = stack.pop().unwrap();
			let first_num = stack.pop().unwrap();

			let mut new_num;
			if ch == '+' {
				new_num = first_num + second_num;
			} else if ch == '-' {
				new_num = first_num - second_num;
			} else {
				new_num = first_num * second_num;
			}
			stack.push(new_num);
		} else {
			let number = token.parse::<int>();
			stack.push(number.unwrap());
		}
	}
	if stack.len() != 1 {
		println!("Invalid postfix notation");
		return None;		
	}
	Some(*stack.last().unwrap())
}

fn convert_to_postfix(input: Vec<&str>) -> Vec<&str> {
	let mut postfix: Vec<&str> = Vec::new();
	let mut stack: Vec<int> = Vec::new();
	let op = "+-*";

	for token in input.iter() {
		let ch = token.char_at(0);
		if token.len() == 1 && !UnicodeChar::is_numeric(ch) {			
			let position = op.find(|&: c: char| {
				if c == ch { true }
				else { false }
			});
			if position != None {
				let idx = position.unwrap() as int;
				if stack.is_empty() {
					stack.push(idx);
				} else {
					while !stack.is_empty() {
						let prec2 = *stack.last().unwrap() / 2i;
						let prec1 = idx / 2i;
						if prec2 > prec1 {
							let pos = stack.pop().unwrap() as uint;
							postfix.push(op.slice(pos, pos + 1u));
						} else {
							break;
						}
					}
					stack.push(idx);
				}
			} else if ch== '(' {
				stack.push(-2i);
			} else if ch == ')' {
				while *stack.last().unwrap() != -2i {
					let pos = stack.pop().unwrap() as uint;
					postfix.push(op.slice(pos, pos + 1u));
				}
				stack.pop();
			} else {
				println!("Invalid character");
				postfix.clear();
				return postfix;
			}
		} else {
			let ret = token.find(|&: c: char| {
				if !UnicodeChar::is_numeric(c) { true }
				else { false }
			});
			if ret != None {
				println!("Invalid character");
				postfix.clear();
				return postfix;
			} else {
				postfix.push(*token);
			}
		}
	}
	while !stack.is_empty() {
		if *stack.last().unwrap() == -2i {
			println!("Mismatched parenthesis");
			postfix.clear();
			return postfix;
		}
		let pos = stack.pop().unwrap() as uint;
		postfix.push(op.slice(pos, pos + 1u));
	}

	postfix
}

fn tokenize(input: &str) -> Vec<&str> {
	let mut tokens = Vec::new();

	let split: Vec<&str> = input.split(' ').collect();	

	for word in split.iter() {
		if word.is_empty() {
			continue;
		}
		let &mut text = word;
		let mut pos;
		loop {
			let position = text.find(|&: c: char| {
				match c {
					'+' => true,
					'-' => true,
					'*' => true,
					'(' => true,
					')' => true,
					_ => false
				}
			});
			if position != None {
				pos = position.unwrap();
				if pos == 0u { // operator
					pos = 1u;
				}
				tokens.push(text.slice(0, pos));
				text = text.slice_from(pos);
			} else {
				if !text.is_empty() {
					tokens.push(text);
				}
				break;
			}
		}
	}

	tokens
}