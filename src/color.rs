use std::fmt::{Display, Formatter};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn hi_red_on_yellow() {
        let hi = "Hello".red().on_yellow();
        assert_eq!(format!("{}", hi), "\x1B[31;43mHello\x1B[0m");
    }
    #[test]
    fn hi_yellow_on_red() {
        let hi = "Hello".yellow().on_red();
        assert_eq!(format!("{}", hi), "\x1B[33;41mHello\x1B[0m");
    }
}

#[derive(Eq, PartialEq)]
enum Color {
    Red,
    Yellow,
}

impl Color {
    fn to_fg_color(&self) -> &str {
        match self {
            Color::Red => "31",
            Color::Yellow => "33",
        }
    }
    fn to_bg_color(&self) -> &str {
        match self {
            Color::Red => "41",
            Color::Yellow => "43",
        }
    }
}

trait Colorize {
    const FG_RED: &'static str = "31";
    const FG_YELLOW: &'static str = "33";
    const BG_RED: &'static str = "41";
    const BG_YELLOW: &'static str = "43";
    fn red(self) -> ColorString;
    fn yellow(self) -> ColorString;
    fn color(self, color: Color) -> ColorString;
    fn on_yellow(self) -> ColorString;
    fn on_red(self) -> ColorString;
    fn on_color(self, color: Color) -> ColorString;
}

#[derive(Default, Eq, PartialEq)]
struct ColorString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
}

impl Display for ColorString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("\x1B[").unwrap();
        let mut wrote = false;
        if let Some(color) = &self.fgcolor {
            f.write_str(color.to_fg_color()).unwrap();
            wrote = true;
        }
        if let Some(color) = &self.bgcolor {
            if wrote {
                f.write_str(";").unwrap();
            }
            f.write_str(color.to_bg_color()).unwrap();
        }
        f.write_str("m").unwrap();
        f.write_str(&self.input).unwrap();
        f.write_str("\x1b[0m")
    }
}

impl Colorize for &str {
    fn red(self) -> ColorString {
        self.color(Color::Red)
    }

    fn yellow(self) -> ColorString {
        self.color(Color::Yellow)
    }

    fn color(self, color: Color) -> ColorString {
        ColorString {
            input: self.to_string(),
            fgcolor: Some(color),
            ..ColorString::default()
        }
    }

    fn on_yellow(self) -> ColorString {
        self.on_color(Color::Yellow)
    }

    fn on_red(self) -> ColorString {
        self.on_color(Color::Red)
    }

    fn on_color(self, color: Color) -> ColorString {
        ColorString {
            input: self.to_string(),
            bgcolor: Some(color),
            ..ColorString::default()
        }
    }
}
impl Colorize for ColorString {
    fn red(self) -> Self {
        self.color(Color::Red)
    }

    fn yellow(self) -> Self {
        self.color(Color::Yellow)
    }

    fn color(self, color: Color) -> Self {
        Self {
            fgcolor: Some(color),
            ..self
        }
    }

    fn on_yellow(self) -> Self {
        self.on_color(Color::Yellow)
    }

    fn on_red(self) -> Self {
        self.on_color(Color::Red)
    }

    fn on_color(self, color: Color) -> Self {
        Self {
            bgcolor: Some(color),
            ..self
        }
    }
}
