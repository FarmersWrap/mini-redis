use crate::{Connection, Frame, Parse, Shutdown};

/// GC command for managing garbage collection settings
#[derive(Debug)]
pub struct Gc;

impl Gc {
    /// Parse a `Gc` instance from a received frame.
    ///
    /// The `Parse` argument provides a cursor-like API to read fields from the
    /// `Frame`. At this point, the entire frame has already been received from
    /// the socket.
    ///
    /// The `GC` string has already been consumed.
    ///
    /// # Returns
    ///
    /// Returns the `Gc` value on success. If the frame is malformed, `Err` is
    /// returned.
    ///
    /// # Format
    ///
    /// Expects an array frame containing:
    /// - `GC` (command name)
    /// - `GET` or `SET` (subcommand)
    /// - Optional parameters for SET
    ///
    /// ```text
    /// GC GET                    # Get current GC configuration
    /// GC SET interval 250       # Set cleanup interval to 250ms
    /// GC SET batch 100          # Set batch size to 100
    /// GC SET enabled true       # Enable/disable GC
    /// ```
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<Gc> {
        // The `GC` string has already been consumed. Parse the subcommand.
        let subcommand = parse.next_string()?.to_lowercase();
        
        match subcommand.as_str() {
            "get" => {
                // GC GET - just consume any remaining fields
                parse.finish()?;
                Ok(Gc)
            }
            "set" => {
                // GC SET parameter value
                let param = parse.next_string()?.to_lowercase();
                let _value = parse.next_string()?;
                parse.finish()?;
                
                // Validate parameters
                match param.as_str() {
                    "interval" | "batch" | "enabled" => {
                        // Valid parameter, continue
                    }
                    _ => {
                        return Err(format!("Invalid GC parameter: {}", param).into());
                    }
                }
                
                Ok(Gc)
            }
            _ => {
                Err(format!("Invalid GC subcommand: {}", subcommand).into())
            }
        }
    }

    /// Apply the `Gc` command.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    pub(crate) async fn apply(self, _db: &crate::Db, dst: &mut Connection, _shutdown: &mut Shutdown) -> crate::Result<()> {
        // For now, just return basic GC information
        // In a full implementation, this would interact with the GC task
        let response = Frame::Bulk("GC command - use GC GET for configuration, GC SET for updates".into());
        dst.write_frame(&response).await?;
        
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Frame;

    #[test]
    fn test_gc_parse_frames_get() {
        let mut frame = Frame::array();
        frame.push_bulk("get".into());

        let mut parse = Parse::new(frame).unwrap();
        let cmd = Gc::parse_frames(&mut parse).unwrap();

        assert!(matches!(cmd, Gc));
    }

    #[test]
    fn test_gc_parse_frames_set() {
        let mut frame = Frame::array();
        frame.push_bulk("set".into());
        frame.push_bulk("interval".into());
        frame.push_bulk("250".into());

        let mut parse = Parse::new(frame).unwrap();
        let cmd = Gc::parse_frames(&mut parse).unwrap();

        assert!(matches!(cmd, Gc));
    }

    #[test]
    fn test_gc_parse_frames_invalid_subcommand() {
        let mut frame = Frame::array();
        frame.push_bulk("invalid".into());

        let mut parse = Parse::new(frame).unwrap();
        let result = Gc::parse_frames(&mut parse);
        assert!(result.is_err());
    }
} 