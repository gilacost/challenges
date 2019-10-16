/// This module contains all challenges that have to do with strings manipulation.
///
/// Reverse takes the str parameter being passed and return the string in reversed
/// order.
/// # Arguments
///
/// * `string` - A string slice that holds the name of the person
///
/// # Examples:
///
/// """
/// use string::reverse::make;
/// make("Hello World") == "dlroW olleH";
/// """

pub fn make(string: String) -> String {
    let reversed = string.chars().rev().collect();
    return reversed;
}

#[cfg(test)]
mod test {
    use crate::string::reverse::make;

    #[test]
    fn reverse_test() {
        assert_eq!(make("Hello world".to_string()), "dlrow olleH");
    }
}
