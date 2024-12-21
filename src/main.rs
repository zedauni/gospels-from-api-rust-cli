use std::{error::Error, process};
use structopt::StructOpt;

#[allow(dead_code)]
#[derive(Debug, StructOpt)]
#[structopt(name = "gospels", about = "Gospels via API")]
struct Opt {
    /// Specific date to process (YYYY-MM-DD)
    #[structopt(
        short = "d",
        long = "day",
        help = "Date spécifique à traiter (YYYY-MM-DD)"
    )]
    day: Option<String>,

    /// Month to process (YYYY-MM)
    #[structopt(short = "m", long = "month", help = "Mois à traiter (YYYY-MM)")]
    month: Option<String>,

    /// Range of months to process (start_month end_month)
    #[structopt(
        short = "r",
        long = "range",
        help = "Plage de mois à traiter (YYYY-MM YYYY-MM)",
        required = false
    )]
    range: Option<Vec<String>>,
}

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    Opt::clap().print_help()?;
    println!();
    process::exit(1);
}
