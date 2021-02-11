use std::sync::Arc;

use indy_api_types::{errors::prelude::*, PoolHandle};

use crate::{
    domain::{
        ledger::request::ProtocolVersion,
        pool::{PoolConfig, PoolOpenConfig},
    },
    services::PoolService,
};

pub(crate) struct PoolController {
    pool_service: Arc<PoolService>,
}

impl PoolController {
    pub fn new(pool_service: Arc<PoolService>) -> PoolController {
        PoolController { pool_service }
    }

    pub(crate) fn create(&self, name: String, config: Option<PoolConfig>) -> IndyResult<()> {
        trace!("create > name {:?} config {:?}", name, config);

        self.pool_service.create(&name, config)?;

        let res = Ok(());
        trace!("create < {:?}", res);
        res
    }

    pub(crate) async fn delete(&self, name: String) -> IndyResult<()> {
        trace!("delete > name {:?}", name);

        self.pool_service.delete(&name).await?;

        let res = Ok(());
        trace!("delete < {:?}", res);
        res
    }

    pub(crate) async fn open(
        &self,
        name: String,
        config: Option<PoolOpenConfig>,
    ) -> IndyResult<PoolHandle> {
        trace!("open > name {:?} config {:?}", name, config);

        let handle = self.pool_service.open(name, config).await?;

        let res = Ok(handle);
        trace!("open < {:?}", res);
        res
    }

    pub(crate) fn list(&self) -> IndyResult<String> {
        trace!("list > ");

        let pools = self.pool_service.list()?;

        let pools = serde_json::to_string(&pools)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize pools list")?;

        let res = Ok(pools);
        trace!("list < {:?}", res);
        res
    }

    pub(crate) async fn close(&self, pool_handle: PoolHandle) -> IndyResult<()> {
        trace!("close > handle {:?}", pool_handle);

        self.pool_service.close(pool_handle).await?;

        let res = Ok(());
        trace!("close < {:?}", res);
        res
    }

    pub(crate) async fn refresh(&self, handle: PoolHandle) -> IndyResult<()> {
        trace!("refresh > handle {:?}", handle);

        self.pool_service.refresh(handle).await?;

        let res = Ok(());
        trace!("refresh < {:?}", res);
        res
    }

    pub(crate) fn set_protocol_version(&self, version: usize) -> IndyResult<()> {
        trace!("set_protocol_version > version {:?}", version);

        if version != 1 && version != 2 {
            Err(err_msg(
                IndyErrorKind::PoolIncompatibleProtocolVersion,
                format!("Unsupported Protocol version: {}", version),
            ))?;
        }

        ProtocolVersion::set(version);

        let res = Ok(());
        trace!("set_protocol_version < {:?}", res);
        res
    }
}
