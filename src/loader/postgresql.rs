//! Loader for postgresql.

use crate::loader::Loader;
use crate::schema::{Field, Relation, Schema, Table};

use anyhow::Result;
use itertools::Itertools;
use postgres::tls::NoTls;
use postgres::Client;
use postgres::Row;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

/// Configuration for the loader.
pub struct Config {
    pub hostname: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub schema: String,
}

/// Struct that manages the loading and implements `Loader` trait.
pub struct Conn {
    pg_client: RefCell<Client>,
    schema: String,
}

impl Conn {
    /// Create the loader.
    pub fn new(config: &Config) -> Result<Self> {
        let pg_client = postgres::Config::new()
            .user(&config.username)
            .password(&config.password)
            .dbname(&config.database)
            .host(&config.hostname)
            .connect(NoTls)?;

        let pg_client = RefCell::new(pg_client);
        let schema = config.schema.to_string();
        Ok(Self { pg_client, schema })
    }
}

impl Loader for Conn {
    fn load(&self) -> Result<Schema> {
        let mut client = self.pg_client.borrow_mut();
        let tables_rows = client.query(tables_query(), &[&self.schema])?;
        let relations_rows = client.query(relations_query(), &[&self.schema])?;

        let tables: Vec<_> = tables_rows
            .into_iter()
            .group_by(|row| row.get(0))
            .into_iter()
            .map(|(name, rows)| {
                let fields: Vec<_> = rows
                    .into_iter()
                    .map(|row| {
                        let field: Field = row.try_into().unwrap();
                        field
                    })
                    .collect();

                Table { name, fields }
            })
            .collect();

        let relations: Vec<_> = relations_rows
            .into_iter()
            .map(|row| {
                let relation: Relation = row.try_into().unwrap();
                // let relation = Relation::try_from(row).unwrap();
                relation
            })
            .collect();

        Ok(Schema { relations, tables })
    }
}

impl TryFrom<Row> for Relation {
    type Error = String;

    fn try_from(row: Row) -> std::result::Result<Self, String> {
        let fields: HashMap<String, String> = row
            .columns()
            .iter()
            .enumerate()
            .map(|(i, c)| (c.name().to_string(), row.get(i)))
            .collect();

        Ok(Self {
            on_table: fetch_field(&fields, "on_table")?,
            on_field: fetch_field(&fields, "on_field")?,
            to_table: fetch_field(&fields, "to_table")?,
            to_field: fetch_field(&fields, "to_field")?,
        })
    }
}

impl TryFrom<Row> for Field {
    type Error = String;

    fn try_from(row: Row) -> std::result::Result<Self, String> {
        let fields: HashMap<String, String> = row
            .columns()
            .iter()
            .enumerate()
            .map(|(i, c)| (c.name().to_string(), row.get(i)))
            .collect();

        Ok(Self(
            fetch_field(&fields, "column_name")?,
            fetch_field(&fields, "data_type")?,
        ))
    }
}

fn fetch_field(map: &HashMap<String, String>, key: &str) -> std::result::Result<String, String> {
    map.get(key)
        .map(|s| s.clone())
        .ok_or(format!("could not find field {}", key))
}

fn tables_query() -> &'static str {
    "
    select table_name, column_name, data_type
      from information_schema.columns
     where table_schema = $1
     order by table_name, column_name
    "
}

fn relations_query() -> &'static str {
    "
    select *
      from (
        select ns.nspname AS schemaname,
               cl.relname AS on_table,
               attr.attname AS on_field,
               clf.relname AS to_table,
               attrf.attname AS to_field
          from pg_constraint con
                 join pg_class cl
                     on con.conrelid = cl.oid
                 join pg_namespace ns
                     on cl.relnamespace = ns.oid
                 join pg_class clf
                     on con.confrelid = clf.oid
                 join pg_attribute attr
                     on attr.attnum = ANY(con.conkey) and
                 attr.attrelid = con.conrelid
                 join pg_attribute attrf
                     on attrf.attnum = ANY(con.confkey) and
                 attrf.attrelid = con.confrelid
      ) as fk
     where fk.schemaname = $1
    "
}
