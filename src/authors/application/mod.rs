use super::domain::{Author, Repository};

pub struct Service {
    repo: Box<dyn Repository>,
}

impl Service {
    pub fn new(repo: impl Repository + 'static) -> Self {
        Self {
            repo: Box::new(repo),
        }
    }

    fn get_available_authors(&self) -> Vec<Author> {
        self.repo.all_authors()
    }

    pub fn find_authors(&self, aliases: Vec<String>) -> Vec<Author> {
        self.repo.find_authors(aliases)
    }

    pub fn print_available(&self) {
        let authors = self.get_available_authors();
        println!();
        for author in &authors {
            println!("{}", author);
        }
        println!();
    }
}

#[cfg(test)]
mod test;
