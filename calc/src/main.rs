use std::io;

fn main() {

    println!("CALCULATOR");
    
    loop{
    println!("Please input two numbers, q for quit:");

    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("failed to read from stdin");
        
    let ex=num1.trim();
    
    if ex=="q" {
    	break
    }

    let num1: u32 = match num1.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to parse num1. Defaulting to 0.");
            0
        }
    };

    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("failed to read from stdin");

    let num2: u32 = match num2.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to parse num2. Defaulting to 0.");
            0
        }
    };
   
    println!("Two numbers:{} {}",num1,num2);
    
    println!("Enter operation to be performed:");
    
    let mut op = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("failed to read from stdin");
        
    let op = op.trim();
        
    if op=="+" {
    	
    	println!("Operation: {}",op);
    	
    	let add= num1+num2;
    	println!("{}{}{}={}",num1,op,num2,add);
    }
    
    else if op=="-" {
    	
    	println!("Operation: {}",op);
    	
    	let sub= num1-num2;
    	println!("{}{}{}={}",num1,op,num2,sub);
    }
    
    if op=="*" {
    	
    	println!("Operation: {}",op);
    	
    	let into= num1*num2;
    	println!("{}{}{}={}",num1,op,num2,into);
    }
    
    if op=="/" {
    	
    	println!("Operation: {}",op);
    	
    	let dev= num1+num2;
    	println!("{}{}{}={}",num1,op,num2,dev);
    }
    
    /*
    	let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" =>num1 / num2
        
        _ => {
            println!("Invalid operation. Exiting.");
            return;
        }
    };

    println!("Result: {}", result);
}
    */
}
}

