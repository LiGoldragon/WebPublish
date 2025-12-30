use webpublish::{ApplyOutcome, Configuration, StdinBytes};

fn main() {
    let mut args = std::env::args();
    let _binary = args.next();
    if args.next().is_some() {
        eprintln!("Usage: pipe the webpublish Cap'n Proto message to stdin.");
        std::process::exit(2);
    }

    let stdin_bytes = match StdinBytes::from_stdin() {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("stdin error: {error}");
            std::process::exit(1);
        }
    };
    if stdin_bytes.is_empty() {
        eprintln!("Usage: pipe the webpublish Cap'n Proto message to stdin.");
        std::process::exit(2);
    }

    let configuration = match Configuration::from_bytes(stdin_bytes) {
        Ok(configuration) => configuration,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    match ApplyOutcome::from_configuration(configuration) {
        Ok(outcome) => std::process::exit(outcome.exit_code()),
        Err(error) => std::process::exit(error.exit_code()),
    }
}
