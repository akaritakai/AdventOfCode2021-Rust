use std::fs;
use std::ops::Not;
use std::path::{PathBuf, Path};
use std::sync::{Arc, RwLock};

use reqwest::StatusCode;

pub struct PuzzleInputFetcher {
    // The base URL for Advent of Code (by default 'https://adventofcode.com/')
    base_url: String,

    // The location where puzzle input is stored (by default 'puzzle')
    input_path: PathBuf,

    // The location where the session token is stored (by default 'cookie.txt')
    session_token_path: PathBuf,

    // Per-day lock for loading the puzzle input into our input cache
    is_input_set: Vec<Arc<RwLock<bool>>>,

    // The input cache that stores our puzzles
    inputs: Vec<String>,

    // Lock for loading the session token into our session token cache
    is_session_token_set: Arc<RwLock<bool>>,

    // The session token cache
    session_token: String,
}

impl PuzzleInputFetcher {
    // Creates a PuzzleInputFetcher using the default values
    pub fn create() -> PuzzleInputFetcher {
        PuzzleInputFetcher::create_custom(
            "https://adventofcode.com",
            Path::new("puzzle"),
            Path::new("cookie.txt"))
    }

    // Creates a PuzzleInputFetcher using the with a specified base url, puzzle input path, and
    // session token path. Used only for testing.
    pub fn create_custom(base_url: &str, input_path: &Path, session_token_path: &Path)
        -> PuzzleInputFetcher {
        let mut is_input_set = Vec::with_capacity(25);
        (0..25).for_each(|_| is_input_set.push(Arc::new(RwLock::new(false))));
        PuzzleInputFetcher {
            base_url: base_url.to_string(),
            input_path: input_path.to_path_buf(),
            is_input_set,
            inputs: vec![String::new(); 25],
            session_token_path: session_token_path.to_path_buf(),
            is_session_token_set: Arc::new(RwLock::new(false)),
            session_token: String::new(),
        }
    }

    // Returns the puzzle input for the given day first by fetching it from the in-memory cache,
    // then by fetching it from the local store, and finally by fetching it from the remote store
    // (the site itself).
    pub fn get_puzzle_input(&mut self, day: u8) -> Result<&str> {
        let index = (day - 1) as usize;
        if self.is_input_set[index].read().unwrap().not() {
            // Puzzle is not in our cache
            let mut is_input_set = self.is_input_set[index].write().unwrap();
            if is_input_set.not() {
                if let Ok(local_input) = self.fetch_local_puzzle_input(day) {
                    // Puzzle is in our local store
                    self.inputs[index].push_str(local_input.as_str());
                    *is_input_set = true;
                    return Ok(self.inputs[index].as_str())
                }
                // Puzzle is not in our local store
                if self.is_session_token_set.read().unwrap().not() {
                    // Session token is not cached
                    let mut is_session_token_set = self.is_session_token_set.write().unwrap();
                    if is_session_token_set.not() {
                        let session_token = self.fetch_session_token()?;
                        self.session_token.push_str(session_token.trim());
                        *is_session_token_set = true;
                    }
                }
                let session_token = self.session_token.as_str();
                let remote_input = self.fetch_remote_puzzle_input(day, session_token)?;
                self.store_puzzle_input_locally(day, remote_input.as_str());
                self.inputs[index].push_str(remote_input.as_str());
                *is_input_set = true;
                return Ok(self.inputs[index].as_str())
            }
        }
        Ok(self.inputs[index].as_str())
    }

    fn fetch_local_puzzle_input(&self, day: u8) -> Result<String> {
        let path = self.input_path.to_path_buf().join(day.to_string());
        fs::read_to_string(path)
            .map_err(|e| format!("Failed to fetch local puzzle for day {}: {}", day, e))
    }

    fn store_puzzle_input_locally(&self, day: u8, input: &str) {
        // Storing puzzle input locally on disk is a 'nice to have' feature in that it reduces load
        // on the Advent of Code site for subsequent runs, but if we can't save to disk -- it
        // shouldn't be a critical error.
        let _ = fs::create_dir_all(self.input_path.to_path_buf());
        let _ = fs::write(self.input_path.join(day.to_string()), input);
    }

    fn fetch_remote_puzzle_input(&self, day: u8, session_token: &str) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let path = format!("{}{}", self.base_url, remote_url_path(day));
        let response = client.get(&path)
            .header("Cookie", format!("session={}", session_token))
            .send()
            .map_err(|e| format!("Failed to fetch remote puzzle input for day {}: {}", day, e))?;
        if response.status() != StatusCode::OK {
            Err(format!("Failed to fetch remote puzzle input for day {}: \
                         Got status code = {}", day, response.status()))
        } else {
            response.text()
                .map_err(|e| format!("Failed to fetch remote puzzle input for day {}: \
                                            Failed to read body as text: {}", day, e))
        }
    }

    // Fetches the session token from the disk
    fn fetch_session_token(&self) -> Result<String> {
        let session_token = fs::read_to_string(self.session_token_path.to_path_buf())
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("Failed to fetch session token from {}: {}",
                                 path_to_str(&self.session_token_path), e))?;
        let has_right_length = session_token.len() == 96;
        let has_right_charset = session_token.chars()
            .all(|x| (x >= '0' && x <= '9') || (x >= 'a' && x <= 'z'));
        if !has_right_length || !has_right_charset {
            Err(format!("Session token is not in the right format. \
                         Expected 96 lowercase hex digits. Got: {}", session_token))
        } else {
            Ok(session_token)
        }
    }
}

fn remote_url_path(day: u8) -> String {
    format!("/2021/day/{}/input", day.to_string())
}

fn path_to_str(path: &Path) -> String {
    path.to_path_buf().into_os_string().into_string().unwrap()
}

type Result<T> = std::result::Result<T, String>;

#[cfg(test)]
mod tests {
    use crate::puzzle_input_fetcher::{PuzzleInputFetcher, remote_url_path};

    use rand::Rng;
    use std::fs::File;
    use std::io::Write;
    use tempfile::{tempdir, NamedTempFile};
    use httpmock::Method::GET;
    use httpmock::MockServer;

    //noinspection DuplicatedCode
    #[test]
    fn test_fetch_from_local_store() {
        let server = MockServer::start();
        let base_url = &server.base_url();
        let puzzle_store_dir = tempdir().unwrap();
        let session_token_path = NamedTempFile::new().unwrap();
        let session_token = random_session_token();
        let mut fetcher = PuzzleInputFetcher::create_custom(
            base_url,
            puzzle_store_dir.path(),
            session_token_path.path());
        for day in 1..26 {
            let puzzle_input = random_puzzle();
            let puzzle_file_path = puzzle_store_dir.path().join(day.to_string());
            let mut puzzle_file = File::create(puzzle_file_path).unwrap();
            puzzle_file.write_all(puzzle_input.as_bytes()).unwrap();
            let mock = server.mock(|when, then| {
                when.method(GET)
                    .path(remote_url_path(day).as_str())
                    .header("Cookie", format!("session={}", session_token).as_str());
                then.status(501);
            });
            assert_eq!(fetcher.get_puzzle_input(day).unwrap(), puzzle_input);
            mock.assert_hits(0);
        }
    }

    //noinspection DuplicatedCode
    #[test]
    fn test_fetch_from_remote_store() {
        let server = MockServer::start();
        let base_url = &server.base_url();
        let puzzle_store_dir = tempdir().unwrap();
        let mut session_token_path = NamedTempFile::new().unwrap();
        let session_token = random_session_token();
        session_token_path.write_all(session_token.as_bytes()).unwrap();
        let mut fetcher = PuzzleInputFetcher::create_custom(
            base_url,
            puzzle_store_dir.path(),
            session_token_path.path());
        for day in 1..26 {
            let puzzle_input = random_puzzle();
            let mock = server.mock(|when, then| {
                when.method(GET)
                    .path(remote_url_path(day).as_str())
                    .header("Cookie", format!("session={}", session_token).as_str());
                then.status(200)
                    .body(&puzzle_input);
            });
            assert_eq!(fetcher.get_puzzle_input(day).unwrap(), puzzle_input);
            mock.assert();
        }
    }

    //noinspection DuplicatedCode
    #[test]
    fn test_error_returned_when_all_sources_unavailable() {
        let server = MockServer::start();
        let base_url = &server.base_url();
        let puzzle_store_dir = tempdir().unwrap();
        let mut session_token_path = NamedTempFile::new().unwrap();
        let session_token = random_session_token();
        session_token_path.write_all(session_token.as_bytes()).unwrap();
        let mut fetcher = PuzzleInputFetcher::create_custom(
            base_url,
            puzzle_store_dir.path(),
            session_token_path.path());
        for day in 1..26 {
            let mock = server.mock(|when, then| {
                when.method(GET)
                    .path(remote_url_path(day).as_str())
                    .header("Cookie", format!("session={}", session_token).as_str());
                then.status(501);
            });
            assert!(fetcher.get_puzzle_input(day).is_err());
            mock.assert();
        }
    }

    //noinspection DuplicatedCode
    #[test]
    fn test_error_when_fetching_from_remote_if_missing_session_token() {
        let server = MockServer::start();
        let base_url = &server.base_url();
        let puzzle_store_dir = tempdir().unwrap();
        let session_token_path = NamedTempFile::new().unwrap();
        let mut fetcher = PuzzleInputFetcher::create_custom(
            base_url,
            puzzle_store_dir.path(),
            session_token_path.path());
        for day in 1..26 {
            let mock = server.mock(|when, then| {
                when.method(GET)
                    .path(remote_url_path(day).as_str());
                then.status(400)
                    .body("Puzzle inputs differ by user.  Please log in to get your puzzle input.");
            });
            assert!(fetcher.get_puzzle_input(day).is_err());
            mock.assert_hits(0);
        }
    }

    //noinspection DuplicatedCode
    #[test]
    fn test_error_when_fetching_from_remote_if_invalid_session_token() {
        let server = MockServer::start();
        let base_url = &server.base_url();
        let puzzle_store_dir = tempdir().unwrap();
        let mut truncated_token = random_session_token();
        truncated_token.truncate(95);
        let session_tokens: Vec<String> = vec![
            truncated_token, // session token too short
            random_session_token() + "a", // session token too long
            "X".repeat(96), // session token has invalid characters
            String::new(), // session token is the empty string
        ];
        for session_token in session_tokens {
            let mut session_token_path = NamedTempFile::new().unwrap();
            session_token_path.write_all(session_token.as_bytes()).unwrap();
            let mut fetcher = PuzzleInputFetcher::create_custom(
                base_url,
                puzzle_store_dir.path(),
                session_token_path.path());
            for day in 1..26 {
                let puzzle_input = random_puzzle();
                let mock = server.mock(|when, then| {
                    when.method(GET)
                        .path(remote_url_path(day).as_str())
                        .header("Cookie", format!("session={}", session_token).as_str());
                    then.status(200)
                        .body(puzzle_input);
                });
                assert!(fetcher.get_puzzle_input(day).is_err());
                mock.assert_hits(0);
            }
        }
    }

    //noinspection DuplicatedCode
    #[test]
    fn test_error_when_fetching_from_remote_if_session_token_wrong() {
        let server = MockServer::start();
        let base_url = &server.base_url();
        let puzzle_store_dir = tempdir().unwrap();
        let mut session_token_path = NamedTempFile::new().unwrap();
        let session_token = random_session_token();
        session_token_path.write_all(session_token.as_bytes()).unwrap();
        let mut fetcher = PuzzleInputFetcher::create_custom(
            base_url,
            puzzle_store_dir.path(),
            session_token_path.path());
        for day in 1..26 {
            let mock = server.mock(|when, then| {
                when.method(GET)
                    .path(remote_url_path(day).as_str())
                    .header("Cookie", format!("session={}", session_token).as_str());
                then.status(400)
                    .body("Puzzle inputs differ by user.  Please log in to get your puzzle input.");
            });
            assert!(fetcher.get_puzzle_input(day).is_err());
            mock.assert();
        }
    }

    //noinspection DuplicatedCode
    #[test]
    fn test_error_when_fetching_from_remote_if_puzzle_requested_early() {
        let server = MockServer::start();
        let base_url = &server.base_url();
        let puzzle_store_dir = tempdir().unwrap();
        let mut session_token_path = NamedTempFile::new().unwrap();
        let session_token = random_session_token();
        session_token_path.write_all(session_token.as_bytes()).unwrap();
        let mut fetcher = PuzzleInputFetcher::create_custom(
            base_url,
            puzzle_store_dir.path(),
            session_token_path.path());
        for day in 1..26 {
            let mock = server.mock(|when, then| {
                when.method(GET)
                    .path(remote_url_path(day).as_str())
                    .header("Cookie", format!("session={}", session_token).as_str());
                then.status(404)
                    .body("Please don't repeatedly request this endpoint before it unlocks! \
                           The calendar countdown is synchronized with the server time; \
                           the link will be enabled on the calendar the instant this puzzle becomes available.");
            });
            assert!(fetcher.get_puzzle_input(day).is_err());
            mock.assert();
        }
    }

    fn random_puzzle() -> String {
        // Puzzle inputs tend to contain a wide variety of ASCII characters including line feed.
        // They can also be fairly large.
        let charset = format!("{}{}{}{}{}{}{}{}",
            "\n", // ASCII code 10 (line feed)
            " !\"#$%&'()*+,-./", // ASCII codes 32-47 (symbols)
            "0123456789", // ASCII codes 48-57 (digits)
            ":;<=>?@", // ASCII codes 58-64 (symbols)
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ", // ASCII codes 65-90 (uppercase letters)
            "[\\]^_`", // ASCII codes 91-96 (symbols)
            "abcdefghijklmnopqrstuvwxyz", // ASCII codes 97-122 (lowercase letters)
            "{|}~" // ASCII codes 123-126 (symbols)
        );
        random_string(charset.as_str(), 65535)
    }

    fn random_session_token() -> String {
        // Session tokens appear to be 96 characters of ASCII hex digits
        random_string("0123456789abcdef", 96)
    }

    fn random_string(charset: &str, length: usize) -> String {
        let mut rng = rand::thread_rng();
        (0..length).map(|_| {
            let i = rng.gen_range(0..charset.len());
            charset.chars().nth(i).unwrap()
        }).collect()
    }
}