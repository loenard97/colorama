# colorama

A simple way to colorize the output of your cli application.

This crate contains a single trait `Colored` that is implemented for the `String` type.
Calling `.color("red")`, `.background("green")` or `.style("bold")` 
will wrap your string with the corresponding ANSI escape sequence.

Different styles can be concatenated together:
```
String::from("Hello World").color("red").background("green").style("bold")
```
Unknown color / style names are silently ignored.

Note: This package does not check if the program is running inside a terminal or
if it is called via pipes. If you want this functionality, check out 
[termcolor](https://crates.io/crates/termcolor), [colored](https://crates.io/crates/colored) 
and / or [atty](https://crates.io/crates/atty).
