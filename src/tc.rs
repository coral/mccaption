#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TimeCodeFormat {
    Fps24,
    Fps25,
    Fps30,
    Fps30DropFrame,
    Fps50,
    Fps60,
    Fps60DropFrame,
}

impl TimeCodeFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "24" => Some(TimeCodeFormat::Fps24),
            "25" => Some(TimeCodeFormat::Fps25),
            "30" => Some(TimeCodeFormat::Fps30),
            "30DF" => Some(TimeCodeFormat::Fps30DropFrame),
            "50" => Some(TimeCodeFormat::Fps50),
            "60" => Some(TimeCodeFormat::Fps60),
            "60DF" => Some(TimeCodeFormat::Fps60DropFrame),
            _ => None,
        }
    }
}

impl Default for TimeCodeFormat {
    fn default() -> Self {
        TimeCodeFormat::Fps30
    }
}

impl Into<&str> for TimeCodeFormat {
    fn into(self) -> &'static str {
        match self {
            TimeCodeFormat::Fps24 => "24",
            TimeCodeFormat::Fps25 => "25",
            TimeCodeFormat::Fps30 => "30",
            TimeCodeFormat::Fps30DropFrame => "30DF",
            TimeCodeFormat::Fps50 => "50",
            TimeCodeFormat::Fps60 => "60",
            TimeCodeFormat::Fps60DropFrame => "60DF",
        }
    }
}

impl Into<String> for TimeCodeFormat {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_string() {
        use crate::tc::TimeCodeFormat;
        let rate_str = "30DF";
        assert_eq!(
            TimeCodeFormat::from_str(rate_str),
            Some(TimeCodeFormat::Fps30DropFrame)
        );

        let rate_str = "30DFF";
        assert_eq!(TimeCodeFormat::from_str(rate_str), None);

        let rate_str = "60";
        assert_eq!(
            TimeCodeFormat::from_str(rate_str),
            Some(TimeCodeFormat::Fps60)
        );
    }

    #[test]
    fn to_string() {
        use crate::tc::TimeCodeFormat;
        let s: String = TimeCodeFormat::Fps30DropFrame.into();
        assert_eq!(s, "30DF");

        let vs: &str = TimeCodeFormat::Fps30DropFrame.into();
        assert_eq!(vs, "30DF");
    }
}

pub struct TimeCode {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub frame: u32,
}

impl From<(u32, u32, u32, u32)> for TimeCode {
    fn from((hour, minute, second, frame): (u32, u32, u32, u32)) -> Self {
        TimeCode {
            hour,
            minute,
            second,
            frame,
        }
    }
}
