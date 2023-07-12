use super::domain::Repository;

pub struct FSRepo {}

impl Repository for FSRepo {
    fn find_authors(aliases: Vec<String>) -> Result<Vec<super::domain::Author>, String> {
        todo!()
    }

    fn all_authors() -> Result<Vec<super::domain::Author>, String> {
        todo!()
    }
}

#[cfg(test)]
mod test;
