mod schema;

use chrono::{Datelike, Local};
use schema::{holidays_schema::HolidayRoot, sections_schema::SectionsRoot};
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Debug)]
pub struct Holiday {
    pub name: String,
    pub wikipedia_url: String,
}

impl Holiday {
    pub fn get_greeting(&self) -> String {
        let contains_day = self.name.to_lowercase().contains("day");
        if contains_day {
            format!("Happy {}", self.name)
        } else {
            format!("Happy {} Day", self.name)
        }
    }
}

#[derive(Error, Debug)]
pub enum HolidayErrors {
    #[error("connection error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("wikipedia page for {0} doesn't have holidays.")]
    NoHolidaysFound(String),
}

#[derive(Debug)]
pub enum HolidayDate {
    Today,
    ManualDate { month: u32, day: u32 },
}

impl HolidayDate {
    pub fn get_date(&self) -> String {
        match self {
            HolidayDate::Today => {
                let today = Local::today();
                let month = get_month(today.month());
                format!("{}_{}", month, today.day())
            }
            HolidayDate::ManualDate { month, day } => {
                let month = get_month(*month);
                format!("{}_{}", month, day)
            }
        }
    }
}

fn get_month(m: u32) -> &'static str {
    match m {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => panic!("Invalid month"),
    }
}

pub async fn get_holidays(date: &HolidayDate) -> Result<Vec<Holiday>, HolidayErrors> {
    let resp = reqwest::get(format!(
        "https://en.wikipedia.org/w/api.php/?action=parse&format=json&prop=sections&page={}",
        date.get_date()
    ))
    .await?
    .json::<SectionsRoot>()
    .await?;

    let section = resp
        .parse
        .sections
        .iter()
        .find(|section| section.line == "Holidays and observances")
        .ok_or(HolidayErrors::NoHolidaysFound(date.get_date()))?;

    let resp = reqwest::get(
        format!("https://en.wikipedia.org/w/api.php/?action=parse&format=json&prop=text&disableeditsection=1&page={}&section={}", 
            date.get_date(), section.index)).await?
    .json::<HolidayRoot>().await?;

    let document = Html::parse_document(&resp.parse.text.field);
    let selector = Selector::parse("li a:nth-child(1)").unwrap();
    let holidays = document
        .select(&selector)
        .filter(|e| e.inner_html() != "feast day")
        .filter(|e| {
            e.value()
                .attr("href")
                .map(|h| h.starts_with("/wiki/") && h != "/wiki/Feast_day")
                .unwrap_or(false)
        })
        .map(|e| {
            let name = e.text().fold(String::new(), |mut acc, el| {
                acc.push_str(el);
                acc
            });
            let href = e
                .value()
                .attr("href")
                .map(|url| format!("https://en.wikipedia.org{}", url))
                .unwrap_or(format!(
                    "https://en.wikipedia.org/w/index.php?search={name}"
                ));

            Holiday {
                name,
                wikipedia_url: href,
            }
        })
        .collect::<Vec<_>>();

    Ok(holidays)
}
