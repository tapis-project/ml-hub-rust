// This test is ignored by default, as it requires a specific file structure and may not be suitable for all environments.
#[cfg(test)]
mod compression_test {
    use crate::common::infra::fs::archiver::Archiver;
    use crate::common::presentation::http::v1::dto::Compression;
    use std::{
        fs,
        path::PathBuf,
        io::Write,
    };

    const PATH_FOR_TESTING: &str = "./path_for_testing";
    const FILE_FOR_TESTING: &str = "file_for_testing.txt";
    const TEST_ZIP_FILE: &str = "compression_test_result.zip";

    const TEST_CONTENT: &str = "This is a test file for compression.";

    #[test]#[ignore]
    fn setup_test_file() {

        // create a directory structure for testing
        fs::create_dir_all(PATH_FOR_TESTING).expect("Failed to create");

        // check the test file exists
        // if the file is not found, create a dummy file
        let test_file = PathBuf::from(PATH_FOR_TESTING).join(FILE_FOR_TESTING);
        if !test_file.exists() {
            let mut file = fs::File::create(&test_file).expect("Failed to create test file");
            write!(file, "{}", TEST_CONTENT).expect("Failed to write to test file");
        }
    }

    #[test]#[ignore]
    fn test_zip() {
        // Ensure the test file is set up before zipping
        setup_test_file();

        // if zip file already exists, this will overwrite it
        let source: &PathBuf = &PathBuf::from(PATH_FOR_TESTING);
        let destination: &PathBuf = &PathBuf::from(TEST_ZIP_FILE);
        let compression_option = Some(Compression::Deflated);
        let result = Archiver::zip(&source, &destination, compression_option, None);
        assert!(result.is_ok(), "Zipping failed: {:?}", result.err());
    }

    #[test]#[ignore]
    fn test_unzip() {
        // if zip file already exists, this will overwrite it
        let source: &PathBuf = &PathBuf::from(TEST_ZIP_FILE);
        let destination: &PathBuf = &PathBuf::from(PATH_FOR_TESTING).join("unzipped_test_dir");
        let result = Archiver::unzip(&source, &destination, None);
        let unzipped_file = destination.join(PATH_FOR_TESTING).join(FILE_FOR_TESTING);
        let content = fs::read_to_string(&unzipped_file).expect("Failed to read unzipped file");
        assert!(result.is_ok(), "Unzipping failed: {:?}", result.err());
        assert_eq!(content, TEST_CONTENT, "Unzipped file content does not match expected content");
    }

    #[test]#[ignore]
    fn delete_test_dir() {
        let test_dir = PathBuf::from(PATH_FOR_TESTING);
        if test_dir.exists() {
            fs::remove_dir_all(test_dir).expect("Failed to delete test directory");
        }
    }

    #[test]#[ignore]
    fn delete_test_zip_file() {
        // delete the test zip file
        let test_zip_file = PathBuf::from(TEST_ZIP_FILE);
        if test_zip_file.exists() {
            fs::remove_file(test_zip_file).expect("Failed to delete test zip file");
        }
    }
}