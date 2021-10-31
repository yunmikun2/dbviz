//! Prints all the data in plain text format.
//!

use crate::drawer::Drawer;
use crate::schema::{Field, Relation, Schema, Table};

use std::io::{Result, Write};

pub struct PlainText;

impl<W> Drawer<W> for PlainText
where
    W: Write,
{
    fn write(&self, schema: &Schema, buffer: &mut W) -> Result<()> {
        buffer.write(b"=== Tables ===\n")?;

        for table in schema.tables.iter() {
            write_table(table, buffer)?;
            buffer.write(b"\n")?;
        }

        buffer.write(b"=== Relations ===\n")?;

        for relation in schema.relations.iter() {
            write_relation(relation, buffer)?;
            buffer.write(b"\n")?;
        }

        buffer.write(b"=== Done ===\n")?;

        Ok(())
    }
}

fn write_table<W>(table: &Table, buffer: &mut W) -> Result<()>
where
    W: Write,
{
    buffer.write(b"[")?;
    buffer.write(table.name.as_bytes())?;
    buffer.write(b"]\n")?;

    for field in table.fields.iter() {
        let Field(field_name, field_type) = field;
        buffer.write(field_name.as_bytes())?;
        buffer.write(b": ")?;
        buffer.write(field_type.as_bytes())?;
        buffer.write(b"\n")?;
    }

    buffer.write(b"\n")?;

    Ok(())
}

fn write_relation<W>(relation: &Relation, buffer: &mut W) -> Result<()>
where
    W: Write,
{
    buffer.write(relation.on_table.as_bytes())?;
    buffer.write(b":")?;
    buffer.write(relation.on_field.as_bytes())?;
    buffer.write(b" -> ")?;
    buffer.write(relation.on_table.as_bytes())?;
    buffer.write(b":")?;
    buffer.write(relation.on_field.as_bytes())?;
    buffer.write(b"\n")?;
    Ok(())
}
