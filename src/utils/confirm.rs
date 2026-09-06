use std::io::Write;
use anyhow::Result;

pub fn confirm(message: &str, cancel_msg: &str) -> Result<bool> {
    loop {
        println!("{}", message);
        print!("[y/N]: ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            println!("{}", cancel_msg);
            return Ok(false);
        }

        match input.trim().to_lowercase().as_str() {
            "y" => return Ok(true),
            "n" => {
                println!("{}", cancel_msg);
                return Ok(false);
            }
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'.");
            }
        }
    }
}
