//! {{description}}

use clap::Parser;
use color_eyre::eyre::Result;

/// {{description}}
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Who to greet
    #[arg(default_value = "world")]
    name: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    println!("Hello, {}!", cli.name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }
}
