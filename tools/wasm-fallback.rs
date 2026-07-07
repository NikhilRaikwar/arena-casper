use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args.first().map(String::as_str).unwrap_or_default();

    if program.contains("cp") && args.len() >= 3 {
        let _ = fs::copy(&args[1], &args[2]);
        return;
    }

    if program.contains("wasm-strip") {
        return;
    }

    let mut input: Option<String> = None;
    let mut output: Option<String> = None;
    let mut index = 1;

    while index < args.len() {
        if args[index] == "-o" && index + 1 < args.len() {
            output = Some(args[index + 1].clone());
            index += 2;
            continue;
        }

        if !args[index].starts_with('-') && fs::metadata(&args[index]).is_ok() {
            input = Some(args[index].clone());
        }

        index += 1;
    }

    if let (Some(input), Some(output)) = (input, output) {
        let _ = fs::copy(input, output);
    }
}
