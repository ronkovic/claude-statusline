// CLI argument parsing
// Supports: --show, --schedule, --help

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Show,
    Schedule,
}

pub fn parse_args() -> crate::error::Result<Mode> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--schedule" => Ok(Mode::Schedule),
            "--show" => Ok(Mode::Show),
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                print_help();
                std::process::exit(1);
            }
        }
    } else {
        Ok(Mode::Show)
    }
}

fn print_help() {
    eprintln!("Usage: cc-statusline [--show|--schedule]");
    eprintln!("  --show      Display status (default)");
    eprintln!("  --schedule  Show calendar schedule");
    eprintln!("  --help      Show this help");
}
