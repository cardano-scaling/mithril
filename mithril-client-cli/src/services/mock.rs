use std::path::Path;

use async_trait::async_trait;
use mithril_common::certificate_chain::CertificateVerifier;
use mithril_common::crypto_helper::ProtocolGenesisVerificationKey;
use mithril_common::digesters::{ImmutableDigester, ImmutableDigesterError};
use mithril_common::entities::{Beacon, Certificate, ProtocolMessage};
use mithril_common::StdResult;
use mockall::mock;

mock! {
    pub DigesterImpl { }

    #[async_trait]
    impl ImmutableDigester for DigesterImpl {
        async fn compute_digest(
            &self,
            dirpath: &Path,
            beacon: &Beacon,
        ) -> Result<String, ImmutableDigesterError>;
    }
}

mock! {
    pub CertificateVerifierImpl { }

    #[async_trait]
    impl CertificateVerifier for CertificateVerifierImpl {
        async fn verify_genesis_certificate(
            &self,
            genesis_certificate: &Certificate,
            genesis_verification_key: &ProtocolGenesisVerificationKey,
        ) -> StdResult<()>;

        async fn verify_certificate(
            &self,
            certificate: &Certificate,
            genesis_verification_key: &ProtocolGenesisVerificationKey,
        ) -> StdResult<Option<Certificate>>;

        async fn verify_certificate_chain(
            &self,
            certificate: Certificate,
            genesis_verification_key: &ProtocolGenesisVerificationKey,
        ) -> StdResult<()>;

        fn verify_protocol_message(
            &self,
            protocol_message: &ProtocolMessage,
            certificate: &Certificate,
        ) -> bool;
    }
}