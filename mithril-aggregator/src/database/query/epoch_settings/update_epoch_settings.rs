use sqlite::Value;

use mithril_common::entities::{Epoch, ProtocolParameters};
use mithril_persistence::sqlite::{Query, SourceAlias, SqLiteEntity, WhereCondition};

use crate::database::record::EpochSettingsRecord;

/// Query to update [EpochSettingsRecord] in the sqlite database
pub struct UpdateEpochSettingsQuery {
    condition: WhereCondition,
}

impl UpdateEpochSettingsQuery {
    pub fn one(epoch: Epoch, protocol_parameters: ProtocolParameters) -> Self {
        let epoch_settings_id: i64 = epoch.try_into().unwrap();

        Self {
            condition: WhereCondition::new(
                "(epoch_setting_id, protocol_parameters) values (?1, ?2)",
                vec![
                    Value::Integer(epoch_settings_id),
                    Value::String(serde_json::to_string(&protocol_parameters).unwrap()),
                ],
            ),
        }
    }
}

impl Query for UpdateEpochSettingsQuery {
    type Entity = EpochSettingsRecord;

    fn filters(&self) -> WhereCondition {
        self.condition.clone()
    }

    fn get_definition(&self, condition: &str) -> String {
        // it is important to alias the fields with the same name as the table
        // since the table cannot be aliased in a RETURNING statement in SQLite.
        let projection = Self::Entity::get_projection()
            .expand(SourceAlias::new(&[("{:epoch_setting:}", "epoch_setting")]));

        format!("insert or replace into epoch_setting {condition} returning {projection}")
    }
}

#[cfg(test)]
mod tests {
    use mithril_common::test_utils::fake_data;
    use mithril_persistence::sqlite::ConnectionExtensions;

    use crate::database::query::GetEpochSettingsQuery;
    use crate::database::test_helper::{insert_epoch_settings, main_db_connection};

    use super::*;

    #[test]
    fn test_update_epoch_settings() {
        let connection = main_db_connection().unwrap();
        insert_epoch_settings(&connection, &[3]).unwrap();

        let epoch_settings_record = connection
            .fetch_first(UpdateEpochSettingsQuery::one(
                Epoch(3),
                fake_data::protocol_parameters(),
            ))
            .unwrap()
            .unwrap();

        assert_eq!(Epoch(3), epoch_settings_record.epoch_settings_id);
        assert_eq!(
            fake_data::protocol_parameters(),
            epoch_settings_record.protocol_parameters
        );

        let mut cursor = connection
            .fetch(GetEpochSettingsQuery::by_epoch(Epoch(3)).unwrap())
            .unwrap();
        let epoch_settings_record = cursor
            .next()
            .expect("Should have an epoch settings for epoch 3.");

        assert_eq!(Epoch(3), epoch_settings_record.epoch_settings_id);
        assert_eq!(
            fake_data::protocol_parameters(),
            epoch_settings_record.protocol_parameters
        );
        assert_eq!(0, cursor.count());
    }
}
