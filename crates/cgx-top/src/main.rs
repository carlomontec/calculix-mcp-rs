use clap::Parser;
use std::path::PathBuf;

/// The terminal companion to CalculiX GraphiX (CGX).
/// Fast, zero-dependency TUI for inspecting FEA results (.frd).
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the CalculiX results file (.frd)
    #[arg(value_name = "FILE")]
    pub file: PathBuf,

    /// Print summary statistics to stdout without launching TUI
    #[arg(short, long)]
    pub summary: bool,

    /// Filter analysis to a specific node or element set (e.g. END_TIP_BLADE)
    #[arg(short = 'S', long, value_name = "SET_NAME")]
    pub set: Option<String>,

    /// Compute peak von Mises stress history across all steps
    #[arg(long)]
    pub mises: bool,

    /// Compute peak principal stresses (sigma_1, sigma_3) across all steps
    #[arg(long)]
    pub principal: bool,

    /// Output query results as comma-separated values (CSV)
    #[arg(long)]
    pub csv: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    println!("============================================================");
    println!("  cgx-top v{} - CalculiX Terminal Inspector", env!("CARGO_PKG_VERSION"));
    println!("============================================================");
    println!("Target file: {}", args.file.display());

    if let Some(set_name) = &args.set {
        println!("Active set filter: [{}]", set_name);
    }

    if args.mises {
        println!("Requested calculation: Peak von Mises stress history");
    }

    if args.summary {
        println!("\n[Summary Mode]");
        println!("(ASCII/Binary FRD parser engine initializing...)");
    } else {
        println!("\n[Interactive TUI Mode]");
        println!("(Ratatui viewport initializing...)");
    }

    Ok(())
}
