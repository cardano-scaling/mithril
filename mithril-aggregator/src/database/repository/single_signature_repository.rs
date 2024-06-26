use std::sync::Arc;

use mithril_common::entities::SingleSignatures;
use mithril_common::StdResult;
use mithril_persistence::sqlite::SqliteConnection;

use crate::database::provider::UpdateSingleSignatureRecordProvider;
use crate::database::record::{OpenMessageRecord, SingleSignatureRecord};

/// Service to deal with single_signature (read & write).
pub struct SingleSignatureRepository {
    connection: Arc<SqliteConnection>,
}

impl SingleSignatureRepository {
    /// Create a new SingleSignatureStoreAdapter service
    pub fn new(connection: Arc<SqliteConnection>) -> Self {
        Self { connection }
    }

    /// Create a new Single Signature in database
    pub async fn create_single_signature(
        &self,
        single_signature: &SingleSignatures,
        open_message: &OpenMessageRecord,
    ) -> StdResult<SingleSignatureRecord> {
        let single_signature = SingleSignatureRecord::try_from_single_signatures(
            single_signature,
            &open_message.open_message_id,
            open_message.epoch.offset_to_signer_retrieval_epoch()?,
        )?;
        let provider = UpdateSingleSignatureRecordProvider::new(&self.connection);

        provider.persist(single_signature)
    }
}
