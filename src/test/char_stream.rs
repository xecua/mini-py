#![allow(dead_code)]
use crate::char_stream::CharStream;
use std::fs::{read_to_string,remove_file};
use stdio_override::StdoutOverride;

const TEST_OUTPUT_FILENAME: &'static str = "tmp.txt";

// ?????


// #[test]
fn test_empty() {
    {
        let mut char_stream = CharStream::new("testcase/empty.txt").unwrap();
        let _guard = StdoutOverride::override_file(TEST_OUTPUT_FILENAME).unwrap();
        char_stream.lc();
        let content = read_to_string(TEST_OUTPUT_FILENAME).unwrap();
        assert_eq!(content, "0");
        remove_file(TEST_OUTPUT_FILENAME).unwrap();
    }
    {
        let mut char_stream = CharStream::new("testcase/empty.txt").unwrap();
        let _guard = StdoutOverride::override_file(TEST_OUTPUT_FILENAME).unwrap();
        char_stream.apos();
        let content = read_to_string(TEST_OUTPUT_FILENAME).unwrap();
        assert_eq!(content, "");
        remove_file(TEST_OUTPUT_FILENAME).unwrap();
    }
}

// #[test]
fn test_only_ln() {
    {
        let mut char_stream = CharStream::new("testcase/only_ln.txt").unwrap();
        let _guard = StdoutOverride::override_file(TEST_OUTPUT_FILENAME).unwrap();
        char_stream.lc();
        let content = read_to_string(TEST_OUTPUT_FILENAME).unwrap();
        assert_eq!(content, "1");
        remove_file(TEST_OUTPUT_FILENAME).unwrap();
    }
    {
        let mut char_stream = CharStream::new("testcase/only_ln.txt").unwrap();
        let _guard = StdoutOverride::override_file(TEST_OUTPUT_FILENAME).unwrap();
        char_stream.apos();
        let content = read_to_string(TEST_OUTPUT_FILENAME).unwrap();
        assert_eq!(content, "");
        remove_file(TEST_OUTPUT_FILENAME).unwrap();
    }
}

// #[test]
fn test_oneline() {
    {
        let mut char_stream = CharStream::new("testcase/oneline.txt").unwrap();
        let _guard = StdoutOverride::override_file(TEST_OUTPUT_FILENAME).unwrap();
        char_stream.lc();
        let content = read_to_string(TEST_OUTPUT_FILENAME).unwrap();
        assert_eq!(content, "1");
        remove_file(TEST_OUTPUT_FILENAME).unwrap();
    }
    {
        let mut char_stream = CharStream::new("testcase/oneline.txt").unwrap();
        let _guard = StdoutOverride::override_file(TEST_OUTPUT_FILENAME).unwrap();
        char_stream.apos();
        let content = read_to_string(TEST_OUTPUT_FILENAME).unwrap();
        assert_eq!(content, "line 1, col 1\n");
        remove_file(TEST_OUTPUT_FILENAME).unwrap();
    }
}

// #[test]
fn test_test() {
    {
        let mut char_stream = CharStream::new("testcase/test.txt").unwrap();
        let _guard = StdoutOverride::override_file(TEST_OUTPUT_FILENAME).unwrap();
        char_stream.lc();
        let content = read_to_string(TEST_OUTPUT_FILENAME).unwrap();
        assert_eq!(content, "4");
        remove_file(TEST_OUTPUT_FILENAME).unwrap();
    }
    {
        let mut char_stream = CharStream::new("testcase/test.txt").unwrap();
        let _guard = StdoutOverride::override_file(TEST_OUTPUT_FILENAME).unwrap();
        char_stream.apos();
        let content = read_to_string(TEST_OUTPUT_FILENAME).unwrap();
        assert_eq!(content, "line 1, col 9\nline 3, col 7\nline 3, col 16\n");
        remove_file(TEST_OUTPUT_FILENAME).unwrap();
    }
}
