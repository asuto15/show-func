use std::env;
use std::process::Command;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} -i <path-to-binary>", args[0]);
        std::process::exit(1);
    }

    if args[1] != "-i" {
        eprintln!("Unknown option: {}", args[1]);
        std::process::exit(1);
    }

    let which_output = Command::new("which")
        .arg("objdump")
        .output()?;

    if !which_output.status.success() {
        eprintln!("objdump not found");
        std::process::exit(1);
    }

    let output = Command::new("objdump")
        .arg("-d")
        .arg(&args[2])
        .output()?;

    if !output.status.success() {
        let error_str = String::from_utf8_lossy(&output.stderr);
        eprintln!("objdump failed: {}", error_str);
        std::process::exit(1);
    }

    let output_str = String::from_utf8_lossy(&output.stdout);

    if args.contains(&"-i".to_string()) {
        analyze_objdump(&output_str)?;
    }

    Ok(())
}

fn analyze_objdump(output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut functions = Vec::new();
    let mut current_function: Option<String> = None;
    let function_regex = Regex::new(r"([0-9a-f]+) <(.+?)>:")?;
    let mut instruction_count = 0;

    for line in output.lines() {
        if let Some(captures) = function_regex.captures(line) {
            if let Some(func) = current_function.replace(captures[2].to_string()) {
                functions.push((func, instruction_count));
            }
            instruction_count = 0;
            continue;
        }

        if line.trim().is_empty() || line.contains(">:") || line.starts_with("Disassembly") {
            continue;
        }
        if line.contains(':') && line.split_whitespace().nth(1).is_some() {
            instruction_count += 1;
        }
    }

    if let Some(func) = current_function {
        functions.push((func, instruction_count));
    }

    for (function, count) in functions {
        println!("{}:\n\t{} instructions", function, count);
    }

    Ok(())
}