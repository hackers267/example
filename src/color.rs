use std::fmt::{Display, Formatter, Write};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn color_test(){
        let hi = "Hello".red().on_yellow();
        assert_eq!(format!("{}",hi),"\x1B[31;43mHello\x1B[0m");
    }
}

trait Colorize {
    const FG_RED:&'static str="31";
    const BG_YELLOW:&'static str="43";
    fn red(self)->ColorString;
    fn on_yellow(self)->ColorString;
}

#[derive(Default,Eq, PartialEq)]
struct ColorString {
    input: String,
    fgcolor:String,
    bgcolor:String,
}

impl Display for ColorString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("\x1B[").unwrap();
        let mut wrote = false;
        if !self.fgcolor.is_empty() {
            f.write_str(&self.fgcolor).unwrap();
            wrote = true;
        }
        if !self.bgcolor.is_empty() {
            if wrote {
                f.write_str(";").unwrap();
            }
            f.write_str(&self.bgcolor).unwrap();
        }
        f.write_str("m").unwrap();
        f.write_str(&self.input).unwrap();
        f.write_str("\x1b[0m")
    }
}

impl Colorize for &str{
    fn red(self) -> ColorString {
        ColorString{
            input:self.to_string(),
            fgcolor:ColorString::FG_RED.to_string(),
            ..ColorString::default()
        }
    }

    fn on_yellow(self) -> ColorString {
        ColorString {
            input:self.to_string(),
            bgcolor:ColorString::BG_YELLOW.to_string(),
            ..ColorString::default()
        }
    }
}
impl Colorize for ColorString{
    fn red(self) -> Self {
        Self {
            fgcolor:ColorString::FG_RED.to_string(),
            ..self
        }
    }

    fn on_yellow(self) -> Self {
        Self {
            bgcolor:ColorString::BG_YELLOW.to_string(),
            ..self
        }
    }
}