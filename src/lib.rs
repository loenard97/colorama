//! # colorama
//!
//! A simple way to colorize the output of your cli application.
//! 
//! This crate contains a single trait `Colored` that is implemented for the `String` type.
//! Calling `.color("red")`, `.background("green")` or `.style("bold")` 
//! will wrap your string with the corresponding ANSI escape sequence.
//! 
//! Different styles can be concatenated together:
//! ```
//! String::from("Hello World").color("red").background("green").style("bold")
//! ```
//! Unknown color / style names are silently ignored.
//! 
//! Note: This package does not check if the program is running inside a terminal or
//! if it is called via pipes. If you want this functionality, check out 
//! [termcolor](https://crates.io/crates/termcolor), [colored](https://crates.io/crates/colored) 
//! and / or [atty](https://crates.io/crates/atty).
//! 

fn map_color(color: &str) -> &str {
    match color {
        "normal" => "\x1b[0m",
        "black" => "\x1b[30m",
        "red" => "\x1b[31m",
        "green" => "\x1b[32m",
        "yellow" => "\x1b[33m",
        "blue" => "\x1b[34m",
        "magenta" => "\x1b[35m",
        "cyan" => "\x1b[36m",
        "white" => "\x1b[37m",
        "bright black" => "\x1b[90m",
        "bright red" => "\x1b[91m",
        "bright green" => "\x1b[92m",
        "bright yellow" => "\x1b[93m",
        "bright blue" => "\x1b[94m",
        "bright magenta" => "\x1b[95m",
        "bright cyan" => "\x1b[96m",
        "bright white" => "\x1b[97m",
        _ => "",
    }
}

fn map_background(background: &str) -> &str {
    match background {
        "normal" => "\x1b[0m",
        "black" => "\x1b[48;5;40m",
        "red" => "\x1b[48;5;41m",
        "green" => "\x1b[48;5;42m",
        "yellow" => "\x1b[48;5;43m",
        "blue" => "\x1b[48;5;44m",
        "magenta" => "\x1b[48;5;45m",
        "cyan" => "\x1b[48;5;46m",
        "white" => "\x1b[48;5;47m",
        "bright black" => "\x1b[48;5;100",
        "bright red" => "\x1b[48;5;101m",
        "bright green" => "\x1b[48;5;102m",
        "bright yellow" => "\x1b[48;5;103m",
        "bright blue" => "\x1b[48;5;104m",
        "bright magenta" => "\x1b[48;5;105m",
        "bright cyan" => "\x1b[48;5;106m",
        "bright white" => "\x1b[48;5;107m",
        _ => "",
    }
}

fn map_style(style: &str) -> &str {
    match style {
        "normal" => "\x1b[0m",
        "bold" => "\x1b[1m",
        "faint" => "\x1b[2m",
        "italic" => "\x1b[3m",
        "underline" => "\x1b[4m",
        _ => "",
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
    /// let my_string = String::from("Hello World");
    /// println!("{}", my_string.color("green"));
    /// ```
    fn color(self, color: &str) -> Self;

    /// Display String with a given background color.
    /// Possible values are:
    /// 
    /// normal, black, red, green, yellow, blue, magenta, cyan, white, bright black, bright red, bright green, bright yellow, bright blue, bright magenta, bright cyan, bright white
    /// 
    /// # Example
    /// ```
    /// let my_string = String::from("Hello World");
    /// println!("{}", my_string.background("green"));
    /// ```
    fn background(self, background: &str) -> Self;

    /// Display String in a given style.
    /// Possible values are:
    /// 
    /// normal, bold, faint, italic, underline
    /// 
    /// # Example
    /// ```
    /// let my_string = String::from("Hello World");
    /// println!("{}", my_string.style("bold"));
    /// ```
    fn style(self, style: &str) -> Self;
}

impl Colored for String {
    /// Display String in a given color.
    /// Possible values are:
    /// 
    /// normal, black, red, green, yellow, blue, magenta, cyan, white, bright black, bright red, bright green, bright yellow, bright blue, bright magenta, bright cyan, bright white
    /// 
    /// # Example
    /// ```
    /// let my_string = String::from("Hello World");
    /// println!("{}", my_string.color("green"));
    /// ```
    fn color(self, color: &str) -> Self {
        let mut colored = String::new();
        colored.push_str(map_color(color));
        colored.push_str(&self);
        colored.push_str(map_color("normal"));

        colored
    }

    /// Display String with a given background color.
    /// Possible values are:
    /// 
    /// normal, black, red, green, yellow, blue, magenta, cyan, white, bright black, bright red, bright green, bright yellow, bright blue, bright magenta, bright cyan, bright white
    /// 
    /// # Example
    /// ```
    /// let my_string = String::from("Hello World");
    /// println!("{}", my_string.background("green"));
    /// ```
    fn background(self, background: &str) -> Self {
        let mut colored = String::new();
        colored.push_str(map_background(background));
        colored.push_str(&self);
        colored.push_str(map_background("normal"));

        colored
    }

    /// Display String in a given style.
    /// Possible values are:
    /// 
    /// normal, bold, faint, italic, underline
    /// 
    /// # Example
    /// ```
    /// let my_string = String::from("Hello World");
    /// println!("{}", my_string.style("bold"));
    /// ```
    fn style(self, style: &str) -> Self {
        let mut styled = String::new();
        styled.push_str(map_style(style));
        styled.push_str(&self);
        styled.push_str(map_style("normal"));

        styled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color() {
        assert_eq!(
            String::from("Hello World").color("red"), 
            "\x1b[31mHello World\x1b[0m"
        );
    }

    #[test]
    fn background() {
        assert_eq!(
            String::from("Hello World").background("red"), 
            "\x1b[48;5;41mHello World\x1b[0m"
        );
    }

    #[test]
    fn style() {
        assert_eq!(
            String::from("Hello World").style("underline"), 
            "\x1b[4mHello World\x1b[0m"
        );
    }

    #[test]
    fn no_match() {
        assert_eq!(
            String::from("Hello World").color("unknown"), 
            "Hello World\x1b[0m"
        );
    }

    #[test]
    fn color_and_style() {
        assert_eq!(
            String::from("Hello World").color("red").style("bold"), 
            "\x1b[1m\x1b[31mHello World\x1b[0m\x1b[0m"
        );
    }

    #[test]
    fn color_background_and_style() {
        assert_eq!(
            String::from("Hello World").color("red").background("red").style("bold"), 
            "\x1b[1m\x1b[48;5;41m\x1b[31mHello World\x1b[0m\x1b[0m\x1b[0m"
        );
    }
}
