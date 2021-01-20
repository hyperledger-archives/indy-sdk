use std::sync::Arc;

use crate::domain::ledger::request::ProtocolVersion;
use crate::domain::pool::{PoolConfig, PoolOpenConfig};
use indy_api_types::errors::prelude::*;
use crate::services::pool::PoolService;
use indy_api_types::PoolHandle;

pub struct PoolCommandExecutor {
    pool_service:Arc<PoolService>,
}

impl PoolCommandExecutor {
    pub fn new(pool_service:Arc<PoolService>) -> PoolCommandExecutor {
        PoolCommandExecutor {
            pool_service,
        }
    }

    pub(crate) fn create(&self, name: String, config: Option<PoolConfig>) -> IndyResult<()> {
        trace!("create > name {:?} config {:?}", name, config);

        self.pool_service.create(&name, config)?;

        trace!("create < res ()");

        Ok(())
    }

    pub(crate) async fn delete(&self, name: String) -> IndyResult<()> {
        trace!("delete > name {:?}", name);

        self.pool_service.delete(&name).await?;

        trace!("delete < res ()");

        Ok(())
    }

    pub(crate) async fn open(&self, name: String, config: Option<PoolOpenConfig>) -> IndyResult<PoolHandle> {
        trace!("open > name {:?} config {:?}", name, config);

        let result = self
            .pool_service
            .open(name, config)
            .await;

        trace!("open < res {:?}", result);

        result
    }

    pub(crate) fn list(&self) -> IndyResult<String> {
        trace!("list > ");

        let res = self.pool_service
            .list()
            .and_then(|pools| ::serde_json::to_string(&pools)
            .to_indy(IndyErrorKind::InvalidState, "Can't serialize pools list"))?;

        trace!("list < res: {:?}", res);

        Ok(res)
    }

    pub(crate) async fn close(&self, pool_handle: PoolHandle) -> IndyResult<()> {
        trace!("close > handle {:?}", pool_handle);

        let result = self
            .pool_service
            .close(pool_handle)
            .await?;

        trace!("close < ()");

        Ok(())
    }

    pub(crate) async fn refresh(&self, handle: PoolHandle) -> IndyResult<()> {
        trace!("refresh > handle {:?}", handle);

        let result = self
            .pool_service
            .refresh(handle)
            .await?;

        trace!("refresh < ()");

        Ok(())
    }

    pub(crate) fn set_protocol_version(&self, version: usize) -> IndyResult<()> {
        trace!("set_protocol_version > version {:?}", version);

        if version != 1 && version != 2 {
            return Err(err_msg(IndyErrorKind::PoolIncompatibleProtocolVersion, format!("Unsupported Protocol version: {}", version)));
        }

        ProtocolVersion::set(version);

        trace!("set_protocol_version < ()");

        Ok(())
    }
}
