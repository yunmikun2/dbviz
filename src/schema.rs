//! Core entities.

/// All the schema information.
#[derive(Debug)]
pub struct Schema {
    /// List of tables in the database.
    pub tables: Vec<Table>,
    /// List of relations in the database.
    pub relations: Vec<Relation>,
}

/// Table information.
#[derive(Debug)]
pub struct Table {
    /// Table name.
    pub name: String,
    /// List of fields.
    pub fields: Vec<Field>,
}

/// Row description.
#[derive(Debug)]
pub struct Field(pub FieldName, pub FieldType);

/// Relation node.
#[derive(Debug)]
pub struct Relation {
    /// Table that the constraint references.
    pub on_table: TableName,
    /// Field that the constraint references.
    pub on_field: FieldName,
    /// Table which the fk references.
    pub to_table: TableName,
    /// Field which the fk references.
    pub to_field: FieldName,
}

pub type TableName = String;
pub type FieldName = String;
pub type FieldType = String;
