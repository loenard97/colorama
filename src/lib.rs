//! # colorama
//!
//! A simple way to colorize the output of your cli application.
//!
//! This crate contains a single trait `Colored` that is implemented for the `String` type.
//! Calling `.color("red")`, `.background("green")` or `.style("bold")`
//! will wrap your string with the corresponding ANSI escape sequence.
//!
//! Different styles can be concatenated together:
//! ```rust
//! use colorama::Colored;
//!
//! let mut s = String::from("colorama");
//!
//! s.color("red").background("green").style("bold");
//!
//! println!("{}", s);
//! assert_eq!(s, "\x1b[1m\x1b[42m\x1b[31mcolorama\x1b[0m\x1b[0m\x1b[0m");
//! ```
//! Unknown color / style names are silently ignored.
//!
//! Note: This package does not check if the program is running inside a terminal or
//! if it is called via pipes. If you want this functionality, check out
//! [termcolor](https://crates.io/crates/termcolor), [colored](https://crates.io/crates/colored)
//! and / or [atty](https://crates.io/crates/atty).
//!

fn map_color(color: &str) -> Option<&str> {
    match color {
        "normal" => Some("\x1b[0m"),
        "black" => Some("\x1b[30m"),
        "red" => Some("\x1b[31m"),
        "green" => Some("\x1b[32m"),
        "yellow" => Some("\x1b[33m"),
        "blue" => Some("\x1b[34m"),
        "magenta" => Some("\x1b[35m"),
        "cyan" => Some("\x1b[36m"),
        "white" => Some("\x1b[37m"),
        "bright black" => Some("\x1b[90m"),
        "bright red" => Some("\x1b[91m"),
        "bright green" => Some("\x1b[92m"),
        "bright yellow" => Some("\x1b[93m"),
        "bright blue" => Some("\x1b[94m"),
        "bright magenta" => Some("\x1b[95m"),
        "bright cyan" => Some("\x1b[96m"),
        "bright white" => Some("\x1b[97m"),
        _ => None,
    }
}

fn map_background(background: &str) -> Option<&str> {
    match background {
        "normal" => Some("\x1b[0m"),
        "black" => Some("\x1b[40m"),
        "red" => Some("\x1b[41m"),
        "green" => Some("\x1b[42m"),
        "yellow" => Some("\x1b[43m"),
        "blue" => Some("\x1b[44m"),
        "magenta" => Some("\x1b[45m"),
        "cyan" => Some("\x1b[46m"),
        "white" => Some("\x1b[47m"),
        "bright black" => Some("\x1b[100m"),
        "bright red" => Some("\x1b[101m"),
        "bright green" => Some("\x1b[102m"),
        "bright yellow" => Some("\x1b[103m"),
        "bright blue" => Some("\x1b[104m"),
        "bright magenta" => Some("\x1b[105m"),
        "bright cyan" => Some("\x1b[106m"),
        "bright white" => Some("\x1b[107m"),
        _ => None,
    }
}

fn map_style(style: &str) -> Option<&str> {
    match style {
        "normal" => Some("\x1b[0m"),
        "bold" => Some("\x1b[1m"),
        "faint" => Some("\x1b[2m"),
        "italic" => Some("\x1b[3m"),
        "underline" => Some("\x1b[4m"),
        _ => None,
    }
}

pub trait Colored {
    /// Display String in a given color.
    /// Possible values are:
    ///
    /// normal, black, red, green, yellow, blue, magenta, cyan, white, bright black, bright red, bright green, bright yellow, bright blue, bright magenta, bright cyan, bright white
    ///
    /// # Example
    /// ```
    /// use colorama::Colored;
    ///
    /// let mut s = String::from("colorama");
    /// s.color("red");
    ///
    /// assert_eq!(s, "\x1b[31mcolorama\x1b[0m");
    /// ```
    fn color(&mut self, color: &str) -> &mut Self;

    /// Display String with a given background color.
    /// Possible values are:
    ///
    /// normal, black, red, green, yellow, blue, magenta, cyan, white, bright black, bright red, bright green, bright yellow, bright blue, bright magenta, bright cyan, bright white
    ///
    /// # Example
    /// ```
    /// use colorama::Colored;
    ///
    /// let mut s = String::from("colorama");
    /// s.background("red");
    ///
    /// assert_eq!(s, "\x1b[41mcolorama\x1b[0m");
    /// ```
    fn background(&mut self, background: &str) -> &mut Self;

    /// Display String in a given style.
    /// Possible values are:
    ///
    /// normal, bold, faint, italic, underline
    ///
    /// # Example
    /// ```
    /// use colorama::Colored;
    ///
    /// let mut s = String::from("colorama");
    /// s.style("underline");
    ///
    /// assert_eq!(s, "\x1b[4mcolorama\x1b[0m");
    /// ```
    fn style(&mut self, style: &str) -> &mut Self;
}

impl Colored for String {
    /// Display String in a given color.
    /// Possible values are:
    ///
    /// normal, black, red, green, yellow, blue, magenta, cyan, white, bright black, bright red, bright green, bright yellow, bright blue, bright magenta, bright cyan, bright white
    ///
    /// # Example
    /// ```
    /// use colorama::Colored;
    ///
    /// let mut s = String::from("colorama");
    /// s.color("red");
    ///
    /// assert_eq!(s, "\x1b[31mcolorama\x1b[0m");
    /// ```
    fn color(&mut self, color: &str) -> &mut Self {
        map_color(color).map(|c| {
            self.insert_str(0, c);
            self.push_str("\x1b[0m");
        });

        self
    }

    /// Display String with a given background color.
    /// Possible values are:
    ///
    /// normal, black, red, green, yellow, blue, magenta, cyan, white, bright black, bright red, bright green, bright yellow, bright blue, bright magenta, bright cyan, bright white
    ///
    /// # Example
    /// ```
    /// use colorama::Colored;
    ///
    /// let mut s = String::from("colorama");
    /// s.background("red");
    ///
    /// assert_eq!(s, "\x1b[41mcolorama\x1b[0m");
    /// ```
    fn background(&mut self, background: &str) -> &mut Self {
        map_background(background).map(|b| {
            self.insert_str(0, b);
            self.push_str("\x1b[0m");
        });

        self
    }

    /// Display String in a given style.
    /// Possible values are:
    ///
    /// normal, bold, faint, italic, underline
    ///
    /// # Example
    /// ```
    /// use colorama::Colored;
    ///
    /// let mut s = String::from("colorama");
    /// s.style("underline");
    ///
    /// assert_eq!(s, "\x1b[4mcolorama\x1b[0m");
    /// ```
    fn style(&mut self, style: &str) -> &mut Self {
        map_style(style).map(|s| {
            self.insert_str(0, s);
            self.push_str("\x1b[0m");
        });

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_match() {
        let mut s = String::from("colorama");
        s.color("unknown");

        assert_eq!(s, "colorama");
    }

    #[test]
    fn color_and_style() {
        let mut s = String::from("colorama");
        s.color("red").style("bold");

        assert_eq!(s, "\x1b[1m\x1b[31mcolorama\x1b[0m\x1b[0m");
    }

    #[test]
    fn color_background_and_style() {
        let mut s = String::from("colorama");
        s.color("red").background("green").style("bold");

        assert_eq!(s, "\x1b[1m\x1b[42m\x1b[31mcolorama\x1b[0m\x1b[0m\x1b[0m");
    }
}
