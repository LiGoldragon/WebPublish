use webpublish::{ApplyOutcome, Configuration, StdinBytes};

fn main() {
    let mut args = std::env::args();
    let _binary = args.next();
    let command = args.next();
    if command.as_deref() != Some("apply") || args.next().is_some() {
        eprintln!("Usage: webpublish apply");
        std::process::exit(2);
    }

    let stdin_bytes = match StdinBytes::from_stdin() {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("stdin error: {error}");
            std::process::exit(1);
        }
    };

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
