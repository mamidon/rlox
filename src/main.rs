
#[allow(dead_code)]
mod debug;

#[allow(dead_code)]
mod scanning;

#[allow(dead_code)]
mod chunks;

#[allow(dead_code)]
mod runtime;

#[allow(dead_code)]
fn main() -> Result<(), i32> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
        println!("hey");
    } else {
       eprintln!("Usage: rlox [path]");
    }
    
    Ok(())
}

fn repl() {
    use std::io::Write;;
    
    loop {
        let mut input = String::new();
        
        print!("> ");
        std::io::stdout().flush().expect("There was a problem writing to console");;
        std::io::stdin()
            .read_line(&mut input)
            .expect("There was a problem reading your input");
        
        if input.len() == 1 {
            break;
        }
        
        interpret(&input);
    }
}

fn run_file(path_name: &str) {
    use std::fs;
    use std::io::Read;
    
    let mut file = fs::File::open(path_name)
        .expect("File not found");
    
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("File contents are not accessible");
    
    interpret(&input);
}

#[allow(unused_variables)]
fn interpret(input: &str) -> runtime::ExecutionResult {
    
    runtime::ExecutionResult::Ok
}

