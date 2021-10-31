//! Plugins to draw schemas.

pub mod dot;
pub mod plain_text;

use crate::schema::Schema;

use std::io::{self, Write};

/// Interface for drawing the diagram of the database schema.
pub trait Drawer<W: Write> {
    /// Writes the schema to the buffer.
    fn write(&self, schema: &Schema, buffer: &mut W) -> io::Result<()>;
}
