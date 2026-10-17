use crate::database::connection::DatabaseKind;
use crate::database::schema::{Blueprint, ColumnDefinition, ColumnType};

pub fn compile_create(blueprint: &Blueprint, kind: DatabaseKind) -> String {
    let mut sql = if kind == DatabaseKind::Mssql {
        format!(
            "IF NOT EXISTS (SELECT * FROM sys.tables WHERE name = '{}')\nCREATE TABLE {} (\n",
            blueprint.table_name, blueprint.table_name
        )
    } else {
        format!("CREATE TABLE IF NOT EXISTS {} (\n", blueprint.table_name)
    };

    let mut col_sqls = Vec::new();
    for col in &blueprint.columns {
        col_sqls.push(format!("  {}", compile_column(col, kind)));
    }

    sql.push_str(&col_sqls.join(",\n"));
    sql.push_str("\n)");

    sql
}

pub fn compile_drop(table_name: &str, kind: DatabaseKind) -> String {
    if kind == DatabaseKind::Mssql {
        format!("IF OBJECT_ID('{}', 'U') IS NOT NULL DROP TABLE {}", table_name, table_name)
    } else {
        format!("DROP TABLE IF EXISTS {}", table_name)
    }
}

fn compile_column(col: &ColumnDefinition, kind: DatabaseKind) -> String {
    let mut sql = format!("{} ", col.name);

    sql.push_str(&get_type(col, kind));

    if !col.nullable {
        sql.push_str(" NOT NULL");
    }

    if let Some(default) = &col.default {
        sql.push_str(&format!(" DEFAULT {}", default));
    }

    if col.unique {
        sql.push_str(" UNIQUE");
    }

    sql
}

fn get_type(col: &ColumnDefinition, kind: DatabaseKind) -> String {
    match col.col_type {
        ColumnType::Id => match kind {
            DatabaseKind::Sqlite => "INTEGER PRIMARY KEY AUTOINCREMENT".to_string(),
            DatabaseKind::MySql => "INT AUTO_INCREMENT PRIMARY KEY".to_string(),
            DatabaseKind::Postgres => "SERIAL PRIMARY KEY".to_string(),
            DatabaseKind::Mssql => "INT IDENTITY(1,1) PRIMARY KEY".to_string(),
        },
        ColumnType::String(len) => {
            let length = len.unwrap_or(255);
            if kind == DatabaseKind::Mssql {
                format!("NVARCHAR({})", length)
            } else {
                format!("VARCHAR({})", length)
            }
        }
        ColumnType::Integer => "INT".to_string(),
        ColumnType::BigInteger => "BIGINT".to_string(),
        ColumnType::Float => match kind {
            DatabaseKind::Mssql => "FLOAT".to_string(),
            _ => "DOUBLE".to_string(),
        },
        ColumnType::Boolean => match kind {
            DatabaseKind::Sqlite => "BOOLEAN".to_string(),
            DatabaseKind::MySql => "TINYINT(1)".to_string(),
            DatabaseKind::Postgres => "BOOLEAN".to_string(),
            DatabaseKind::Mssql => "BIT".to_string(),
        },
        ColumnType::Text => match kind {
            DatabaseKind::Mssql => "NVARCHAR(MAX)".to_string(),
            _ => "TEXT".to_string(),
        },
        ColumnType::Timestamp | ColumnType::DateTime => match kind {
            DatabaseKind::Sqlite => "DATETIME".to_string(),
            DatabaseKind::MySql => "DATETIME".to_string(),
            DatabaseKind::Postgres => "TIMESTAMP".to_string(),
            DatabaseKind::Mssql => "DATETIME2".to_string(),
        },
        ColumnType::Date => "DATE".to_string(),
        ColumnType::Json => match kind {
            DatabaseKind::Sqlite => "TEXT".to_string(),
            DatabaseKind::MySql => "JSON".to_string(),
            DatabaseKind::Postgres => "JSONB".to_string(),
            DatabaseKind::Mssql => "NVARCHAR(MAX)".to_string(),
        },
    }
}
