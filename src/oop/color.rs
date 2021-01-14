#[derive(Debug, Clone)]
pub struct ColorString {
    input: String,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
}

#[derive(Debug, Clone)]
pub enum Color {
    RED,
    YELLOW,
    BLUE,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let src= s.to_lowercase();
        match src.as_ref() {
            "red" => Ok(Color::RED),
            "blue" => Ok(Color::BLUE),
            "yellow" => Ok(Color::YELLOW),
            _ => Err(()),
        }
    }
}

impl<'a> From<&'a str> for Color {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::RED)
    }
}
impl From<String> for Color {
    fn from(src: String) -> Self {
        src.parse().unwrap_or(Color::RED)
    }
}



impl Color {
    fn to_fg(&self) -> &str {
        match *self {
            Color::BLUE => "34",
            Color::RED => "31",
            Color::YELLOW => "33"
        }
    }

    fn to_bg(&self) -> &str {
        match *self {
            Color::BLUE => "44",
            Color::RED => "41",
            Color::YELLOW => "43"
        }
    }
}



pub trait Colorized {
    fn red(self) -> ColorString;
    fn blue(self) -> ColorString;
    fn yellow(self) -> ColorString;
    fn color<S: Into<Color>>(self, color: S) -> ColorString;
    fn on_red(self) -> ColorString;
    fn on_blue(self) -> ColorString;
    fn on_yellow(self) -> ColorString;
    fn on_color<S: Into<Color>>(self, color: S) -> ColorString;
}

impl Default for ColorString {
    fn default() -> Self{
        ColorString {
            input: String::default(),
            fg_color: None,
            bg_color: None,
        }
    }
}

impl Colorized for ColorString {
    fn red(self) -> ColorString {
        self.color(Color::RED)
    }

    fn blue(self) -> ColorString {
        self.color(Color::BLUE)
    }

    fn yellow(self) -> ColorString {
        self.color(Color::YELLOW)
    }

    fn color<S: Into<Color>>(self, color: S) -> ColorString {
        ColorString {
            fg_color: Some(color.into()),
            ..self
        }
    }

    fn on_red(self) -> ColorString {
        self.on_color(Color::RED)

    }

    fn on_blue(self) -> ColorString {
        self.on_color(Color::BLUE)
    }

    fn on_yellow(self) -> ColorString {
       self.on_color(Color::YELLOW)
    }

    fn on_color<S: Into<Color>>(self, color: S) -> ColorString {
       ColorString {
           bg_color: Some(color.into()),
           ..self
       }
    }
}


impl<'a> Colorized for &'a str{
    fn red(self) -> ColorString {
        self.color(Color::RED)
    }

    fn blue(self) -> ColorString {
        self.color(Color::BLUE)
    }

    fn yellow(self) -> ColorString {
        self.color(Color::YELLOW)
    }

    fn color<S: Into<Color>>(self, color: S) -> ColorString {
        ColorString {
            fg_color: Some(color.into()),
            input: String::from(self),
            ..ColorString::default()
        }
    }

    fn on_red(self) -> ColorString {
        self.on_color(Color::RED)

    }

    fn on_blue(self) -> ColorString {
        self.on_color(Color::BLUE)
    }

    fn on_yellow(self) -> ColorString {
       self.on_color(Color::YELLOW)
    }

    fn on_color<S: Into<Color>>(self, color: S) -> ColorString {
        ColorString {
            bg_color: Some(color.into()),
            input: String::from(self),
            ..ColorString::default()
        }
    }
}

impl ColorString {
    pub fn compute_style(&self) -> String{
        let mut res = String::from("\x1B[");
        let mut has_wrote = false;
        if let Some(bg) = &self.bg_color {
            if has_wrote {
                res.push(';');
            }
            res.push_str(bg.to_bg());
            has_wrote = true;
        }
        if let Some(fg) = &self.fg_color {
            if has_wrote {
                res.push(';');
                res.push_str(fg.to_fg());
            }
        }
        res.push('m');
        res
    }
}

use std::{fmt::Display, str::FromStr};
impl Display for ColorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let input = &self.input.clone();
        f.write_str(&self.compute_style())?;
        f.write_str(input)?;
        f.write_str("\x1B[0m")?;
        Ok(())
    }
}