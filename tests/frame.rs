use mini_redis::Frame;
use bytes::Bytes;
use std::io::Cursor;

#[test]
fn test_frame_display() {
    // Test Simple frame
    let frame = Frame::Simple("OK".to_string());
    assert_eq!(frame.to_string(), "OK");

    // Test Error frame
    let frame = Frame::Error("ERR invalid command".to_string());
    assert_eq!(frame.to_string(), "ERR invalid command");

    // Test Integer frame
    let frame = Frame::Integer(42);
    assert_eq!(frame.to_string(), "42");

    // Test Integer frame with negative value
    let frame = Frame::Integer(-1);
    assert_eq!(frame.to_string(), "-1");

    // Test Bulk frame
    let frame = Frame::Bulk(Bytes::from("hello world"));
    assert_eq!(frame.to_string(), "hello world");

    // Test Null frame
    let frame = Frame::Null;
    assert_eq!(frame.to_string(), "(nil)");

    // Test Array frame
    let frame = Frame::Array(vec![
        Frame::Bulk(Bytes::from("SET")),
        Frame::Bulk(Bytes::from("key")),
        Frame::Bulk(Bytes::from("value")),
    ]);
    assert_eq!(frame.to_string(), "[SET, key, value]");

    // Test empty array
    let frame = Frame::Array(vec![]);
    assert_eq!(frame.to_string(), "[]");
}

#[test]
fn test_frame_parsing() {
    // Test parsing Simple frame
    let data = b"+OK\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    assert!(matches!(frame, Frame::Simple(ref s) if s == "OK"));

    // Test parsing Error frame
    let data = b"-ERR invalid command\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    assert!(matches!(frame, Frame::Error(ref s) if s == "ERR invalid command"));

    // Test parsing Integer frame
    let data = b":42\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    assert!(matches!(frame, Frame::Integer(42)));

    // Test parsing negative Integer frame
    let data = b":-1\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    assert!(matches!(frame, Frame::Integer(-1)));

    // Test parsing Bulk frame
    let data = b"$11\r\nhello world\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    assert!(matches!(frame, Frame::Bulk(ref b) if **b == *b"hello world"));

    // Test parsing Null frame
    let data = b"$-1\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    assert!(matches!(frame, Frame::Null));

    // Test parsing Array frame
    let data = b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    match frame {
        Frame::Array(ref arr) => {
            assert_eq!(arr.len(), 3);
            assert!(matches!(&arr[0], Frame::Bulk(ref b) if **b == *b"SET"));
            assert!(matches!(&arr[1], Frame::Bulk(ref b) if **b == *b"key"));
            assert!(matches!(&arr[2], Frame::Bulk(ref b) if **b == *b"value"));
        }
        _ => panic!("Expected Array frame"),
    }

    // Test parsing empty array
    let data = b"*0\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    match frame {
        Frame::Array(ref arr) => assert_eq!(arr.len(), 0),
        _ => panic!("Expected empty Array frame"),
    }
}

#[test]
fn test_frame_parsing_errors() {
    // Test incomplete frame
    let data = b"+OK";
    let mut cursor = Cursor::new(data.as_ref());
    let result = Frame::parse(&mut cursor);
    assert!(result.is_err());

    // Test invalid frame type
    let data = b"?invalid\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let result = Frame::parse(&mut cursor);
    assert!(result.is_err());

    // Test invalid integer
    let data = b":not_a_number\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let result = Frame::parse(&mut cursor);
    assert!(result.is_err());

    // Test invalid bulk length
    let data = b"$abc\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let result = Frame::parse(&mut cursor);
    assert!(result.is_err());

    // Test invalid array length
    let data = b"*abc\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let result = Frame::parse(&mut cursor);
    assert!(result.is_err());
}

#[test]
fn test_frame_creation() {
    // Test creating Simple frame
    let frame = Frame::Simple("OK".to_string());
    assert!(matches!(frame, Frame::Simple(ref s) if s == "OK"));

    // Test creating Error frame
    let frame = Frame::Error("ERR invalid command".to_string());
    assert!(matches!(frame, Frame::Error(ref s) if s == "ERR invalid command"));

    // Test creating Integer frame
    let frame = Frame::Integer(42);
    assert!(matches!(frame, Frame::Integer(42)));

    // Test creating Bulk frame
    let frame = Frame::Bulk(Bytes::from("hello world"));
    assert!(matches!(frame, Frame::Bulk(ref b) if **b == *b"hello world"));

    // Test creating Null frame
    let frame = Frame::Null;
    assert!(matches!(frame, Frame::Null));

    // Test creating Array frame
    let frame = Frame::Array(vec![
        Frame::Bulk(Bytes::from("SET")),
        Frame::Bulk(Bytes::from("key")),
        Frame::Bulk(Bytes::from("value")),
    ]);
    match frame {
        Frame::Array(ref arr) => {
            assert_eq!(arr.len(), 3);
            assert!(matches!(&arr[0], Frame::Bulk(ref b) if **b == *b"SET"));
            assert!(matches!(&arr[1], Frame::Bulk(ref b) if **b == *b"key"));
            assert!(matches!(&arr[2], Frame::Bulk(ref b) if **b == *b"value"));
        }
        _ => panic!("Expected Array frame"),
    }
}

#[test]
fn test_frame_equality() {
    let frame1 = Frame::Simple("OK".to_string());
    let frame2 = Frame::Simple("OK".to_string());
    let frame3 = Frame::Simple("ERROR".to_string());

    assert_eq!(frame1, frame2);
    assert_ne!(frame1, frame3);

    let frame1 = Frame::Integer(42);
    let frame2 = Frame::Integer(42);
    let frame3 = Frame::Integer(43);

    assert_eq!(frame1, frame2);
    assert_ne!(frame1, frame3);

    let frame1 = Frame::Bulk(Bytes::from("hello"));
    let frame2 = Frame::Bulk(Bytes::from("hello"));
    let frame3 = Frame::Bulk(Bytes::from("world"));

    assert_eq!(frame1, frame2);
    assert_ne!(frame1, frame3);

    let frame1 = Frame::Array(vec![Frame::Simple("OK".to_string())]);
    let frame2 = Frame::Array(vec![Frame::Simple("OK".to_string())]);
    let frame3 = Frame::Array(vec![Frame::Simple("ERROR".to_string())]);

    assert_eq!(frame1, frame2);
    assert_ne!(frame1, frame3);
}

#[test]
fn test_frame_clone() {
    let frame = Frame::Array(vec![
        Frame::Bulk(Bytes::from("SET")),
        Frame::Bulk(Bytes::from("key")),
        Frame::Bulk(Bytes::from("value")),
    ]);

    let cloned = frame.clone();
    assert_eq!(frame, cloned);
}

#[test]
fn test_frame_debug() {
    let frame = Frame::Simple("OK".to_string());
    let debug_str = format!("{:?}", frame);
    assert!(debug_str.contains("Simple"));
    assert!(debug_str.contains("OK"));
}

#[test]
fn test_frame_with_unicode() {
    // Test with UTF-8 characters
    let data = b"+Hello \xF0\x9F\x98\x80\r\n"; // Hello 😀
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    assert!(matches!(frame, Frame::Simple(ref s) if s.contains("Hello")));

    // Test with Chinese characters
    let data = b"$6\r\n\xE4\xB8\xAD\xE6\x96\x87\r\n"; // 中文
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    assert!(matches!(frame, Frame::Bulk(ref b) if b.len() > 0));
}

#[test]
fn test_frame_with_special_characters() {
    // Test with newlines in bulk data
    let data = b"$11\r\nhello\nworld\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    match frame {
        Frame::Bulk(ref b) => {
            let content = String::from_utf8_lossy(b);
            assert!(content.contains("hello"));
            assert!(content.contains("world"));
            assert!(content.contains('\n'));
        }
        _ => panic!("Expected Bulk frame"),
    }

    // Test with carriage returns in bulk data
    let data = b"$11\r\nhello\rworld\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    match frame {
        Frame::Bulk(ref b) => {
            let content = String::from_utf8_lossy(b);
            assert!(content.contains("hello"));
            assert!(content.contains("world"));
            assert!(content.contains('\r'));
        }
        _ => panic!("Expected Bulk frame"),
    }
}

#[test]
fn test_frame_array_nesting() {
    // Test nested arrays
    let data = b"*2\r\n*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n*1\r\n$5\r\nvalue\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();

    match frame {
        Frame::Array(ref arr) => {
            assert_eq!(arr.len(), 2);

            // First nested array
            match &arr[0] {
                Frame::Array(ref nested) => {
                    assert_eq!(nested.len(), 2);
                    assert!(matches!(&nested[0], Frame::Bulk(ref b) if **b == *b"GET"));
                    assert!(matches!(&nested[1], Frame::Bulk(ref b) if **b == *b"key"));
                }
                _ => panic!("Expected nested Array frame"),
            }

            // Second nested array
            match &arr[1] {
                Frame::Array(ref nested) => {
                    assert_eq!(nested.len(), 1);
                    assert!(matches!(&nested[0], Frame::Bulk(ref b) if **b == *b"value"));
                }
                _ => panic!("Expected nested Array frame"),
            }
        }
        _ => panic!("Expected Array frame"),
    }
}

#[test]
fn test_frame_large_integers() {
    // Test large positive integer
    let data = b":9223372036854775807\r\n"; // i64::MAX
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    assert!(matches!(frame, Frame::Integer(9223372036854775807)));

    // Test large negative integer
    let data = b":-9223372036854775808\r\n"; // i64::MIN
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    assert!(matches!(frame, Frame::Integer(-9223372036854775808)));
}

#[test]
fn test_frame_empty_bulk() {
    // Test empty bulk string
    let data = b"$0\r\n\r\n";
    let mut cursor = Cursor::new(data.as_ref());
    let frame = Frame::parse(&mut cursor).unwrap();
    match frame {
        Frame::Bulk(ref b) => assert_eq!(b.len(), 0),
        _ => panic!("Expected Bulk frame"),
    }
}
