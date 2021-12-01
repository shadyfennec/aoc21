use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use color_eyre::eyre;
use color_eyre::Help;
use eyre::WrapErr;

use aoc_derive::days;

pub mod ui;

days!(1);

pub type Day = (usize, Option<Box<dyn AocDay + Send + Sync>>);

pub fn days() -> Vec<Day> {
    (1..=25).map(|i| (i, get_day(i).ok())).collect()
}

pub trait AocDay {
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn inputs(&self) -> &[&'static str];
    fn run(
        &self,
        input: Vec<String>,
        output: Sender<String>,
        debug: Sender<String>,
    ) -> eyre::Result<()>;

    fn run_timed(
        &self,
        input: String,
        output: Sender<String>,
        debug: Sender<String>,
    ) -> eyre::Result<Duration> {
        let input = BufReader::new(OpenOptions::new().read(true).write(false).open(input)?)
            .lines()
            .filter_map(|s| s.ok())
            .collect();

        let start = Instant::now();

        self.run(input, output, debug)?;

        Ok(start.elapsed())
    }
}

#[derive(Debug)]
pub enum AocError {
    UnimplementedDay,
    NonExistentDay,
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AocError::UnimplementedDay => write!(f, "Day not implemented yet"),
            AocError::NonExistentDay => {
                write!(f, "Non-existent day")
            }
        }
    }
}

impl Error for AocError {}
