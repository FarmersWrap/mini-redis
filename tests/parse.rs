use mini_redis::{Frame, Parse, ParseError};
use bytes::Bytes;

#[test]
fn test_parse_next_string() {
    let frames = vec![
        Frame::Bulk(Bytes::from("hello")),
        Frame::Bulk(Bytes::from("world")),
        Frame::Simple("OK".to_string()),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test parsing bulk strings
    assert_eq!(parse.next_string().unwrap(), "hello");
    assert_eq!(parse.next_string().unwrap(), "world");

    // Test parsing simple string
    assert_eq!(parse.next_string().unwrap(), "OK");

    // Test end of iterator
    assert!(parse.next_string().is_err());
}

#[test]
fn test_parse_next_bytes() {
    let frames = vec![
        Frame::Bulk(Bytes::from("hello")),
        Frame::Bulk(Bytes::from("world")),
    ];

    let mut parse = Parse::new(frames.into_iter());

    assert_eq!(parse.next_bytes().unwrap(), Bytes::from("hello"));
    assert_eq!(parse.next_bytes().unwrap(), Bytes::from("world"));

    // Test end of iterator
    assert!(parse.next_bytes().is_err());
}

#[test]
fn test_parse_next_int() {
    let frames = vec![
        Frame::Integer(42),
        Frame::Integer(100),
        Frame::Bulk(Bytes::from("123")),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test parsing integer frames
    assert_eq!(parse.next_int().unwrap(), 42);
    assert_eq!(parse.next_int().unwrap(), 100);

    // Test parsing bulk string as integer
    assert_eq!(parse.next_int().unwrap(), 123);

    // Test end of iterator
    assert!(parse.next_int().is_err());
}

#[test]
fn test_parse_next_i64() {
    let frames = vec![
        Frame::Integer(42),
        Frame::Integer(-100),
        Frame::Bulk(Bytes::from("-123")),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test parsing positive integer
    assert_eq!(parse.next_i64().unwrap(), 42);

    // Test parsing negative integer
    assert_eq!(parse.next_i64().unwrap(), -100);

    // Test parsing negative bulk string as integer
    assert_eq!(parse.next_i64().unwrap(), -123);

    // Test end of iterator
    assert!(parse.next_i64().is_err());
}

#[test]
fn test_parse_next_frame() {
    let frames = vec![
        Frame::Bulk(Bytes::from("hello")),
        Frame::Integer(42),
        Frame::Simple("OK".to_string()),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test getting next frame
    match parse.next_frame() {
        Ok(Frame::Bulk(ref b)) => assert_eq!(b, b"hello"),
        _ => panic!("Expected Bulk frame"),
    }

    match parse.next_frame() {
        Ok(Frame::Integer(42)) => {},
        _ => panic!("Expected Integer frame with value 42"),
    }

    match parse.next_frame() {
        Ok(Frame::Simple(ref s)) => assert_eq!(s, "OK"),
        _ => panic!("Expected Simple frame"),
    }

    // Test end of iterator
    assert!(parse.next_frame().is_err());
}

#[test]
fn test_parse_peek() {
    let frames = vec![
        Frame::Bulk(Bytes::from("hello")),
        Frame::Integer(42),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test peeking without consuming
    match parse.peek() {
        Ok(Frame::Bulk(ref b)) => assert_eq!(b, b"hello"),
        _ => panic!("Expected Bulk frame"),
    }

    // Verify the frame is still there
    match parse.next_frame() {
        Ok(Frame::Bulk(ref b)) => assert_eq!(b, b"hello"),
        _ => panic!("Expected Bulk frame"),
    }

    // Test peeking at end
    match parse.peek() {
        Ok(Frame::Integer(42)) => {},
        _ => panic!("Expected Integer frame with value 42"),
    }
}

#[test]
fn test_parse_peek_n() {
    let frames = vec![
        Frame::Bulk(Bytes::from("hello")),
        Frame::Integer(42),
        Frame::Simple("OK".to_string()),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test peeking at specific position
    match parse.peek_n(0) {
        Ok(Frame::Bulk(ref b)) => assert_eq!(b, b"hello"),
        _ => panic!("Expected Bulk frame at position 0"),
    }

    match parse.peek_n(1) {
        Ok(Frame::Integer(42)) => {},
        _ => panic!("Expected Integer frame at position 1"),
    }

    match parse.peek_n(2) {
        Ok(Frame::Simple(ref s)) => assert_eq!(s, "OK"),
        _ => panic!("Expected Simple frame at position 2"),
    }

    // Test peeking beyond available frames
    assert!(parse.peek_n(3).is_err());
}

#[test]
fn test_parse_skip() {
    let frames = vec![
        Frame::Bulk(Bytes::from("hello")),
        Frame::Integer(42),
        Frame::Simple("OK".to_string()),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Skip first frame
    parse.skip().unwrap();

    // Verify first frame is skipped
    match parse.next_frame() {
        Ok(Frame::Integer(42)) => {},
        _ => panic!("Expected Integer frame after skip"),
    }

    // Skip second frame
    parse.skip().unwrap();

    // Verify second frame is skipped
    match parse.next_frame() {
        Ok(Frame::Simple(ref s)) => assert_eq!(s, "OK"),
        _ => panic!("Expected Simple frame after second skip"),
    }

    // Test skipping at end
    assert!(parse.skip().is_err());
}

#[test]
fn test_parse_remaining() {
    let frames = vec![
        Frame::Bulk(Bytes::from("hello")),
        Frame::Integer(42),
        Frame::Simple("OK".to_string()),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test initial remaining count
    assert_eq!(parse.remaining(), 3);

    // Consume one frame
    parse.next_frame().unwrap();
    assert_eq!(parse.remaining(), 2);

    // Consume another frame
    parse.next_frame().unwrap();
    assert_eq!(parse.remaining(), 1);

    // Consume last frame
    parse.next_frame().unwrap();
    assert_eq!(parse.remaining(), 0);
}

#[test]
fn test_parse_error_handling() {
    let frames = vec![
        Frame::Bulk(Bytes::from("not_a_number")),
        Frame::Bulk(Bytes::from("also_not_a_number")),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test parsing non-numeric bulk string as integer
    let result = parse.next_int();
    assert!(result.is_err());

    // Test parsing non-numeric bulk string as i64
    let result = parse.next_i64();
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_iterator() {
    let frames: Vec<Frame> = vec![];
    let mut parse = Parse::new(frames.into_iter());

    // Test all methods on empty iterator
    assert!(parse.next_string().is_err());
    assert!(parse.next_bytes().is_err());
    assert!(parse.next_int().is_err());
    assert!(parse.next_i64().is_err());
    assert!(parse.next_frame().is_err());
    assert!(parse.peek().is_err());
    assert!(parse.peek_n(0).is_err());
    assert!(parse.skip().is_err());
    assert_eq!(parse.remaining(), 0);
}

#[test]
fn test_parse_mixed_types() {
    let frames = vec![
        Frame::Bulk(Bytes::from("123")),
        Frame::Integer(456),
        Frame::Bulk(Bytes::from("-789")),
        Frame::Simple("OK".to_string()),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test parsing bulk string as integer
    assert_eq!(parse.next_int().unwrap(), 123);

    // Test parsing integer frame
    assert_eq!(parse.next_int().unwrap(), 456);

    // Test parsing negative bulk string as i64
    assert_eq!(parse.next_i64().unwrap(), -789);

    // Test parsing simple string
    assert_eq!(parse.next_string().unwrap(), "OK");
}

#[test]
fn test_parse_unicode_strings() {
    let frames = vec![
        Frame::Bulk(Bytes::from("Hello 世界")),
        Frame::Bulk(Bytes::from("こんにちは")),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test parsing UTF-8 strings
    assert_eq!(parse.next_string().unwrap(), "Hello 世界");
    assert_eq!(parse.next_string().unwrap(), "こんにちは");
}

#[test]
fn test_parse_large_numbers() {
    let frames = vec![
        Frame::Integer(9223372036854775807), // i64::MAX
        Frame::Integer(-9223372036854775808), // i64::MIN
        Frame::Bulk(Bytes::from("9223372036854775807")),
        Frame::Bulk(Bytes::from("-9223372036854775808")),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test parsing large integers
    assert_eq!(parse.next_int().unwrap(), 9223372036854775807);
    assert_eq!(parse.next_i64().unwrap(), -9223372036854775808);

    // Test parsing large numbers as strings
    assert_eq!(parse.next_int().unwrap(), 9223372036854775807);
    assert_eq!(parse.next_i64().unwrap(), -9223372036854775808);
}

#[test]
fn test_parse_error_types() {
    let frames = vec![
        Frame::Error("ERR invalid command".to_string()),
        Frame::Error("WRONGTYPE Operation against a key holding the wrong kind of value".to_string()),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test parsing error frames as strings
    assert_eq!(parse.next_string().unwrap(), "ERR invalid command");
    assert_eq!(parse.next_string().unwrap(), "WRONGTYPE Operation against a key holding the wrong kind of value");
}

#[test]
fn test_parse_null_handling() {
    let frames = vec![
        Frame::Null,
        Frame::Bulk(Bytes::from("value")),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test that Null frames can be parsed as strings (empty string)
    assert_eq!(parse.next_string().unwrap(), "");

    // Test normal bulk string
    assert_eq!(parse.next_string().unwrap(), "value");
}

#[test]
fn test_parse_array_frames() {
    let frames = vec![
        Frame::Array(vec![
            Frame::Bulk(Bytes::from("nested")),
            Frame::Integer(42),
        ]),
        Frame::Bulk(Bytes::from("simple")),
    ];

    let mut parse = Parse::new(frames.into_iter());

    // Test that array frames can be parsed as strings (string representation)
    let result = parse.next_string();
    assert!(result.is_ok());
    assert!(result.unwrap().contains("nested"));

    // Test normal bulk string
    assert_eq!(parse.next_string().unwrap(), "simple");
}
