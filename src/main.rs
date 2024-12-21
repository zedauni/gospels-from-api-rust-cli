use chrono::{Datelike, Duration, NaiveDate};
use serde_json::Value;
use std::{error::Error, io::Write, process};
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

#[allow(dead_code)]
fn process_month(start_date: &NaiveDate) -> Result<serde_json::Value, Box<dyn Error>> {
    let (current_month_start, current_month_end) = month_range(start_date);
    let total_days = (current_month_end - current_month_start).num_days() + 1;
    let mut all_results = serde_json::Map::new();

    for i in 0..total_days {
        let current_date = current_month_start + Duration::days(i);
        let results = process_day(&current_date)?;
        all_results.insert(
            format_date_for_url(&current_date),
            serde_json::Value::Array(results),
        );

        let progress = ((i + 1) as f64 / total_days as f64) * 100.0;
        print!(
            "\r{}... {:.2}%",
            current_month_start.format("%Y-%m"),
            progress
        );
        std::io::stdout().flush()?;
    }
    println!();
    Ok(serde_json::Value::Object(all_results))
}

#[allow(dead_code)]
fn format_date_for_url(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

#[allow(dead_code)]
fn month_range(date: &NaiveDate) -> (NaiveDate, NaiveDate) {
    let first_day = date.with_day(1).unwrap();
    let last_day = if first_day.month() == 12 {
        first_day
            .with_year(first_day.year() + 1)
            .unwrap()
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
            - Duration::days(1)
    } else {
        first_day
            .with_month(first_day.month() + 1)
            .unwrap()
            .with_day(1)
            .unwrap()
            - Duration::days(1)
    };
    (first_day, last_day)
}

#[allow(dead_code)]
fn process_day(date: &NaiveDate) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
    let url_date = format_date_for_url(date);
    let url = format!("https://api.aelf.org/v1/messes/{}/afrique", url_date);
    let response =
        reqwest::blocking::get(&url).map_err(|e| format!("Erreur de requête à l'API: {}", e))?;
    let json: Value = response
        .json()
        .map_err(|_| "Erreur de réponse de l'API: données non disponibles".to_string())?;

    let lectures: Vec<serde_json::Value> = json
        .get("messes")
        .and_then(|messes| messes.as_array())
        .map(|messes| {
            messes
                .iter()
                .flat_map(|messe| {
                    messe.get("lectures")
                        .and_then(|lectures| lectures.as_array())
                        .unwrap_or(&Vec::new())
                        .iter()
                        .filter(|lecture| lecture.get("type").and_then(|t| t.as_str()) == Some("evangile"))
                        .map(|lecture|{
                            serde_json::json!({
                                "ref": lecture.get("ref").and_then(|v|v.as_str()).unwrap_or(""),
                                "titre": lecture.get("titre").and_then(|v|v.as_str()).unwrap_or(""),
                                "contenu": lecture.get("contenu").and_then(|v|v.as_str()).unwrap_or("")
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(lectures)
}
