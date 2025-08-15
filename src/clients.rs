//! Client implementations for Mini-Redis
//! 
//! This module provides various Redis client implementations including:
//! - Async client for non-blocking operations
//! - Blocking client for synchronous operations
//! - Buffered client for improved performance

// Client implementations
mod client;
mod blocking_client;
mod buffered_client;

// Public exports
pub use client::{Client, Message, Subscriber};
pub use blocking_client::BlockingClient;
pub use buffered_client::BufferedClient; 