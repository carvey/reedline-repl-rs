//! Minimal example
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl, Result};
use reedline_repl_rs::reedline::ExternalPrinter;

/// Write "Hello" with given name
fn hello<T>(args: ArgMatches, _context: &mut T) -> Result<Option<String>> {
    Ok(Some(format!(
        "Hello, {}",
        args.get_one::<String>("who").unwrap()
    )))
}

fn main() -> Result<()> {

    let printer = ExternalPrinter::default();

    let mut repl = Repl::new(())
        .with_name("MyApp")
        .with_version("v0.1.0")
        .with_description("My very cool app")
        .with_banner("Welcome to MyApp")
        .with_external_printer(printer.clone())
        .with_command(
            Command::new("hello")
                .arg(Arg::new("who").required(true))
                .about("Greetings!"),
            hello,
        );
    
    // pass the printer to another thread
    std::thread::spawn(move || {
        let mut i = 1;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(2));
            printer.print(format!("Hello {}", i)).unwrap();
            i += 1;
        }
    });

    repl.run()
}
