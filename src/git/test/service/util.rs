use crate::{common::fs::wrapper::MockFileLoader, test_utils::dummy_file::DummyFile};

// TODO: rename file and eval if this is needed in other modules
pub fn successful_file_loader() -> MockFileLoader {
	let mut mock_file_loader = MockFileLoader::new();
	mock_file_loader
		.expect_load()
		.returning(|_| Some(Box::new(DummyFile::empty())));
	mock_file_loader
}
