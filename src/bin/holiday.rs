use ansi_term::Colour::{Green, Purple, Yellow};
use color_eyre::eyre::Result;
use ferio::{get_holidays, HolidayDate};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let date = HolidayDate::Today;
    let holidays = get_holidays(&date).await?;
    eprintln!(
        "There are {} {} on {}",
        holidays.len(),
        Green.bold().paint("Holidays and observances"),
        Purple.bold().paint(date.get_date())
    );

    eprintln!("\n-----------------------------------------------------");

    for (index, holiday) in holidays.iter().enumerate() {
        println!(
            "{}. {} <{}>",
            index + 1,
            Yellow.paint(holiday.get_greeting()),
            holiday.wikipedia_url
        );
    }

    eprintln!("-----------------------------------------------------\n");

    Ok(())
}
