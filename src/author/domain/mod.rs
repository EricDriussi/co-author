use colored::Colorize;
use std::fmt::Display;

#[derive(Debug)]
pub struct Author {
    alias: String,
    name: String,
    email: String,
}

impl Author {
    pub fn new(alias: &str, name: &str, email: &str) -> Self {
        Self {
            alias: String::from(alias),
            name: String::from(name),
            email: String::from(email),
        }
    }

    fn signature(&self) -> String {
        return String::from(format!("Co-Authored by: {} <{}>", self.name, self.email));
    }
}

impl PartialEq for Author {
    fn eq(&self, other: &Self) -> bool {
        let same_alias = self.alias == other.alias;
        let same_name = self.name == other.name;
        let same_email = self.email == other.email;
        return same_alias && same_name && same_email;
    }
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            "⦔".yellow(),
            self.alias.blue(),
            "->".green(),
            self.name
        )
    }
}

pub trait Repository {
    fn find_authors(&self, aliases: Vec<&str>) -> Vec<Author>;
    fn all_authors(&self) -> Vec<Author>;
}

#[cfg(test)]
mod test;
