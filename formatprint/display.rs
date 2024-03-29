/*
 * Customizing display
 */

use std::fmt;

// Define a structure which fmt::Display will be implemented for
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("{}", Structure(3));
}
