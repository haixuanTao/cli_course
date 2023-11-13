use bat::{Input, PrettyPrinter};
use colored::Colorize;
use eyre::{Result, WrapErr};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use inquire::Confirm;
use rand::Rng;
use std::time::Duration;
use std::{env, thread};

use clap::Parser;
use std::{
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Flag to skip and go to clap section
    #[arg(long, default_value_t = false)]
    go_to_inquire: bool,
    /// Flag to skip and go to clap section
    #[arg(long, default_value_t = false)]
    go_to_termcolor: bool,
}

fn text(s: &str, confirmation: &str) -> Result<()> {
    println!("");
    let _ = Confirm::new(s).with_default(true).prompt()?;
    println!("");
    println!("{}", confirmation);
    println!("");
    Ok(())
}

fn clear() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn next_project() {
    clear();
    println!("{}", "Next cool project is:".bold());
    println!("");
}
fn main() -> Result<()> {
    clear();
    let args = Args::parse();
    if args.go_to_inquire != true && args.go_to_termcolor != true {
        println!("{}", "This is our dora-course on on CLI! AS A CLI!".bold());

        text("Are you ready to get started?", "🎉 Nice!")?;
        clear();

        println!("{}", "This course will have several parts:".bold());
        println!("");
        println!("  1. Why do we need CLI");
        println!("  2. Cool dependencies for CLI in Rust");
        println!("  3. Not sure yet");
        println!("  4. Q&A");

        text("Does that sounds right?", "🔥 Alright!")?;
        clear();

        println!("{}", "Introduction".bold());
        println!("");
        println!("CLI for Command Line Interface are very nice because");
        println!("It's actually the cheapest way to interact with a program!");
        println!("CLI can then easily be automated!");

        text("Nice, huh?", "😊 Yeah!")?;
        clear();

        println!("{}", "Examples of things we can build:".bold());
        println!("");
        println!(
            "  1. Automate Data Wrangling. 
        Check out: https://www.youtube.com/watch?v=sz_dsktIjt4"
        );
        println!("  2. Cookiecutter tools for non-techies. Complete form. Format document. Apply AI/ML to basic tools.
        Check out: https://github.com/cookiecutter/cookiecutter");
        println!("  3. Create awesome client for your own library/server, that users love!");

        text("That's awesome, isn't it?", "😎 Absolutely!")?;
        clear();

        println!(
            "{}",
            "And in Rust there is awesome tools for building CLI, like:".bold()
        );
        println!("");
        println!(
            "  1. Clap-rs. Awesome argument parser!
        Check out: https://docs.rs/clap/latest/clap/"
        );
        match text(
            "In fact you can try to use the `--go-to-inquire` argument and try clap-rs?",
            "",
        ) {
            Ok(_) => return Ok(()),
            Err(_) => println!("Ok so let's go to the next session!"),
        };
    } else if args.go_to_termcolor != true {
        next_project();
        println!(
            "  2. Inquire. interactively ask the user for information via the CLI.

    See: https://github.com/mikaelmello/inquire"
        );
        println!("");
        println!("Example:");
        println!("");
        let options: Vec<&str> = vec![
            "Banana",
            "Apple",
            "Strawberry",
            "Grapes",
            "Lemon",
            "Tangerine",
            "Watermelon",
            "Orange",
            "Pear",
            "Avocado",
            "Pineapple",
        ];
        let ans = inquire::Select::new("What's your favorite fruit?", options.clone())
            .prompt()
            .unwrap();
        println!("I like {} too!!!", ans);
        println!("");

        next_project();
        println!(
            "  3. Eyre. That helps error handling

        See: https://docs.rs/eyre/latest/eyre/
        "
        );

        let _ans = text(
            "You can try to fail the following prompt by ctrl-c or answering no:",
            "",
        )
        .context(
            "This result was catched!
            
            You can go to `--go-to-termcolor` now 😅",
        )?;
    }
    next_project();
    println!(
        "  4. termcolor. Add colors to your CLI prints 🌈
    See: https://docs.rs/termcolor/latest/termcolor/

    or:
    
            Colored. Coloring terminal with simple traits 🎠
    See: https://github.com/colored-rs/colored
    "
    );
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    println!("");
    writeln!(&mut stdout, "Like this!")?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Black)))?;

    text("CLIs do not have to be all black , do they?", "Nope!")?;

    next_project();
    println!(
        "  5. ctrlc. Control ctrl-c signals 

    See: https://docs.rs/ctrlc/latest/ctrlc/"
    );
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("Ctrl-C was pressed! Someone is ready to leave the class 😅")
    })
    .context("Error setting Ctrl-C handler")?;

    println!("Press Ctrl-C...");
    println!("");
    while running.load(Ordering::SeqCst) {}
    text(
        "Do you still want to see the next cool project?",
        "👍 sure!",
    )?;

    next_project();
    println!(
        "  6. Bat. Cat with syntax highlighting
    
    See: https://github.com/sharkdp/bat"
    );

    text("Let's try it out!", "👍 sure!")?;
    PrettyPrinter::new()
        .header(true)
        .grid(true)
        .line_numbers(true)
        .paging_mode(bat::PagingMode::QuitIfOneScreen)
        .input_file(std::file!())
        .print()
        .wrap_err("Something went wrong with viewing log file")?;

    clear();

    text(
        "Now we can also talk about project, we're not using in dora but wished!",
        "👍 sure!",
    )?;

    next_project();
    println!(
        "  7. Indicatif. Progress bar for CLI.
    
    See: https://github.com/console-rs/indicatif"
    );
    text("Let's try it out!", "👍 sure!")?;

    indicatif();

    text("Wow!", "")?;

    next_project();

    println!(
        "  8. ratatui. terminal user interfaces in rust.
    
    See: https://github.com/ratatui-org/ratatui
    "
    );

    text("Got it!", "")?;
    next_project();

    println!(
        "  9. Dioxus Lab. Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust.
    
    See: https://github.com/DioxusLabs/dioxus
    "
    );

    text("Nice!", "")?;
    clear();

    println!("{}", "That is all for today!".bold());
    println!("");
    println!("For homework, please check out t");
    println!("  2. Cool dependencies for CLI in Rust");
    println!("  3. Not sure yet");
    println!("  4. Q&A");
    Ok(())
}

fn indicatif() {
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let n = 100;
    let pb = m.add(ProgressBar::new(n));
    pb.set_style(sty.clone());
    pb.set_message("todo");
    let pb2 = m.add(ProgressBar::new(n));
    pb2.set_style(sty.clone());
    pb2.set_message("finished");

    let pb3 = m.insert_after(&pb2, ProgressBar::new(1024));
    pb3.set_style(sty);

    m.println("starting!").unwrap();

    let mut threads = vec![];

    let m_clone = m.clone();
    let h3 = thread::spawn(move || {
        for i in 0..1024 {
            thread::sleep(Duration::from_millis(2));
            pb3.set_message(format!("item #{}", i + 1));
            pb3.inc(1);
        }
        m_clone.println("pb3 is done!").unwrap();
        pb3.finish_with_message("done");
    });

    for i in 0..n {
        thread::sleep(Duration::from_millis(15));
        if i == n / 3 {
            thread::sleep(Duration::from_secs(1));
        }
        pb.inc(1);
        let m = m.clone();
        let pb2 = pb2.clone();
        threads.push(thread::spawn(move || {
            let spinner = m.add(ProgressBar::new_spinner().with_message(i.to_string()));
            spinner.enable_steady_tick(Duration::from_millis(100));
            thread::sleep(
                rand::thread_rng().gen_range(Duration::from_secs(1)..Duration::from_secs(2)),
            );
            pb2.inc(1);
        }));
    }
    pb.finish_with_message("all jobs started");

    for thread in threads {
        let _ = thread.join();
    }
    let _ = h3.join();
    pb2.finish_with_message("all jobs done");
    m.clear().unwrap();
}
