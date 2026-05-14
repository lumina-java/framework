use crate::database::connection::DatabaseKind;

#[derive(Debug, Clone)]
pub enum ColumnType {
    Id,
    String(Option<usize>), // Option length
    Integer,
    BigInteger,
    Float,
    Boolean,
    Text,
    Timestamp,
    DateTime,
    Date,
    Json,
}

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub col_type: ColumnType,
    pub nullable: bool,
    pub unique: bool,
    pub default: Option<String>,
    pub auto_increment: bool,
    pub primary_key: bool,
}

impl ColumnDefinition {
    pub fn new(name: &str, col_type: ColumnType) -> Self {
        Self {
            name: name.to_string(),
            col_type,
            nullable: false,
            unique: false,
            default: None,
            auto_increment: false,
            primary_key: false,
        }
    }
}

pub struct Blueprint {
    pub table_name: String,
    pub columns: Vec<ColumnDefinition>,
    pub commands: Vec<String>, // For indexes, etc.
}

impl Blueprint {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            columns: Vec::new(),
            commands: Vec::new(),
        }
    }

    pub fn id(&mut self) -> &mut ColumnDefinition {
        let mut col = ColumnDefinition::new("id", ColumnType::Id);
        col.primary_key = true;
        col.auto_increment = true;
        self.columns.push(col);
        self.columns.last_mut().unwrap()
    }

    pub fn string(&mut self, name: &str, length: Option<usize>) -> &mut ColumnDefinition {
        let col = ColumnDefinition::new(name, ColumnType::String(length));
        self.columns.push(col);
        self.columns.last_mut().unwrap()
    }

    pub fn integer(&mut self, name: &str) -> &mut ColumnDefinition {
        let col = ColumnDefinition::new(name, ColumnType::Integer);
        self.columns.push(col);
        self.columns.last_mut().unwrap()
    }

    pub fn big_integer(&mut self, name: &str) -> &mut ColumnDefinition {
        let col = ColumnDefinition::new(name, ColumnType::BigInteger);
        self.columns.push(col);
        self.columns.last_mut().unwrap()
    }

    pub fn text(&mut self, name: &str) -> &mut ColumnDefinition {
        let col = ColumnDefinition::new(name, ColumnType::Text);
        self.columns.push(col);
        self.columns.last_mut().unwrap()
    }

    pub fn boolean(&mut self, name: &str) -> &mut ColumnDefinition {
        let col = ColumnDefinition::new(name, ColumnType::Boolean);
        self.columns.push(col);
        self.columns.last_mut().unwrap()
    }

    pub fn timestamp(&mut self, name: &str) -> &mut ColumnDefinition {
        let col = ColumnDefinition::new(name, ColumnType::Timestamp);
        self.columns.push(col);
        self.columns.last_mut().unwrap()
    }

    pub fn timestamps(&mut self) {
        self.timestamp("created_at")
            .default("CURRENT_TIMESTAMP".to_string());
        self.timestamp("updated_at")
            .default("CURRENT_TIMESTAMP".to_string());
    }

    pub fn soft_deletes(&mut self) {
        let mut col = ColumnDefinition::new("deleted_at", ColumnType::Timestamp);
        col.nullable = true;
        self.columns.push(col);
    }
}

pub struct Schema;

impl Schema {
    pub fn create_sql(
        table_name: &str,
        kind: DatabaseKind,
        callback: impl FnOnce(&mut Blueprint),
    ) -> String {
        let mut blueprint = Blueprint::new(table_name);
        callback(&mut blueprint);

        crate::database::grammar::compile_create(&blueprint, kind)
    }

    pub fn drop_sql(table_name: &str, kind: DatabaseKind) -> String {
        crate::database::grammar::compile_drop(table_name, kind)
    }
}

impl ColumnDefinition {
    pub fn nullable(&mut self) -> &mut Self {
        self.nullable = true;
        self
    }

    pub fn unique(&mut self) -> &mut Self {
        self.unique = true;
        self
    }

    pub fn default(&mut self, val: String) -> &mut Self {
        self.default = Some(val);
        self
    }
}
