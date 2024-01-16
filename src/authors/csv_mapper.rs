use super::author::Author;

pub struct CsvMapper {}

impl CsvMapper {
	pub fn to_author(line: &str) -> Option<Author> {
		let fields: Vec<&str> = line.split(',').collect();
		if fields.len() != 3 {
			return None;
		}
		Some(Author::from(fields[0], fields[1], fields[2]))
	}
}
