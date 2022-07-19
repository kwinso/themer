use std::env;

pub fn expand_tilde(p: &String) -> String {
    let mut new = p.clone();

    if new.starts_with("~") {
        new = new.replacen("~", &env::var("HOME").unwrap_or(String::new()), 1);
    }

    new
}
