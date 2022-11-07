use std::{collections::HashMap, error::Error, fmt::Display};

use sqlite::{Connection, Row, Value};

use crate::sqlite::{HydrationError, Projection, ProjectionField, Provider, SqLiteEntity};

/// Application using a database
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationNodeType {
    /// Aggregator node type
    Aggregator,

    /// Signer node type
    Signer,
}

impl ApplicationNodeType {
    /// [ApplicationNodeType] constructor.
    pub fn new(node_type: &str) -> Result<Self, Box<dyn Error>> {
        match node_type {
            "aggregator" => Ok(Self::Aggregator),
            "signer" => Ok(Self::Signer),
            _ => Err(format!("unknown node type '{}'", node_type).into()),
        }
    }
}

impl Display for ApplicationNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aggregator => write!(f, "aggregator"),
            Self::Signer => write!(f, "signer"),
        }
    }
}

/// Entity related to the `db_version` database table.
#[derive(Debug, PartialEq, Eq)]
pub struct DatabaseVersion {
    /// Semver of the database structure.
    pub database_version: String,

    /// Name of the application.
    pub application_type: ApplicationNodeType,
}

impl SqLiteEntity for DatabaseVersion {
    fn hydrate(row: Row) -> Result<Self, HydrationError> {
        Ok(Self {
            database_version: row.get::<String, _>(0),
            application_type: ApplicationNodeType::new(&row.get::<String, _>(1))
                .map_err(|e| HydrationError::InvalidData(format!("{}", e)))?,
        })
    }
}

/// Projection dedicated to [DatabaseVersion] entities.
struct DbVersionProjection {
    fields: Vec<ProjectionField>,
}

impl Projection for DbVersionProjection {
    fn set_field(&mut self, field: ProjectionField) {
        self.fields.push(field);
    }

    fn get_fields(&self) -> &Vec<ProjectionField> {
        &self.fields
    }
}
impl DbVersionProjection {
    pub fn new() -> Self {
        let mut projection = Self { fields: Vec::new() };
        projection.add_field("db_version", "{:version:}.version", "text");
        projection.add_field("application_type", "{:version:}.application_type", "text");

        projection
    }
}

/// Provider for the [DatabaseVersion] entities using the [DbVersionProjection].
pub struct VersionProvider<'conn> {
    connection: &'conn Connection,
    projection: DbVersionProjection,
}

impl<'conn> VersionProvider<'conn> {
    /// [VersionProvider] constructor.
    pub fn new(connection: &'conn Connection) -> Self {
        Self {
            connection,
            projection: DbVersionProjection::new(),
        }
    }

    /// Method to create the table at the beginning of the migration procedure.
    /// This code is temporary and should not last.
    pub fn create_table_if_not_exists(&self) -> Result<(), Box<dyn Error>> {
        let connection = self.get_connection();
        let sql = "select exists(select name from sqlite_master where type='table' and name='db_version') as table_exists";
        let table_exists = connection
            .prepare(sql)?
            .into_cursor()
            .bind(&[])?
            .next()
            .unwrap()?
            .get::<i64, _>(0)
            == 1;

        if !table_exists {
            let sql = r#"
create table db_version (application_type text not null primary key, version text not null)
"#;
            connection.execute(sql)?;
        }

        Ok(())
    }

    /// Read the database version from the database.
    pub fn get_database_version(&self) -> Result<Option<DatabaseVersion>, Box<dyn Error>> {
        let result = self.find(None, &[])?.next();

        Ok(result)
    }
}

impl<'conn> Provider<'conn> for VersionProvider<'conn> {
    type Entity = DatabaseVersion;

    fn get_projection(&self) -> &dyn Projection {
        &self.projection
    }

    fn get_connection(&'conn self) -> &Connection {
        self.connection
    }

    fn get_definition(&self, condition: Option<&str>) -> String {
        let where_clause = condition.unwrap_or("true");
        let mut aliases = HashMap::new();
        let _ = aliases.insert("{:version:}".to_string(), "db_version".to_string());
        let projection = self.get_projection().expand(aliases);

        format!(
            r#"
select {projection}
from db_version
where {where_clause}
"#
        )
    }
}

/// Write [Provider] for the [DatabaseVersion] entities.
/// This will perform an UPSERT and return the updated entity.
pub struct VersionUpdatedProvider<'conn> {
    connection: &'conn Connection,
    projection: DbVersionProjection,
}

impl<'conn> VersionUpdatedProvider<'conn> {
    /// [VersionUpdateprovider] constructor.
    pub fn new(connection: &'conn Connection) -> Self {
        Self {
            connection,
            projection: DbVersionProjection::new(),
        }
    }

    /// Persist the given entity and return the projection of the saved entity.
    pub fn save(&self, version: DatabaseVersion) -> Result<DatabaseVersion, Box<dyn Error>> {
        let params = [
            Value::String(format!("{}", version.application_type)),
            Value::String(version.database_version),
        ];
        let entity = self
            .find(None, &params)?
            .next()
            .ok_or("No data returned after insertion")?;

        Ok(entity)
    }
}

impl<'conn> Provider<'conn> for VersionUpdatedProvider<'conn> {
    type Entity = DatabaseVersion;

    fn get_projection(&self) -> &dyn Projection {
        &self.projection
    }

    fn get_connection(&'conn self) -> &Connection {
        self.connection
    }

    fn get_definition(&self, condition: Option<&str>) -> String {
        let _where_clause = condition.unwrap_or("true");
        let mut aliases = HashMap::new();
        let _ = aliases.insert("{:version:}".to_string(), "db_version".to_string());
        let projection = self.get_projection().expand(aliases);

        format!(
            r#"
insert into db_version (application_type, version) values (?, ?)
  on conflict (application_type) do update set version = excluded.version
returning {projection}
"#
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projection() {
        let projection = DbVersionProjection::new();
        let mut aliases: HashMap<String, String> = HashMap::new();
        let _ = aliases.insert("{:version:}".to_string(), "whatever".to_string());

        assert_eq!(
            "whatever.version as db_version, whatever.application_type as application_type"
                .to_string(),
            projection.expand(aliases)
        );
    }

    #[test]
    fn test_definition() {
        let connection = Connection::open(":memory:").unwrap();
        let provider = VersionProvider::new(&connection);

        assert_eq!(
            r#"
select db_version.version as db_version, db_version.application_type as application_type
from db_version
where true
"#,
            provider.get_definition(None)
        )
    }

    #[test]
    fn test_updated_entity() {
        let connection = Connection::open(":memory:").unwrap();
        let provider = VersionUpdatedProvider::new(&connection);

        assert_eq!(
            r#"
insert into db_version values (?, ?)
  on conflict on (application_type) do update set version = excluded.version
returning db_version.version as db_version, db_version.application_type as application_type
"#,
            provider.get_definition(None)
        )
    }
}
