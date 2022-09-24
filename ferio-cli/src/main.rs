use ansi_term::Colour::{Blue, Green, Purple, Yellow};
use clap::Parser;
use color_eyre::eyre::Result;
use ferio::{get_holidays, HolidayDate};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, name="holiday")]
struct Args {
    /// Optional date to use instead of today's date (format: MMMM_DD)
    /// Example: January_1
    #[clap(value_parser)]
    date: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let date = args.date.map_or(Ok(HolidayDate::Today), |d| d.parse())?;

    let holidays = get_holidays(&date).await?;
    eprintln!(
        "There are {} {} on {}",
        holidays.len(),
        Green.bold().paint("Holidays and observances"),
        Purple.bold().paint(date.get_date())
    );

    eprintln!(
        "{}",
        Blue.paint("-----------------------------------------------------")
    );
    for (index, holiday) in holidays.iter().enumerate() {
        println!(
            "{}. {} <{}>",
            index + 1,
            Yellow.paint(holiday.get_greeting()),
            holiday.wikipedia_url
        );
    }
    eprintln!(
        "{}",
        Blue.paint("-----------------------------------------------------")
    );

    Ok(())
}
