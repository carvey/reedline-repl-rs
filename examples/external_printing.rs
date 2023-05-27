//! Minimal example
use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl, Result};

/// Write "Hello" with given name
fn hello<T>(args: ArgMatches, _context: &mut T) -> Result<Option<String>> {
    Ok(Some(format!(
        "Hello, {}",
        args.get_one::<String>("who").unwrap()
    )))
}

fn main() -> Result<()> {
    let mut repl = Repl::new(())
        .with_name("MyApp")
        .with_version("v0.1.0")
        .with_description("My very cool app")
        .with_banner("Welcome to MyApp")
        .with_command(
            Command::new("hello")
                .arg(Arg::new("who").required(true))
                .about("Greetings!"),
            hello,
        );
    
    // get a clone of the external printer
    let printer = repl.get_external_printer();

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
