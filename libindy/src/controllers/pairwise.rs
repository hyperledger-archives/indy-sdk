use std::{collections::HashMap, sync::Arc};

use indy_api_types::{errors::prelude::*, WalletHandle};
use indy_wallet::{RecordOptions, WalletService};
use log::trace;

use crate::domain::{
    crypto::did::{Did, DidValue, TheirDid},
    pairwise::{Pairwise, PairwiseInfo},
};

pub(crate) struct PairwiseController {
    wallet_service: Arc<WalletService>,
}

impl PairwiseController {
    pub(crate) fn new(wallet_service: Arc<WalletService>) -> PairwiseController {
        PairwiseController { wallet_service }
    }

    pub(crate) async fn pairwise_exists(
        &self,
        wallet_handle: WalletHandle,
        their_did: DidValue,
    ) -> IndyResult<bool> {
        trace!(
            "pairwise_exists > wallet_handle {:?} their_did {:?}",
            wallet_handle, their_did
        );

        let exists = self
            .wallet_service
            .record_exists::<Pairwise>(wallet_handle, &their_did.0)
            .await?;

        let res = Ok(exists);
        trace!("pairwise_exists < {:?}", res);
        res
    }

    pub(crate) async fn create_pairwise(
        &self,
        wallet_handle: WalletHandle,
        their_did: DidValue,
        my_did: DidValue,
        metadata: Option<String>,
    ) -> IndyResult<()> {
        trace!(
            "create_pairwise > wallet_handle {:?} \
                their_did {:?} my_did {:?} metadata {:?}",
            wallet_handle, their_did, my_did, metadata
        );

        self.wallet_service
            .get_indy_record::<Did>(wallet_handle, &my_did.0, &RecordOptions::id())
            .await?;

        self.wallet_service
            .get_indy_record::<TheirDid>(wallet_handle, &their_did.0, &RecordOptions::id())
            .await?;

        let pairwise = Pairwise {
            my_did,
            their_did,
            metadata,
        };

        self.wallet_service
            .add_indy_object(
                wallet_handle,
                &pairwise.their_did.0,
                &pairwise,
                &HashMap::new(),
            )
            .await?;

        let res = Ok(());
        trace!("create_pairwise < {:?}", res);
        res
    }

    pub(crate) async fn list_pairwise(&self, wallet_handle: WalletHandle) -> IndyResult<String> {
        trace!("list_pairwise > wallet_handle {:?}", wallet_handle);

        let mut search = self
            .wallet_service
            .search_indy_records::<Pairwise>(wallet_handle, "{}", &RecordOptions::id_value())
            .await?;

        let mut pairwises: Vec<String> = Vec::new();

        while let Some(pairwise) = search.fetch_next_record().await? {
            let pairwise = pairwise
                .get_value()
                .ok_or_else(|| {
                    err_msg(
                        IndyErrorKind::InvalidState,
                        format!("No value for pairwise {}", pairwise.get_id()),
                    )
                })?
                .to_string();

            pairwises.push(pairwise);
        }

        let pairwises = serde_json::to_string(&pairwises)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize pairwise list")?;

        let res = Ok(pairwises);
        trace!("list_pairwise < {:?}", res);
        res
    }

    pub(crate) async fn get_pairwise(
        &self,
        wallet_handle: WalletHandle,
        their_did: DidValue,
    ) -> IndyResult<String> {
        trace!(
            "get_pairwise > wallet_handle {:?} their_did {:?}",
            wallet_handle, their_did
        );

        let pairwise_info = PairwiseInfo::from(
            self.wallet_service
                .get_indy_object::<Pairwise>(
                    wallet_handle,
                    &their_did.0,
                    &RecordOptions::id_value(),
                )
                .await?,
        );

        let res = serde_json::to_string(&pairwise_info)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize PairwiseInfo")?;

        let res = Ok(res);
        trace!("get_pairwise < {:?}", res);
        res
    }

    pub(crate) async fn set_pairwise_metadata(
        &self,
        wallet_handle: WalletHandle,
        their_did: DidValue,
        metadata: Option<String>,
    ) -> IndyResult<()> {
        trace!(
            "set_pairwise_metadata > wallet_handle {:?} their_did {:?} metadata {:?}",
            wallet_handle, their_did, metadata
        );

        let mut pairwise: Pairwise = self
            .wallet_service
            .get_indy_object(wallet_handle, &their_did.0, &RecordOptions::id_value())
            .await?;

        pairwise.metadata = metadata;

        self.wallet_service
            .update_indy_object(wallet_handle, &their_did.0, &pairwise)
            .await?;

        let res = Ok(());
        trace!("set_pairwise_metadata <<<");
        res
    }
}
