use crate::{
    crypto_helper::KESPeriod,
    entities::{
        HexEncodedOpCert, HexEncodedVerificationKey, HexEncodedVerificationKeySignature, PartyId,
        SignerWithStake, Stake,
    },
};
use serde::{Deserialize, Serialize};

/// Signer Message
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SignerWithStakeMessagePart {
    /// The unique identifier of the signer
    // TODO: Should be removed once the signer certification is fully deployed
    pub party_id: PartyId,

    /// The public key used to authenticate signer signature
    pub verification_key: HexEncodedVerificationKey,

    /// The encoded signer 'Mithril verification key' signature (signed by the
    /// Cardano node KES secret key).
    // TODO: Option should be removed once the signer certification is fully
    //       deployed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_key_signature: Option<HexEncodedVerificationKeySignature>,

    /// The encoded operational certificate of stake pool operator attached to
    /// the signer node.
    // TODO: Option should be removed once the signer certification is fully
    //       deployed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_certificate: Option<HexEncodedOpCert>,

    /// The KES period used to compute the verification key signature
    // TODO: This KES period should not be used as is and should probably be
    //       within an allowed range of KES periods for the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kes_period: Option<KESPeriod>,

    /// The signer stake
    pub stake: Stake,
}

impl SignerWithStakeMessagePart {
    /// Return a dummy test entity (test-only).
    pub fn dummy() -> Self {
        Self {
            party_id: "pool1m8crhnqj5k2kyszf5j2scshupystyxc887zdfrpzh6ty6eun4fx".to_string(),
            verification_key: "7b22766b223a5b3134352c32332c3135382c31322c3138332c3230392c33322c3134302c33372c3132342c3136362c3231352c3136302c3231352c3235302c3133342c3135342c3235302c3234312c3230362c3139342c3232322c382c35392c33332c392c35382c322c3235312c31302c33322c3135352c3232372c3134332c3232362c35372c3135312c37342c3139392c3131372c37352c3136382c3134302c34362c3233392c3134352c37322c31362c32312c3138312c3139332c3134362c38362c3231332c3230342c3139332c3232332c32352c3135372c33342c33332c3232372c35312c3132362c3132362c3135362c36342c3232302c3139392c3231332c31362c34352c3131302c3234332c33352c3134382c37312c3231382c3132342c3132332c31362c3132312c3135322c31382c32362c3231322c3231342c3230312c3139302c3137342c3131352c39372c3234392c3235342c3131362c3234335d2c22706f70223a5b3138332c3134352c3133392c3234322c3132302c3136302c35362c3131382c3234322c3230342c39312c38392c32312c3138342c382c34372c3231332c3130352c36332c3135302c32312c3231372c352c382c3231392c3138382c3131342c3230352c3136362c31362c3234302c3234302c3231342c31362c3230342c3231382c3139332c3138312c32342c35362c34352c39392c3234342c38312c32352c35322c3232342c36372c3136382c3136392c3130392c3132322c38372c34392c3137302c3138312c3135312c31352c3235322c3139352c3231312c3233342c3139352c34392c39312c31392c35312c3234312c33332c35382c3134302c3235322c3234322c362c342c34302c32312c3136372c3234392c3235312c33362c38372c36302c39362c36392c3135322c3231302c39382c3136352c352c362c34312c39362c3233352c37352c3138335d7d".to_string(),
            verification_key_signature: Some("7b227369676d61223a7b227369676d61223a7b227369676d61223a7b227369676d61223a7b227369676d61223a7b227369676d61223a5b33322c3235332c3134372c3132382c39302c3137372c31322c3231302c3232312c37332c31332c3234332c31302c36342c39322c3139322c3131342c3231302c3231372c3133312c3131322c3137322c3231362c3138372c38382c3138362c32372c31342c3134302c3230362c38312c3234332c3132342c3131342c3234362c3130342c35362c3131342c372c3131342c35372c3232392c3135362c32332c39342c32382c3137372c36302c3131302c34332c3136362c392c3139392c3233302c3133342c37302c3233322c3131362c3130302c36382c39342c3135332c3136342c31345d2c226c68735f706b223a5b3136332c3234362c39382c3232362c31302c36302c3131322c3234312c3136372c36322c3230302c3234382c39392c3133382c3136322c3137322c3137352c31332c3138392c392c302c3234392c34322c3232392c3231312c3230362c3235302c3136372c33382c36332c3138392c3134335d2c227268735f706b223a5b3137322c3138392c3138352c3233302c3234382c39342c3235312c3138312c3137392c38362c38342c32332c3137382c3230352c3232362c382c3233312c3230372c3231302c38332c36382c3231342c3231362c37342c3135362c3130322c32382c3233302c382c35322c3130312c3234355d7d2c226c68735f706b223a5b3134302c3230372c39382c3133362c3134312c3233312c3231352c3230342c35322c3135352c38392c3232332c34382c3134392c3138352c3135352c3131342c3235352c39332c3137352c3234332c37302c3137362c3134332c32342c3132352c32392c3231392c3135302c33362c3232352c33375d2c227268735f706b223a5b3137312c3232392c3139332c3130352c3233342c31382c3232392c38312c3235352c3139322c3133302c32352c33322c3138342c312c33392c39332c3138372c382c3233332c36392c37342c35362c3130312c37302c3231332c3232342c33322c31382c3130322c3235332c35355d7d2c226c68735f706b223a5b34322c302c31382c36382c3135332c3234312c3231342c3133352c3139342c34332c3231322c35382c36322c332c3136302c3133332c34342c37342c3131312c37382c3136322c3133322c35372c32362c3138392c36372c3132372c3232352c37352c3137312c31342c3131345d2c227268735f706b223a5b3133372c3135302c39302c3139362c3232322c3234312c3137392c3133372c3130362c33362c3130322c37322c35372c37312c3130392c3235302c392c33362c3134362c3234372c37342c3231362c31322c342c35322c33372c3233342c37302c3233342c37302c36362c34315d7d2c226c68735f706b223a5b3132312c3134352c3233352c3230392c3135322c39302c3135372c3231392c35312c34302c3136372c322c3137372c3138372c39372c3135332c3138392c3130392c3234392c38392c3231372c3135302c3139322c3131302c3232322c3138332c3134362c39392c3134352c35392c3132352c3132305d2c227268735f706b223a5b32362c38352c3137332c3235302c34382c36322c33382c3231392c39312c3138392c3136382c35322c3137392c34342c39332c39362c31362c3136392c38372c31302c3137302c312c3138392c322c3235352c3131312c3230342c3233372c3138312c3137342c31362c3231385d7d2c226c68735f706b223a5b372c37382c3233342c34362c32372c3234322c332c3234312c3231342c3131322c372c34302c3131372c39372c39332c3234322c3130342c3137302c39352c3138372c37382c3134312c3233382c35392c3231302c352c3133342c3234392c3231372c31302c3132312c33345d2c227268735f706b223a5b3134312c3130332c3232332c3233332c3230322c34302c3231352c3135362c3131342c36342c3231332c35392c3233332c33362c3234372c3132342c3130392c3138312c3230302c3136342c3232302c3230352c32392c3133332c3132302c3232342c3132312c3132362c36362c3235322c37312c3233325d7d2c226c68735f706b223a5b3134352c3139352c3234312c35332c3139392c3133362c33322c3235342c3131362c3132302c3137352c3232332c31382c37352c3134362c35312c3131362c3235332c3137342c3132312c3235342c3134302c3136392c33302c3135312c33332c3134392c3131342c3130322c3132332c3139302c33325d2c227268735f706b223a5b32362c3233332c3137382c3138372c3234342c33382c3138372c3132332c3133382c33312c34352c39382c37302c38322c3232392c39302c3137372c36352c3133332c3135372c39372c3233302c35302c37382c3134362c37302c3230322c3130312c35362c32302c3234372c3231375d7d".to_string()),
            operational_certificate: Some("5b5b5b3230332c3130392c34302c32382c3235312c39342c35322c32342c3231322c3131362c3134392c38302c3138332c3136322c312c36322c352c3133332c35372c3230342c31352c3137322c3134372c38362c3132352c35392c31322c3235332c3130312c3138342c32332c31355d2c322c3132382c5b3133382c3131302c3139322c35302c38362c332c3136382c33342c3137322c31392c39312c3133392c3139302c3134302c31382c3137372c33312c34362c3132322c3130362c3233342c3137372c3130382c3232352c3230372c342c302c35392c3233372c3133352c3130342c39382c3133332c3133312c32392c3231322c3137312c3139342c3234342c3139312c3137392c3131392c34322c37352c3135302c36312c3232362c3132312c35342c3232332c3139332c3133382c3139302c32372c3138322c3135322c35362c32312c3136302c3230372c33352c3233372c3130322c31325d5d2c5b3230372c31322c3136382c3139302c34362c3131362c3139362c3133332c3139362c3233312c3132342c3235302c3134372c33372c3137352c3231312c3234372c3139382c3134302c3133392c3234362c3130342c3132342c3232372c34392c352c3235332c3232382c3130372c39332c3133362c3134345d5d".to_string()),
            kes_period: Some(6),
            stake: 234,
        }
    }

    /// Convert a set of signers into message parts
    pub fn from_signers(signers: Vec<SignerWithStake>) -> Vec<Self> {
        signers.into_iter().map(|signer| signer.into()).collect()
    }
}

impl From<SignerWithStake> for SignerWithStakeMessagePart {
    fn from(value: SignerWithStake) -> Self {
        Self {
            party_id: value.party_id,
            verification_key: value.verification_key.try_into().unwrap(),
            verification_key_signature: value.verification_key_signature,
            operational_certificate: value.operational_certificate,
            kes_period: value.kes_period,
            stake: value.stake,
        }
    }
}
