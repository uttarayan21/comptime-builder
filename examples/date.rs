use comptime_builder::*;

impl<Y, M, D> WithField<1, u16, Date> for DateBuilder<Y, M, D> {
    type Output = DateBuilder<Field<1, u16>, M, D>;
    fn with_field(self, value: u16) -> Self::Output {
        DateBuilder {
            year: Field(value),
            month: self.month,
            day: self.day,
        }
    }
}

impl<Y, M, D> WithField<2, u8, Date> for DateBuilder<Y, M, D> {
    type Output = DateBuilder<Y, Field<2, u8>, D>;
    fn with_field(self, value: u8) -> Self::Output {
        DateBuilder {
            year: self.year,
            month: Field(value),
            day: self.day,
        }
    }
}

impl<Y, M, D> WithField<3, u8, Date> for DateBuilder<Y, M, D> {
    type Output = DateBuilder<Y, M, Field<3, u8>>;
    fn with_field(self, value: u8) -> Self::Output {
        DateBuilder {
            year: self.year,
            month: self.month,
            day: Field(value),
        }
    }
}

pub trait WithYear: WithField<1, u16, Date> {
    fn with_year(self, value: u16) -> Self::Output {
        self.with_field(value)
    }
}

impl<T> WithYear for T where T: WithField<1, u16, Date> {}

pub trait WithMonth: WithField<2, u8, Date> {
    fn with_month(self, value: u8) -> Self::Output {
        self.with_field(value)
    }
}

impl<T> WithMonth for T where T: WithField<2, u8, Date> {}

pub trait WithDay: WithField<3, u8, Date> {
    fn with_day(self, value: u8) -> Self::Output {
        self.with_field(value)
    }
}

impl<T> WithDay for T where T: WithField<3, u8, Date> {}

pub struct DateBuilder<Year, Month, Day> {
    year: Year,
    month: Month,
    day: Day,
}

impl DateBuilder<Field<1, u16>, Field<2, u8>, Field<3, u8>> {
    fn build(self) -> Date {
        Date {
            year: self.year.0,
            month: self.month.0,
            day: self.day.0,
        }
    }
}

#[derive(Debug)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}
pub struct Empty;

impl Date {
    pub fn builder() -> DateBuilder<Empty, Empty, Empty> {
        DateBuilder {
            year: Empty,
            month: Empty,
            day: Empty,
        }
    }
}

pub fn main() {
    let date = Date::builder()
        .with_year(2022)
        .with_month(5)
        .with_day(1)
        .build();
    dbg!(date);
}
