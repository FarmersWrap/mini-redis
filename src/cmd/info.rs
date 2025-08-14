use crate::{Connection, Frame, Parse};
use bytes::Bytes;
use tracing::{debug, instrument};

/// Returns information and statistics about the server.
///
/// This command provides various information about the server's state,
/// including metrics, configuration, and performance data.
#[derive(Debug)]
pub struct Info {
    /// Optional section to return (not implemented yet)
    _section: Option<String>,
}

impl Info {
    /// Parse an `Info` instance from a received frame.
    ///
    /// The `Parse` argument provides a cursor-like API to read fields from the
    /// `Frame`. At this point, the entire frame has already been received from
    /// the socket.
    ///
    /// The `INFO` string has already been consumed.
    ///
    /// # Returns
    ///
    /// Returns the `Info` value on success. If the frame is malformed, `Err` is
    /// returned.
    ///
    /// # Format
    ///
    /// Expects an array frame containing one or two entries.
    ///
    /// ```text
    /// INFO [section]
    /// ```
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Info> {
        // The `INFO` string has already been consumed.
        // The next value is optional and represents a section name.
        let section = match parse.next_string() {
            Ok(s) => Some(s),
            Err(_) => None,
        };

        Ok(Info { _section: section })
    }

    /// Apply the `Info` command to the specified `Db` instance.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    #[instrument(skip(self, db, dst))]
    pub(crate) async fn apply(self, db: &crate::Db, dst: &mut Connection) -> crate::Result<()> {
        // Get metrics from the database
        let metrics = db.get_metrics();
        let info_string = metrics.info_string();

        debug!(?info_string);

        // Write the response back to the client as a bulk string
        let response = Frame::Bulk(Bytes::from(info_string));
        dst.write_frame(&response).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Frame;

    #[test]
    fn test_info_parse_frames() {
        let mut frame = Frame::array();
        frame.push_bulk("server".into());

        let mut parse = Parse::new(frame).unwrap();
        let cmd = Info::parse_frames(&mut parse).unwrap();
        assert!(cmd._section.is_some());
        assert_eq!(cmd._section.unwrap(), "server");
    }

    #[test]
    fn test_info_parse_frames_no_section() {
        let mut frame = Frame::array();

        let mut parse = Parse::new(frame).unwrap();
        let cmd = Info::parse_frames(&mut parse).unwrap();
        assert!(cmd._section.is_none());
    }
} 