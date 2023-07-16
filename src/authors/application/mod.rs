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

    pub fn get_available_authors(&self) -> Vec<Author> {
        self.repo.all_authors()
    }

    pub fn find_authors(&self, aliases: Vec<&str>) -> Vec<Author> {
        self.repo.find_authors(aliases)
    }
}

#[cfg(test)]
mod test;
