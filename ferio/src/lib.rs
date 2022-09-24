mod schema;

use chrono::{Datelike, Local};
use schema::{
    holidays_schema::HolidayRoot, image_schema::ImageRoot, sections_schema::SectionsRoot,
};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Holiday {
    pub name: String,
    pub wikipedia_url: String,
}

impl Holiday {
    pub fn get_greeting(&self) -> String {
        let contains_day = self.name.to_lowercase().contains("day");
        let contains_month = self.name.to_lowercase().contains("month");
        if contains_day || contains_month {
            format!("Happy {}", self.name)
        } else {
            format!("Happy {} Day", self.name)
        }
    }
}

#[derive(Error, Debug)]
pub enum HolidayError {
    #[error("Failed to connect to wikipedia")]
    Reqwest(#[from] reqwest::Error),
    #[error("Wikipedia page for {0} doesn't have a holidays section")]
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

#[derive(Error, Debug)]
pub enum HolidayDateError {
    #[error("Invalid date format: {0}")]
    InvalidDate(String),
    #[error("Invalid month: {0}")]
    MonthParseError(String),
    #[error("Invalid day: {0}")]
    DayParseError(#[from] std::num::ParseIntError),
}

impl FromStr for HolidayDate {
    type Err = HolidayDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('_').collect();
        if parts.len() != 2 {
            return Err(Self::Err::InvalidDate(s.to_string()));
        }
        let month = parse_month(parts[0])?;
        let day = parts[1].parse::<u32>()?;

        Ok(HolidayDate::ManualDate { month, day })
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

fn parse_month(s: &str) -> Result<u32, HolidayDateError> {
    match s {
        "January" => Ok(1),
        "February" => Ok(2),
        "March" => Ok(3),
        "April" => Ok(4),
        "May" => Ok(5),
        "June" => Ok(6),
        "July" => Ok(7),
        "August" => Ok(8),
        "September" => Ok(9),
        "October" => Ok(10),
        "November" => Ok(11),
        "December" => Ok(12),
        _ => Err(HolidayDateError::MonthParseError(s.to_string())),
    }
}

pub async fn get_holidays(date: &HolidayDate) -> Result<Vec<Holiday>, HolidayError> {
    let holiday_section_index = get_holidays_section_index(date).await?;

    let resp = reqwest::get(
        format!(
            "https://en.wikipedia.org/w/api.php/?action=parse&format=json&prop=text&disableeditsection=1&page={}&section={}",
            date.get_date(),
            holiday_section_index
        )
    ).await?.json::<HolidayRoot>().await?;

    let document = Html::parse_document(&resp.parse.text.field);
    let selector = Selector::parse("li a:nth-child(1)").unwrap();

    let holidays = document
        .select(&selector)
        .filter(|e| e.inner_html() != "feast day")
        .filter(|e| {
            e.value()
                .attr("href")
                .map(|h| {
                    h.starts_with("/wiki/")
                        && h != "/wiki/Feast_day"
                        && h != "/wiki/Wikipedia:Link_rot"
                })
                .unwrap_or(false)
        })
        .map(|e| {
            let name = e.text().fold(String::new(), |mut acc, el| {
                acc.push_str(el);
                acc
            });

            let wikipedia_url = e
                .value()
                .attr("href")
                .map(|url| format!("https://en.wikipedia.org{}", url))
                .unwrap_or_default();

            Holiday {
                name,
                wikipedia_url,
            }
        })
        .collect::<Vec<_>>();

    Ok(holidays)
}

async fn get_holidays_section_index(date: &HolidayDate) -> Result<String, HolidayError> {
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
        .ok_or_else(|| HolidayError::NoHolidaysFound(date.get_date()))?;

    Ok(section.index.clone())
}

async fn _get_image(wikipedia_url: &str) -> Option<String> {
    let name = wikipedia_url
        .replace("https://en.wikipedia.org/wiki/", "")
        .replace('_', " ");
    let url = format!("https://en.wikipedia.org/w/api.php?action=query&prop=pageimages&format=json&piprop=original&titles={name}");
    let mut resp = reqwest::get(url)
        .await
        .ok()?
        .json::<ImageRoot>()
        .await
        .ok()?;
    let page = resp.query.pages.values_mut().next()?;
    Some(page.original.source.clone())
}
