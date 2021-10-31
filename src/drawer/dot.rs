//! Prints all the data in dot format.
//!

use crate::drawer::Drawer;
use crate::schema::{Field, Relation, Schema, Table};

use std::io::{Result, Write};

/// Graphviz drawer.
pub struct Dot;

const GRAPH_HEADER: &str = "\
digraph erd {
  graph [
    rankdir = \"LR\"
  ];
  node [
    fontsize = \"16\"
    shape = \"plaintext\"
  ];
  edge [
  ];
";

const GRAPH_FOOTER: &str = "\n}\n";

impl<W> Drawer<W> for Dot
where
    W: Write,
{
    fn write(&self, schema: &Schema, buffer: &mut W) -> Result<()> {
        buffer.write(GRAPH_HEADER.as_bytes())?;

        for table in schema.tables.iter() {
            write_table(table, buffer)?;
            buffer.write(b"\n")?;
        }

        for relation in schema.relations.iter() {
            write_relation(relation, buffer)?;
            // buffer.write(b"\n")?;
        }

        buffer.write(GRAPH_FOOTER.as_bytes())?;

        Ok(())
    }
}

fn write_table<W>(table: &Table, buffer: &mut W) -> Result<()>
where
    W: Write,
{
    buffer.write(table_header(&table.name).as_bytes())?;

    for field in table.fields.iter() {
        let Field(field_name, field_type) = field;
        buffer.write(table_field(field_name, field_type).as_bytes())?;
    }

    buffer.write(table_footer().as_bytes())?;

    Ok(())
}

fn table_header(name: &str) -> String {
    format!(
        "  \"{}\" [label=<<table border=\"0\" cellborder=\"1\" cellspacing=\"0\">
            <tr><td port=\"__title\"><font><b>{}</b></font></td></tr>\n",
        name, name
    )
}

fn table_field(field_name: &str, field_type: &str) -> String {
    format!(
        "            <tr><td port=\"{}\"><font>{}: {}</font></td></tr>\n",
        field_name, field_name, field_type
    )
}

fn table_footer() -> String {
    String::from("          </table>>];\n")
}

fn write_relation<W>(relation: &Relation, buffer: &mut W) -> Result<()>
where
    W: Write,
{
    let relation = format!(
        "\"{}\":\"{}\" -> \"{}\":\"{}\"\n",
        relation.on_table, relation.on_field, relation.to_table, relation.to_field
    );

    buffer.write(relation.as_bytes())?;
    Ok(())
}
