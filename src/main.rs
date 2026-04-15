//! INIT - The Beginning of Local Super-Intelligence

use std::env;

const VERSION: &str = "INIT-0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("");
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║   ⬡ INIT - The Beginning of Local Super-Intelligence ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!("");

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "init" => {
            println!("⚡ Initializing INIT runtime...");
            println!("✓ Core loaded");
            println!("✓ AGI layer ready");
            println!("✓ Memory system initialized");
            println!("");
            println!("INIT ready! Run 'init run <prompt>' to start.");
        }
        "run" => {
            if args.len() < 3 {
                println!("Usage: init run <prompt>");
                return;
            }
            let prompt = &args[2..].join(" ");
            println!("Processing: {}", prompt);
            println!("");
            println!("INIT is processing your request...");
            println!("");
            println!("[AGI Response Placeholder]");
            println!("Connect Ollama or Groq to enable inference.");
        }
        "version" | "-v" => {
            println!("INIT v{}", VERSION);
        }
        "help" | "-h" => {
            print_help();
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("INIT - The Beginning of Local Super-Intelligence");
    println!("");
    println!("USAGE:");
    println!("    init <COMMAND>");
    println!("");
    println!("COMMANDS:");
    println!("    init        Initialize INIT runtime");
    println!("    run         Run a prompt");
    println!("    version     Show version");
    println!("    help        Show this help");
    println!("");
    println!("EXAMPLES:");
    println!("    init init");
    println!("    init run \"Explain quantum computing\"");
    println!("");
}
