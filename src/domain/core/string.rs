/// Trim start and end of [`String`] without new allocations
pub fn trim_in_place(s: &mut String) {
    s.replace_range(..(s.len() - s.trim_start().len()), "");
    s.truncate(s.trim_end().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_in_place_works() {
        let strings = [" Test", "Test ", " Test "];
        for s in strings {
            let mut string = String::from(s);
            trim_in_place(&mut string);
            assert_eq!(string.len(), 4);
            assert_eq!(string, String::from("Test"));
        }
    }
}
