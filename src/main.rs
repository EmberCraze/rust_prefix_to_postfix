mod lib;
use std::time::Instant;

fn main(){
    let before = Instant::now();
    let value: Vec<String> = match lib::reverse_polish_parsing(&".8+.7+((2^2)*3-1)-(2^2)+2-sqrt(5*7+3)".to_string()){
        Ok(a) => a,
        Err(e) => {println!("Error {:?}", e); return;},
    };
    println!("Elapsed time: {:.2?}", before.elapsed());
    //dbg!(lib::reverse_polish_parsing(&"122.33.3+((2^2)*3-1)-(2^2)+2-sqrt(5*7+3)".to_string()).unwrap());
}