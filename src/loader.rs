//! Plugins to load schemas.

pub mod postgresql;

use crate::schema::Schema;

use anyhow::Result;

/// Interface for loading a database schema.
pub trait Loader {
    /// Loads the schema from the database.
    fn load(&self) -> Result<Schema>;
}
