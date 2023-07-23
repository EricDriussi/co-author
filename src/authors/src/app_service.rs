use crate::author::{Author, AuthorRepo};

pub struct AuthService<T: AuthorRepo> {
    repo: T,
}

impl<T: AuthorRepo> AuthService<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub fn get_available_authors(&self) -> Vec<Author> {
        self.repo.all_authors()
    }

    pub fn find_authors(&self, aliases: Vec<String>) -> Vec<String> {
        self.repo
            .find_authors(aliases)
            .iter()
            .map(|author| author.signature())
            .collect()
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
