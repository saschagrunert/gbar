use clap::{crate_version, load_yaml, App};
use failure::Fallible;
use gbar::Bar;
use log::LevelFilter;

fn main() -> Fallible<()> {
    // Load the CLI parameters from YAML
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    let level_filter = match matches.occurrences_of("verbose") {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        4 | _ => LevelFilter::Trace,
    };

    // Init the bar
    let mut gbar = Bar::new(level_filter)?;

    // Run the bar
    gbar.run()?;

    Ok(())
}
