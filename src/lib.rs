use std::env;

pub fn get_home_dir() -> String {
    env::var("HOME").expect("Failed to get the home dir.")
}

pub fn get_adof_dir() -> String {
    let home_dir = get_home_dir();
    format!("{}/{}", home_dir, ".adof")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_home_dir() {
        env::set_var("HOME", "/test/home");

        let home_dir = get_home_dir();
        assert_eq!(home_dir, "/test/home");
    }

    #[test]
    fn test_get_adof_dir() {
        env::set_var("HOME", "/test/home");

        let adof_dir = get_adof_dir();
        assert_eq!(adof_dir, "/test/home/.adof");
    }
}

