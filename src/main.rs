use std::io::Write;

use mini_evm_calculator::calculate;

/// An exampfn main() -> Result<(), String> {
fn main() -> Result<(), String> {
    println!("Execute a calculation on a rough mini EVM calculator 😀");
    println!("⚠ You can only use + * - / ( ) and numeric characters. Enter \"exit\" to exit\n");

    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{}", err).map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str) -> Result<bool, String> {
    match line {
        "exit" => {
            write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            Ok(true)
        }
        _ => {
            let res = calculate(line).map_err(|e| e.to_string())?;
            write!(std::io::stdout(), "result> {} = {}\n", line, res).map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            Ok(false)
        }
    }
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "\nEVM calculator> ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
