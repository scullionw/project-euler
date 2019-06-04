#![feature(test)]

use benchtest::*;
use std::iter;
use std::mem;

fn solve() -> usize {
    Date::new()
        .iter()
        .filter(|d| d.year >= 1901 && d.month >= Month::January && d.day >= 1)
        .take_while(|d| d.year <= 2000 && d.month <= Month::December && d.day <= 31)
        .filter(|d| d.dotw == Weekday::Sunday && d.day == 1)
        .count()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Date {
    year: u32,
    month: Month,
    day: u32,
    dotw: Weekday,
}

impl Date {
    fn new() -> Self {
        Self {
            year: 1900,
            month: Month::January,
            day: 1,
            dotw: Weekday::Monday,
        }
    }
    fn iter(mut self) -> impl Iterator<Item = Date> {
        iter::repeat_with(move || {
            let (day, month) = if self.day + 1 > self.month.days(self.year) {
                (1, self.month.next())
            } else {
                (self.day + 1, self.month)
            };

            let dotw = self.dotw.next();

            let year = if self.month == Month::December && self.day == 31 {
                self.year + 1
            } else {
                self.year
            };

            let next = Date {
                year,
                month,
                day,
                dotw,
            };

            mem::replace(&mut self, next)
        })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    fn next(&self) -> Weekday {
        use Weekday::*;
        match self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    fn next(&self) -> Month {
        use Month::*;
        match self {
            January => February,
            February => March,
            March => April,
            April => May,
            May => June,
            June => July,
            July => August,
            August => September,
            September => October,
            October => November,
            November => December,
            December => January,
        }
    }

    fn days(&self, year: u32) -> u32 {
        use Month::*;
        match self {
            January => 31,
            February => {
                if leap_year(year) {
                    29
                } else {
                    28
                }
            }
            March => 31,
            April => 30,
            May => 31,
            June => 30,
            July => 31,
            August => 31,
            September => 30,
            October => 31,
            November => 30,
            December => 31,
        }
    }
}

fn leap_year(year: u32) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}

fn main() {
    println!("{:?}", solve());
}

benchtest! {
    problem_solve: solve() => 171
}
