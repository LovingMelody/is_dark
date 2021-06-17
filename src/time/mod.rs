use chrono::{Local, NaiveDateTime, Utc};

use crate::IsItDark;

mod suntime;

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Dark,
    Light,
    Auto,
}

#[derive(Clone, Copy, Debug)]
pub struct SmartTime {
    light: NaiveDateTime,
    dark: NaiveDateTime,
    longitude: f64,
    latitude: f64,
    elevation: f64,
    mode: Mode,
}

impl SmartTime {
    pub fn new(longitude: f64, latitude: f64, elevation: f64) -> Self {
        let st = suntime::SunTime::calculate(Utc::today(), latitude, longitude, elevation);
        Self {
            light: st.rise.naive_local(),
            dark: st.set.naive_local(),
            longitude,
            latitude,
            elevation,
            mode: Mode::Auto,
        }
    }
    pub fn set_time(light: NaiveDateTime, dark: NaiveDateTime) -> Self {
        Self {
            light,
            dark,
            longitude: 0.0,
            latitude: 0.0,
            elevation: 0.0,
            mode: Mode::Auto,
        }
    }
    pub fn is_smart(&self) -> bool {
        // In the unlikely event that you are at perfect 0
        // move over a little bit thank you :)
        self.longitude > 0.0 || self.latitude > 0.0 || self.elevation > 0.0
    }
    pub fn recalc(&self) -> Self {
        Self::new(self.longitude, self.latitude, self.elevation)
    }
}

impl IsItDark for SmartTime {
    type Error = std::io::Error;

    fn is_dark(&self) -> Result<bool, Self::Error>
    where
        Self::Error: std::error::Error,
    {
        if matches!(self.mode, Mode::Auto) {
            return Ok(matches!(self.mode, Mode::Dark));
        }
        if Local::now().naive_local() > self.light && Local::now().naive_local() < self.dark {
            return Ok(false);
        }
        let t = if self.is_smart() {
            self.recalc()
        } else {
            *self
        };
        Ok(Local::now().naive_local() > t.dark && Local::now().naive_local() < t.light)
    }

    fn set_light(&mut self) -> Result<bool, Self::Error>
    where
        Self::Error: std::error::Error,
    {
        self.mode = Mode::Light;
        Ok(true)
    }

    fn set_dark(&mut self) -> Result<bool, Self::Error>
    where
        Self::Error: std::error::Error,
    {
        self.mode = Mode::Dark;
        Ok(true)
    }
}

impl Default for SmartTime {
    fn default() -> Self {
        Self {
            light: NaiveDateTime::parse_from_str("08:00:00", "%H:%M:%S").unwrap(),
            dark: NaiveDateTime::parse_from_str("08:00:00", "%H:%M:%S").unwrap(),
            longitude: Default::default(),
            latitude: Default::default(),
            elevation: Default::default(),
            mode: Mode::Auto,
        }
    }
}
