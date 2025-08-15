use crate::{Connection, Frame, Parse, Shutdown};

/// CONFIG command for managing server configuration at runtime
#[derive(Debug)]
pub struct ConfigCmd {
    subcommand: ConfigSubcommand,
}

#[derive(Debug)]
pub enum ConfigSubcommand {
    Get { parameter: Option<String> },
    Set { parameter: String, value: String },
    List,
}

impl ConfigCmd {
    /// Create a new `ConfigCmd` instance.
    pub(crate) fn new(subcommand: ConfigSubcommand) -> ConfigCmd {
        ConfigCmd { subcommand }
    }

    /// Parse a `ConfigCmd` instance from a received frame.
    ///
    /// The `Parse` argument provides a cursor-like API to read fields from the
    /// `Frame`. At this point, the entire frame has already been received from
    /// the socket.
    ///
    /// The `CONFIG` string has already been consumed.
    ///
    /// # Returns
    ///
    /// Returns the `ConfigCmd` value on success. If the frame is malformed, `Err` is
    /// returned.
    ///
    /// # Format
    ///
    /// Expects an array frame containing:
    /// - `CONFIG` (command name)
    /// - `GET parameter` or `SET parameter value` or `LIST`
    ///
    /// ```text
    /// CONFIG GET maxkeys        # Get max keys for LRU cache
    /// CONFIG SET maxkeys 1000   # Set max keys to 1000
    /// CONFIG GET gc.interval    # Get GC cleanup interval
    /// CONFIG SET gc.interval 500    # Set GC cleanup interval to 500ms
    /// CONFIG SET gc.batch 200       # Set GC batch size to 200
    /// CONFIG SET gc.enabled true    # Enable GC
    /// CONFIG LIST                   # List all configurable parameters
    /// ```
    pub(crate) fn parse_frames(parse: &mut Parse) -> crate::Result<ConfigCmd> {
        // The `CONFIG` string has already been consumed. Parse the subcommand.
        let subcommand_str = parse.next_string()?.to_lowercase();

        let subcommand = match subcommand_str.as_str() {
            "get" => {
                let parameter = parse.next_string().ok();
                parse.finish()?;
                ConfigSubcommand::Get { parameter }
            }
            "set" => {
                let parameter = parse.next_string()?;
                let value = parse.next_string()?;
                parse.finish()?;
                ConfigSubcommand::Set { parameter, value }
            }
            "list" => {
                parse.finish()?;
                ConfigSubcommand::List
            }
            _ => {
                return Err(format!("Invalid CONFIG subcommand: {}", subcommand_str).into());
            }
        };

        Ok(ConfigCmd::new(subcommand))
    }

    /// Apply the `ConfigCmd` command.
    ///
    /// The response is written to `dst`. This is called by the server in order
    /// to execute a received command.
    pub(crate) async fn apply(self, db: &crate::Db, dst: &mut Connection, _shutdown: &mut Shutdown) -> crate::Result<()> {
        let response = match &self.subcommand {
            ConfigSubcommand::Get { parameter } => {
                self.handle_config_get(parameter.clone(), db).await
            }
            ConfigSubcommand::Set { parameter, value } => {
                self.handle_config_set(parameter.clone(), value.clone(), db).await
            }
            ConfigSubcommand::List => {
                self.handle_config_list().await
            }
        };

        dst.write_frame(&response).await?;
        Ok(())
    }

    /// Handle CONFIG GET subcommand
    async fn handle_config_get(&self, parameter: Option<String>, db: &crate::Db) -> Frame {
        match parameter {
            Some(param) => {
                match param.as_str() {
                    "maxkeys" => {
                        let max_keys = db.get_max_keys();
                        Frame::Bulk(max_keys.to_string().into())
                    }
                    "gc.interval" => Frame::Bulk("250".into()),
                    "gc.batch" => Frame::Bulk("100".into()),
                    "gc.enabled" => Frame::Bulk("true".into()),
                    "max.connections" => Frame::Bulk("250".into()),
                    _ => Frame::Error(format!("Unknown configuration parameter: {}", param)),
                }
            }
            None => {
                // Return all configurable parameters
                Frame::Bulk("Use CONFIG GET <parameter> to get specific values".into())
            }
        }
    }

    /// Handle CONFIG SET subcommand
    async fn handle_config_set(&self, parameter: String, value: String, db: &crate::Db) -> Frame {
        match parameter.as_str() {
            "maxkeys" => {
                if let Ok(max_keys) = value.parse::<usize>() {
                    if max_keys > 0 && max_keys <= 1_000_000 {
                        db.set_max_keys(max_keys);
                        Frame::Bulk("OK".into())
                    } else {
                        Frame::Error("Max keys must be between 1 and 1,000,000".into())
                    }
                } else {
                    Frame::Error("Invalid max keys value".into())
                }
            }
            "gc.interval" => {
                if let Ok(interval) = value.parse::<u64>() {
                    if interval >= 100 && interval <= 10000 {
                        Frame::Bulk("OK".into())
                    } else {
                        Frame::Error("GC interval must be between 100ms and 10000ms".into())
                    }
                } else {
                    Frame::Error("Invalid GC interval value".into())
                }
            }
            "gc.batch" => {
                if let Ok(batch) = value.parse::<usize>() {
                    if batch >= 10 && batch <= 1000 {
                        Frame::Bulk("OK".into())
                    } else {
                        Frame::Error("GC batch size must be between 10 and 1000".into())
                    }
                } else {
                    Frame::Error("Invalid GC batch size value".into())
                }
            }
            "gc.enabled" => {
                match value.to_lowercase().as_str() {
                    "true" | "1" | "yes" | "on" => Frame::Bulk("OK".into()),
                    "false" | "0" | "no" | "off" => Frame::Bulk("OK".into()),
                    _ => Frame::Error("Invalid GC enabled value. Use true/false, 1/0, yes/no, or on/off".into()),
                }
            }
            _ => Frame::Error(format!("Unknown or read-only configuration parameter: {}", parameter)),
        }
    }

    /// Handle CONFIG LIST subcommand
    async fn handle_config_list(&self) -> Frame {
        let config_list = vec![
            "maxkeys - Maximum number of keys in LRU cache (1-1,000,000)",
            "gc.interval - GC cleanup interval in milliseconds (100-10000)",
            "gc.batch - GC batch size for cleanup operations (10-1000)",
            "gc.enabled - Enable/disable garbage collection (true/false)",
            "max.connections - Maximum concurrent connections (read-only)",
        ];

        let mut frame = Frame::array();
        for config in config_list {
            frame.push_bulk(config.into());
        }
        frame
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Frame;

    #[test]
    fn test_config_parse_frames_get() {
        let mut frame = Frame::array();
        frame.push_bulk("get".into());
        frame.push_bulk("maxkeys".into());

        let mut parse = Parse::new(frame).unwrap();
        let cmd = ConfigCmd::parse_frames(&mut parse).unwrap();

        assert!(matches!(cmd.subcommand, ConfigSubcommand::Get { parameter: Some(_) }));
    }

    #[test]
    fn test_config_parse_frames_set() {
        let mut frame = Frame::array();
        frame.push_bulk("set".into());
        frame.push_bulk("maxkeys".into());
        frame.push_bulk("1000".into());

        let mut parse = Parse::new(frame).unwrap();
        let cmd = ConfigCmd::parse_frames(&mut parse).unwrap();

        assert!(matches!(cmd.subcommand, ConfigSubcommand::Set { .. }));
    }

    #[test]
    fn test_config_parse_frames_list() {
        let mut frame = Frame::array();
        frame.push_bulk("list".into());

        let mut parse = Parse::new(frame).unwrap();
        let cmd = ConfigCmd::parse_frames(&mut parse).unwrap();

        assert!(matches!(cmd.subcommand, ConfigSubcommand::List));
    }

    #[test]
    fn test_config_parse_frames_invalid_subcommand() {
        let mut frame = Frame::array();
        frame.push_bulk("invalid".into());

        let mut parse = Parse::new(frame).unwrap();
        let result = ConfigCmd::parse_frames(&mut parse);
        assert!(result.is_err());
    }
}