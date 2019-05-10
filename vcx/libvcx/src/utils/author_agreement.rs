use error::{VcxError, VcxErrorKind, VcxResult};
use serde_json;
use settings;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TxnAuthorAgreementAcceptanceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taa_digest: Option<String>,
    pub acceptance_mechanism_type: String,
    pub time_of_acceptance: u64
}

pub fn set_txn_author_agreement(text: Option<String>,
                                version: Option<String>,
                                taa_digest: Option<String>,
                                acc_mech_type: String,
                                time_of_acceptance: u64) -> VcxResult<()> {
    let meta = TxnAuthorAgreementAcceptanceData {
        text,
        version,
        taa_digest,
        acceptance_mechanism_type: acc_mech_type,
        time_of_acceptance,
    };

    let meta = serde_json::to_string(&meta)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidOption, err))?;

    settings::set_config_value(settings::CONFIG_TXN_AUTHOR_AGREEMENT, &meta);

    Ok(())
}

pub fn get_txn_author_agreement() -> VcxResult<Option<TxnAuthorAgreementAcceptanceData>> {
    match settings::get_config_value(settings::CONFIG_TXN_AUTHOR_AGREEMENT) {
        Ok(value) => {
            let meta: TxnAuthorAgreementAcceptanceData = serde_json::from_str(&value)
                .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidState, err))?;
            Ok(Some(meta))
        }
        Err(_) => Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "indy agreement";
    const VERSION: &str = "1.0.0";
    const ACCEPTANCE_MECHANISM: &str = "acceptance mechanism label 1";
    const TIME_OF_ACCEPTANCE: u64 = 123456789;

    #[test]
    fn set_txn_author_agreement_works() {
        settings::clear_config();

        assert!(settings::get_config_value(settings::CONFIG_TXN_AUTHOR_AGREEMENT).is_err());

        set_txn_author_agreement(Some(TEXT.to_string()),
                                 Some(VERSION.to_string()),
                                 None,
                                 ACCEPTANCE_MECHANISM.to_string(),
                                 TIME_OF_ACCEPTANCE).unwrap();

        assert!(settings::get_config_value(settings::CONFIG_TXN_AUTHOR_AGREEMENT).is_ok());

        settings::clear_config();
    }

    #[test]
    fn get_txn_author_agreement_works() {
        settings::clear_config();

        set_txn_author_agreement(Some(TEXT.to_string()),
                                 Some(VERSION.to_string()),
                                 None,
                                 ACCEPTANCE_MECHANISM.to_string(),
                                 TIME_OF_ACCEPTANCE).unwrap();

        let meta = get_txn_author_agreement().unwrap().unwrap();

        let expected_meta = TxnAuthorAgreementAcceptanceData {
            text: Some(TEXT.to_string()),
            version: Some(VERSION.to_string()),
            taa_digest: None,
            acceptance_mechanism_type: ACCEPTANCE_MECHANISM.to_string(),
            time_of_acceptance: TIME_OF_ACCEPTANCE,
        };

        assert_eq!(expected_meta, meta);

        settings::clear_config();
    }

    #[test]
    fn get_txn_author_agreement_works_for_not_set() {
        settings::clear_config();

        assert!(get_txn_author_agreement().unwrap().is_none());

        settings::clear_config();
    }
}