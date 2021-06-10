//error handling

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
//enum
enum ErrorType {
    Parser,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorType,
}

impl Error {
    fn could_not_parse_function() -> Self {
        Self {
            kind: ErrorType::Parser,
        }
    }
}


//this should contain infix notation(3+4), reverse polish notation, ..
pub struct OperationInfo {
    pub infix: Vec<char>,
    pub reverse_pn: Vec<String>,
}

impl OperationInfo {
    //initializes the structure and fills out the parameters
    pub fn new(infix_input: &String) -> Result<Self>{

        let mut infix_vector: Vec<char> = vec![];

        for chr in infix_input.chars(){
            infix_vector.push(chr);
        }

        let op_info = OperationInfo {
            infix: infix_vector,
            reverse_pn: vec![],
        };
        
        Ok(
            op_info
        )
    }
    
    pub fn reverse_polish_parsing(&mut self)->Result<()>{
        //This is using the shunting yard algorithm
        let math_ops_prio_1: Vec<char> = vec!['+', '-'];
        let math_ops_prio_2: Vec<char> = vec!['*', '/', '%'];
        let math_ops_prio_3: Vec<char> = vec!['^'];
        //this may require word recognition
        let functions: Vec<String> = vec!["sqrt".to_string(), "max".to_string(), "min".to_string()];

        let mut symbol_stack: Vec<&char> = vec![];
        let mut output: Vec<String> = vec![];
        let mut next_output_value_num = "".to_string();
        let mut next_output_value_alpha = "".to_string();
        

        //TODO this algorithm needs revision to optimize
        for (index, chr) in self.infix.iter().enumerate() {
            if chr.is_digit(10) {
                //i must be a digit
                //println!("{} is a digit", chr);
                //looking at the previous and next index we can determine if there were 
                //numbers before this and if this is the last number of the element

                //if the previous element was a number, then we are on a continuos number
                
                if index > 0 && self.infix[index - 1].is_digit(10){
                    next_output_value_num = next_output_value_num + &chr.to_string();
                }
                else {
                    next_output_value_num = chr.to_string();
                }
                //if the next value is not a digit, then we are on the last digit of the whole number
                if index < self.infix.len() - 1 {
                    if !self.infix[index + 1].is_digit(10){
                        output.push(next_output_value_num.to_string());
                    }
                    continue;
                }

                //checking if the last character in the vector is equal to the current
                if index == self.infix.len() - 1 {//self.infix.chars().last().unwrap(){
                    output.push(next_output_value_num.to_string());
                }
            }
            else if chr.is_alphabetic() {
                //i must be a digit
                //println!("{} is a digit", chr);
                //looking at the previous and next index we can determine if there were 
                //numbers before this and if this is the last number of the element

                //if the previous element was a number, then we are on a continuos number
                
                if index > 0 && self.infix[index - 1].is_alphabetic(){
                    next_output_value_alpha = next_output_value_alpha + &chr.to_string();
                }
                else {
                    next_output_value_alpha = chr.to_string();
                }
                

                //if the next value is not a digit, then we are on the last digit of the whole number
                if index < self.infix.len() - 1 {
                    //println!("{:?}", self.infix[index + 1]);
                    if !self.infix[index + 1].is_alphabetic() {
                        if functions.contains(&next_output_value_alpha){
                            output.push(next_output_value_alpha.to_string());
                        }
                        else{
                            return Err(Error::could_not_parse_function());
                        }
                    }
                    continue;
                }

                //checking if the last character in the vector is equal to the current
                if index == self.infix.len() - 1 {//self.infix.chars().last().unwrap(){
                    output.push(next_output_value_alpha.to_string());
                }
            }
            else if math_ops_prio_3.contains(&chr){
                //i must be an operator of prio 3
                //println!("{} is a prio 3 operator", chr);
                loop{
                    //if the last value in the stack is of lower prio or if the stack i empty
                    //push the value to the stack
                    //if the last value is not prio 3 and is not an end parenthesis add the prio 3 operand
                    if !math_ops_prio_3.contains(&symbol_stack[symbol_stack.len()-1]) 
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
                //i must be an operator of prio 2
                //println!("{} is a prio 2 operator", chr);
                loop{
                    //if the last value in the stack is of lower prio or if the stack i empty
                    //push the value to the stack
                    //if the last value is not prio 2 and is not an end parenthesis add the prio 2 operand
                    if ![&math_ops_prio_2[..],&math_ops_prio_3[..]].concat().contains(&symbol_stack[symbol_stack.len()-1]) 
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
                //i must be and operator of prio 1
                //println!("{} is a prio 1 operator", chr);

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
                //i is a start parenthesis
                //println!("{} is a start parenthesis", chr);

                //we always place it on the stack
                symbol_stack.push(chr);
            }
            else if chr == &')'{
                //i is a end parenthesis
                //println!("{} is an end parenthesis", chr);

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
        
        //for debugging\
        //println!("{:?}", symbol_stack);
        //println!("{:?}", output);
        //println!("{:?}", self.infix);

        self.reverse_pn = output;

        Ok(())
    }
}

