//error handling

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
//enums
enum ErrorType {
    Parser,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorType,
}

impl Error {
    pub fn could_not_parse_function() -> Self {
        Self {
            kind: ErrorType::Parser,
        }
    }
    pub fn more_than_one_decimal() -> Self {
        Self {
            kind: ErrorType::Parser,
        }
    }
    pub fn decimal_with_no_numbers() -> Self {
        Self {
            kind: ErrorType::Parser,
        }
    }
}
//TODO add handling of floats, current version handles dot(.) 
//as a separator and thus persceive it as 2 numbers
///A function that uses the shunting yard algorithm to convert infix notation to reverse polish notation
pub fn reverse_polish_parsing(infix_input: &String)->Result<Vec<String>>{
    //This is using the shunting yard algorithm
    let math_ops_prio_1: Vec<char> = vec!['+', '-'];
    let math_ops_prio_2: Vec<char> = vec!['*', '/', '%'];
    let math_ops_prio_3: Vec<char> = vec!['^'];

    let separators: Vec<char> = vec!['.', ','];
    let functions: Vec<String> = vec!["sqrt".to_string(), "max".to_string(), "min".to_string()];

    let mut symbol_stack: Vec<&char> = vec![];
    let mut output: Vec<String> = vec![];
    let mut next_output_value_num = "".to_string();
    let mut next_output_value_alpha = "".to_string();
    let mut separator_encoutered: bool = false;
    let empty_string_vec: Vec<String> = vec![];
    
    let infix_as_chars: Vec<char> = infix_input.chars().collect();

    for (index, chr) in infix_as_chars.iter().enumerate() {
        if chr.is_digit(10) {
            //it must be a digit
            //looking at the previous and next index we can determine if there were 
            //numbers before this and if this is the last number of the element

            //if the previous element was a number, then we are on a continuos number
            //TODO the number is an integer if the next value is not a . or ,
            //TODO this should be connected with the float check
            next_output_value_num.push(*chr);
            //if the next value is not a digit, then we are on the last digit of the whole number
            if index < infix_as_chars.len() - 1 {
                //if the next element is not a number or a float separator, then we are at the end of a number
                if !infix_as_chars[index + 1].is_digit(10) && !separators.contains(&infix_as_chars[index + 1]){
                    let moved_value = std::mem::take(&mut next_output_value_num);
                    output.push(moved_value);
                    separator_encoutered = false;
                }
                //for other cases like separator or number present, we do nothing more and just continue
                continue;
            }

            //checking if the last character in the vector is equal to the current
            if index == infix_as_chars.len() - 1 {
                let moved_value = std::mem::take(&mut next_output_value_num);
                output.push(moved_value);
            }
        }
        // here we need to check for the (, .) to know if this is a decimal number
        else if separators.contains(&chr){
            //we are at a float separator
            //TODO we need to check that there is only one separator for this number
            /*
             * TODO if we are at a separator and the next element is not a number or a separator,
             * we treat the number as an integer
            */
            //TODO if the previous is not a number, we assume 0 as the number
            
            //if we are at a separator and we have encoutered one previously for the same number,
            //this should return an error
            if separator_encoutered == true{
                return Err(Error::more_than_one_decimal());
            }
            //we are at out first separator is the code passes
            separator_encoutered = true;

            //if there are no numbers on either side of the separator, this should give an error
            if index <= 0 && index < infix_as_chars.len() - 1{
                if index == 0 && !infix_as_chars[index + 1].is_digit(10){
                    return Err(Error::decimal_with_no_numbers());
                }
                else if !infix_as_chars[index + 1].is_digit(10) 
                        && !infix_as_chars[index + 1].is_digit(10){
                    return Err(Error::decimal_with_no_numbers());
                }
            }
            //if there is no number on the left of the separator but there is a number on the right,
            //we assume that the left number is a 0
            if index > 0 && !infix_as_chars[index - 1].is_digit(10) || index == 0{
                next_output_value_num.push('0');
            }
            //we want to support both . and , but , is not supprted so we have to convert it to .
            if *chr == ',' {
                next_output_value_num.push('.');
            }
            else {
                next_output_value_num.push(*chr);
            }

        }
        else if chr.is_alphabetic() {
            //it must be an alphabetical character
            //looking at the previous and next index we can determine if there were 
            //characters before this and if this is the last character of the element

            //if the previous element was a character, then we are on a continuos character
            
            next_output_value_alpha.push(*chr);
            

            //if the next value is not a character, then we are on the last character of the whole string
            if index < infix_as_chars.len() - 1 {
                if !infix_as_chars[index + 1].is_alphabetic() {
                    if functions.contains(&next_output_value_alpha){
                        let moved_value = std::mem::take(&mut next_output_value_alpha);
                        output.push(moved_value);
                    }
                    else{
                        return Err(Error::could_not_parse_function());
                    }
                }
                continue;
            }

            //checking if the last character in the vector is equal to the current
            if index == infix_as_chars.len() - 1 {
                let moved_value = std::mem::take(&mut next_output_value_alpha);
                output.push(moved_value);
            }
        }
        else if math_ops_prio_3.contains(&chr){
            //it must be an operator of prio 3

            loop{
                //if the last value in the stack is of lower prio or if the stack i empty
                //push the value to the stack
                //if the last value is not prio 3 and is not an end parenthesis add the prio 3 operand
                if symbol_stack.len() == 0 || !math_ops_prio_3.contains(&symbol_stack[symbol_stack.len()-1]) 
                                && symbol_stack[symbol_stack.len()-1] != &')'{
                    symbol_stack.push(chr);
                    break;
                }
                else{
                    //pop the last symbol and add it to the output
                    output.push(symbol_stack[symbol_stack.len()-1].to_string());
                    symbol_stack.pop();
                }
            }
        }
        else if math_ops_prio_2.contains(&chr){
            //it must be an operator of prio 2
            
            loop{
                //if the last value in the stack is of lower prio or if the stack i empty
                //push the value to the stack
                //if the last value is not prio 2 and is not an end parenthesis add the prio 2 operand
                if symbol_stack.len() == 0 || ![&math_ops_prio_2[..],&math_ops_prio_3[..]].concat().contains(&symbol_stack[symbol_stack.len()-1]) 
                                && symbol_stack[symbol_stack.len()-1] != &')'{
                    symbol_stack.push(chr);
                    break;
                }
                else{
                    //pop the last symbol and add it to the output
                    output.push(symbol_stack[symbol_stack.len()-1].to_string());
                    symbol_stack.pop();
                }
            }
        }
        else if math_ops_prio_1.contains(&chr){
            //it must be and operator of prio 1

            loop{
                //if the last value in the stack is a start parenthesis or 
                //if the stack is empty push the prio 1 operator to the stack
                //else pop all symbols that are higher priority to the output
                if symbol_stack.len() == 0 || symbol_stack[symbol_stack.len()-1] == &'(' {
                    symbol_stack.push(chr);
                    break;
                }
                else{
                    output.push(symbol_stack[symbol_stack.len()-1].to_string());
                    symbol_stack.pop();
                }
            }
        }
        else if chr == &'('{
            //it is a start parenthesis
            //we always place it on the stack
            symbol_stack.push(chr);
        }
        else if chr == &')'{
            //it is a end parenthesis

            loop{
                //pop symbols to the output until it reaches an start parenthesis
                if symbol_stack[symbol_stack.len()-1] != &'(' {
                    output.push(symbol_stack[symbol_stack.len()-1].to_string());
                    symbol_stack.pop();
                }
                else{
                    symbol_stack.pop();
                    break;
                }
            }
        }
    }
    //emptying the stack
    for _ in 0..symbol_stack.len(){
        output.push(symbol_stack[symbol_stack.len()-1].to_string());
        symbol_stack.pop();
    }
    println!("{:?}", output);
    Ok(
        output
    )
}


