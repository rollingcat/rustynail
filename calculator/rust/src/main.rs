
use std::char::UnicodeChar;
use std::cmp::Ordering;

static MAX_DIGIT_NUM: uint = 19u;

fn main() {
	println!("[Calculator]");

	let tests = vec!["2*3",
	// 				 "999+17",
	// 				"12-7",
	// 				"12*7",
	// 				"7-12",
	// 				"((3))",
	// 				"(0-3)",
					"(0-2)*2",
					"(0-3)*27",
	// 				"12+3+123",
	// 				"10-5-25*2",
					"999*12",
					"123*(10-7)",
	// 				"(2))",
					];

	for test in tests.iter() {
		println!("--------------------------------------------");
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

		let result = rpn_with_big_int(postfix);
	  	match result {
	  		Some(x) => println!("result: {}", x),
	  		None => println!("None"),
	  	}
    }
}

fn rpn(input: Vec<&str>) -> Option<i64> {
	let mut stack: Vec<i64> = Vec::new();

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
			let number = token.parse::<i64>();
			stack.push(number.unwrap());
		}
	}
	if stack.len() != 1 {
		println!("Invalid postfix notation");
		return None;
	}
	Some(*stack.last().unwrap())
}

fn compare(a: &str, b: &str) -> Ordering {
	if a.len() > b.len() {
		return Ordering::Greater;
	} else if a.len() < b.len() {
		return Ordering::Less;
	}

	let mut a_ch: u8;
	let mut b_ch: u8;
	for i in range(0u, a.len()) {
		a_ch = a.char_at(i) as u8;
		b_ch = b.char_at(i) as u8;
		if a_ch > b_ch {
			return Ordering::Greater;
		} else if a_ch < b_ch {
			return Ordering::Less;
		}
	}
	Ordering::Equal
}

fn add(a: &str, b: &str) -> String {
	let a_num = a.parse::<i64>();
	let b_num = b.parse::<i64>();
	let new_num = a_num.unwrap() + b_num.unwrap();
	new_num.to_string()
}

fn add_big_int(a: &str, b: &str) -> String {
	let add_len;
	let a_len = a.len();
	let b_len = b.len();

	if a_len >= b.len() { add_len = a_len; }
	else { add_len = b_len; }

	let mut ret = String::with_capacity(add_len + 1);

	let zero_ch: u8 = '0' as u8;
	let mut transfer = 0u8;
	for i in range(1u, add_len+1) {
		let a_ch: u8;
		let b_ch: u8;
		if i > a_len { a_ch = 0u8; }
		else {
			a_ch = a.char_at(a_len-i) as u8 - zero_ch;
		}
		if i > b_len { b_ch = 0u8; }
		else {
			b_ch = b.char_at(b_len-i) as u8 - zero_ch;
		}

		let sum = a_ch + b_ch + transfer;
		transfer = sum / 10u8;
		let num = sum % 10u8 + zero_ch;
		ret.insert(0, num as char);
	}
	if transfer != 0u8 {
		ret.insert(0, (transfer + zero_ch) as char);
	}

	ret
}

fn subtract(a: &str, b: &str) -> String {
	let a_num = a.parse::<i64>();
	let b_num = b.parse::<i64>();
	let new_num = a_num.unwrap() - b_num.unwrap();
	new_num.to_string()
}

fn subtract_big_int(a_in: &str, b_in: &str) -> String {
	let a: &str;
	let b: &str;

	let comp = compare(a_in, b_in);
	if comp == Ordering::Greater {
		a = a_in;
		b = b_in;
	} else if comp == Ordering::Less {
		a = b_in;
		b = a_in;
	} else {
		return "0".to_string();
	}

	let a_len = a.len();
	let b_len = b.len();
	let sub_len = a_len;

	let mut ret = String::with_capacity(sub_len + 1);

	let zero_ch: i8 = '0' as i8;
	let mut borrow = 0i8;
	for i in range(1u, sub_len+1) {
		let a_ch: i8;
		let b_ch: i8;
		if i > a_len { a_ch = 0i8; }
		else {
			a_ch = a.char_at(a_len-i) as i8 - zero_ch;
		}
		if i > b_len { b_ch = 0i8; }
		else {
			b_ch = b.char_at(b_len-i) as i8 - zero_ch;
		}

		let mut sub = a_ch - b_ch - borrow;
		if sub < 0i8 {
			borrow = 1i8;
			sub += 10i8;
		} else {
			borrow = 0i8;
		}

		ret.push(((sub + zero_ch) as u8) as char);
	}

	while ret.char_at(ret.len()-1) == '0' {
		ret.pop();
	}

	if comp == Less {
		ret.push('-');
	}

	ret = ret.chars().rev().collect();
	ret
}

fn multiply(a: &str, b: &str) -> String {
	let a_num = a.parse::<i64>();
	let b_num = b.parse::<i64>();
	let new_num = a_num.unwrap() * b_num.unwrap();
	new_num.to_string()
}

fn normalize(input: &mut Vec<uint>) {
	input.push(0u);

	for i in range(0u, input.len()) {
		if input[i] == 0u {
			continue;
		}
		input[i+1] += input[i] / 10u;
		input[i] %= 10;
	}

	while *input.last().unwrap() == 0u {
		input.pop();
	}
}

fn multiply_big_int_brute_force(a_in: &str, b_in: &str) -> String {
	let mut a: &str = a_in;
	let mut b: &str = b_in;
	if a.char_at(0) == '-' { a = a_in.slice_from(1); }
	if b.char_at(0) == '-' { b = b_in.slice_from(1); }

	let minus: bool;
	if (a_in.char_at(0) == '-') != (b_in.char_at(0) == '-') {
		minus = true;
	} else {
		minus = false;
	}

	let a_len = a.len();
	let b_len = b.len();

	let size = a_len + b_len + 1;
	let mut buf: Vec<uint> = Vec::with_capacity(size + 1);
	for i in range(0, size) {
		buf.push(0u);
	}

	let zero_ch: uint = '0' as uint;
	for i in range(0u, a_len).rev() {
		for j in range(0u, b_len).rev() {
			let a_val = a.char_at(i) as uint - zero_ch;
			let b_val = b.char_at(j) as uint - zero_ch;
			buf[(a_len-i-1) + (b_len-j-1)] += a_val * b_val;
		}
	}
	normalize(&mut buf);

	let mut ret: String = String::with_capacity(buf.len() + 1);
	if minus {
		ret.push('-');
	}
	for i in buf.iter().rev() {
		ret.push((*i + zero_ch) as u8 as char);
	}
	ret
}

fn multiply_big_int(a: &str, b: &str) -> String {
	multiply_big_int_brute_force(a, b)
}

fn need_big_int(a: &str, b: &str, mul: bool) -> bool {
	if mul {
		if (a.len() + b.len()) > MAX_DIGIT_NUM {
			return true;
		}
	} else if a.len() >= MAX_DIGIT_NUM || b.len() >= MAX_DIGIT_NUM {
		return true;
	}
	return false;
}

fn do_add_or_subtract(a: &str, b: &str, op: bool) -> String {
	// true is +, false is -
	let a_sign: bool = a.char_at(0) != '-';
	let b_sign: bool = b.char_at(0) != '-';

	let mut a_input: &str = a;
	let mut b_input: &str = b;
	if !a_sign { a_input = a.slice_from(1); }
	if !b_sign { b_input = b.slice_from(1); }

	let mut ret: String;

	if (a_sign && op && b_sign) || (a_sign && !op && !b_sign) {
		return add_big_int(a_input, b_input);
	} else if (a_sign && op && !b_sign) || (a_sign && !op && b_sign) {
		return subtract_big_int(a_input, b_input);
	} else if (!a_sign && op && b_sign) || (!a_sign && !op && !b_sign) {
		return subtract_big_int(b_input, a_input);
	} else {
		ret = add_big_int(a_input, b_input);
		ret.insert(0, '-');
	}
	ret
}

fn rpn_with_big_int(input: Vec<&str>) -> Option<String> {
	let mut stack: Vec<String> = Vec::new();

	for token in input.iter() {
		let ch = token.char_at(0);
		if token.len() == 1 && !UnicodeChar::is_numeric(ch) {
			if stack.len() < 2 {
				println!("Invalid postfix notation");
				return None;
			}
			let b = stack.pop().unwrap();
			let a = stack.pop().unwrap();

			let first_num = a.as_slice();
			let second_num = b.as_slice();

			let new_num;
			if ch == '+' {
				if need_big_int(first_num, second_num, false) {
					new_num = do_add_or_subtract(first_num, second_num, true);
				} else {
					new_num = add(first_num, second_num);
				}
			} else if ch == '-' {
				if need_big_int(first_num, second_num, false) {
					new_num = do_add_or_subtract(first_num, second_num, false);
				} else {
					new_num = subtract(first_num, second_num);
				}
			} else if ch == '*' {
				if need_big_int(first_num, second_num, true) {
					new_num = multiply_big_int(first_num, second_num);
				} else {
					new_num = multiply(first_num, second_num);
				}
			} else {
				println!("Unsupported operator");
				return None;
			}
			stack.push(new_num);
		} else {
			stack.push(token.to_string());
		}
	}
	if stack.len() != 1 {
		println!("Invalid postfix notation");
		return None;
	}
	stack.pop()
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
						if prec2 >= prec1 {
							let pos = stack.pop().unwrap() as uint;
							postfix.push(op.slice(pos, pos + 1u));
						} else {
							break;
						}
					}
					stack.push(idx);
				}
			} else if ch == '(' {
				stack.push(-2i);
			} else if ch == ')' {
				if stack.is_empty() {
					println!("Mismatched parenthesis");
					postfix.clear();
					return postfix;
				}
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

	let mut pos;
	let mut text = input;
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

	tokens
}
