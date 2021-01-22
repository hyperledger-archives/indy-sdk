extern crate threadpool;
extern crate ursa;

use std::env;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::StreamExt;

use crate::commands::blob_storage::BlobStorageController;
use crate::commands::cache::CacheController;
use crate::commands::crypto::CryptoController;
use crate::commands::did::DidController;
use crate::commands::ledger::LedgerController;
use crate::commands::non_secrets::NonSecretsController;
use crate::commands::pairwise::PairwiseController;
//use crate::commands::payments::{PaymentsCommand, PaymentsCommandExecutor}; FIXME:
use crate::commands::pool::PoolController;
use crate::commands::wallet::WalletController;
// use crate::commands::metrics::{MetricsCommand, MetricsCommandExecutor}; FIXME:
use crate::domain::IndyConfig;
use crate::services::anoncreds::AnoncredsService;
use crate::services::blob_storage::BlobStorageService;
use crate::services::crypto::CryptoService;
use crate::services::ledger::LedgerService;
use indy_api_types::errors::prelude::*;
//use crate::services::payments::PaymentsService; FIXME:
use crate::services::pool::{set_freshness_threshold, PoolService};
//use crate::services::metrics::MetricsService; FIXME:
//use crate::services::metrics::command_index::CommandIndex; FIXME:
use indy_wallet::WalletService;

use self::threadpool::ThreadPool;
use anoncreds::{IssuerController, ProverController, VerifierController};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod anoncreds;
pub mod blob_storage;
pub mod crypto;
pub mod did;
pub mod ledger;
pub mod non_secrets;
pub mod pairwise;
pub mod pool;
pub mod wallet;
//pub mod payments;
pub mod cache;
//pub mod metrics;

type BoxedCallbackStringStringSend = Box<dyn Fn(IndyResult<(String, String)>) + Send + Sync>;

pub enum Command {
    Exit,
    //Payments(PaymentsCommand),
    //Metrics(MetricsCommand),
}

pub struct InstrumentedCommand {
    pub enqueue_ts: u128,
    pub command: Command,
}

impl InstrumentedCommand {
    pub fn new(command: Command) -> InstrumentedCommand {
        InstrumentedCommand {
            enqueue_ts: get_cur_time(),
            command,
        }
    }
}

lazy_static! {
    static ref THREADPOOL: Mutex<ThreadPool> = Mutex::new(ThreadPool::new(4));
}

pub fn indy_set_runtime_config(config: IndyConfig) {
    if let Some(crypto_thread_pool_size) = config.crypto_thread_pool_size {
        THREADPOOL
            .lock()
            .unwrap()
            .set_num_threads(crypto_thread_pool_size);
    }

    match config.collect_backtrace {
        Some(true) => env::set_var("RUST_BACKTRACE", "1"),
        Some(false) => env::set_var("RUST_BACKTRACE", "0"),
        _ => {}
    }

    if let Some(threshold) = config.freshness_threshold {
        set_freshness_threshold(threshold);
    }
}

fn get_cur_time() -> u128 {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time has gone backwards");

    since_epoch.as_millis()
}

pub(crate) struct Locator {
    pub(crate) issuer_command_cxecutor: Arc<IssuerController>,
    pub(crate) prover_command_cxecutor: Arc<ProverController>,
    pub(crate) verifier_command_cxecutor: Arc<VerifierController>,
    pub(crate) crypto_command_executor: Arc<CryptoController>,
    pub(crate) ledger_command_executor: Arc<LedgerController>,
    pub(crate) pool_command_executor: Arc<PoolController>,
    pub(crate) did_command_executor: Arc<DidController>,
    pub(crate) wallet_command_executor: Arc<WalletController>,
    pub(crate) pairwise_command_executor: Arc<PairwiseController>,
    pub(crate) blob_storage_command_executor: Arc<BlobStorageController>,
    pub(crate) non_secret_command_executor: Arc<NonSecretsController>,
    pub(crate) cache_command_executor: Arc<CacheController>,
    pub(crate) executor: futures::executor::ThreadPool,
}

// Global (lazy inited) instance of CommandExecutor
lazy_static! {
    static ref LOCATOR: Arc<Locator> = Arc::new(Locator::new());
}

impl Locator {
    pub fn instance() -> Arc<Locator> {
        LOCATOR.clone()
    }

    fn new() -> Locator {
        let executor = futures::executor::ThreadPool::new().unwrap();

        let anoncreds_service = Arc::new(AnoncredsService::new());
        let blob_storage_service = Arc::new(BlobStorageService::new());
        let crypto_service = Arc::new(CryptoService::new());
        let ledger_service = Arc::new(LedgerService::new());
        //let payments_service = Arc::new(PaymentsService::new());
        let pool_service = Arc::new(PoolService::new());
        let wallet_service = Arc::new(WalletService::new());
        //let metrics_service = Arc::new(MetricsService::new());

        let issuer_command_cxecutor = Arc::new(IssuerController::new(
            anoncreds_service.clone(),
            pool_service.clone(),
            blob_storage_service.clone(),
            wallet_service.clone(),
            crypto_service.clone(),
        ));

        let prover_command_cxecutor = Arc::new(ProverController::new(
            anoncreds_service.clone(),
            wallet_service.clone(),
            crypto_service.clone(),
            blob_storage_service.clone(),
        ));

        let verifier_command_cxecutor =
            Arc::new(VerifierController::new(anoncreds_service.clone()));

        let crypto_command_executor = Arc::new(CryptoController::new(
            wallet_service.clone(),
            crypto_service.clone(),
        ));

        let ledger_command_executor = Arc::new(LedgerController::new(
            pool_service.clone(),
            crypto_service.clone(),
            wallet_service.clone(),
            ledger_service.clone(),
        ));

        let pool_command_executor = Arc::new(PoolController::new(pool_service.clone()));

        let did_command_executor = Arc::new(DidController::new(
            wallet_service.clone(),
            crypto_service.clone(),
            ledger_service.clone(),
            pool_service.clone(),
        ));

        let wallet_command_executor = Arc::new(WalletController::new(
            wallet_service.clone(),
            crypto_service.clone(),
        ));

        let pairwise_command_executor =
            Arc::new(PairwiseController::new(wallet_service.clone()));

        let blob_storage_command_executor = Arc::new(BlobStorageController::new(
            blob_storage_service.clone(),
        ));

        let non_secret_command_executor =
            Arc::new(NonSecretsController::new(wallet_service.clone()));

        // FIXME: let payments_command_executor = Arc::new(PaymentsCommandExecutor::new(payments_service.clone(), wallet_service.clone(), crypto_service.clone(), ledger_service.clone()));

        let cache_command_executor = Arc::new(CacheController::new(
            crypto_service.clone(),
            ledger_service.clone(),
            pool_service.clone(),
            wallet_service.clone(),
        ));

        // FIXME: let metrics_command_executor = Arc::new(MetricsCommandExecutor::new(wallet_service.clone(), metrics_service.clone()));

        std::panic::set_hook(Box::new(|pi| {
            error!("Custom panic hook");
            error!("Custom panic hook: {:?}", pi);
            let bt = backtrace::Backtrace::new();
            error!("Custom panic hook: {:?}", bt);
        }));

        Locator {
            issuer_command_cxecutor: issuer_command_cxecutor.clone(),
            prover_command_cxecutor: prover_command_cxecutor.clone(),
            verifier_command_cxecutor: verifier_command_cxecutor.clone(),
            crypto_command_executor: crypto_command_executor,
            ledger_command_executor: ledger_command_executor.clone(),
            pool_command_executor: pool_command_executor.clone(),
            did_command_executor: did_command_executor.clone(),
            wallet_command_executor: wallet_command_executor.clone(),
            pairwise_command_executor: pairwise_command_executor.clone(),
            blob_storage_command_executor: blob_storage_command_executor.clone(),
            non_secret_command_executor: non_secret_command_executor.clone(),
            cache_command_executor: cache_command_executor.clone(),
            executor: executor.clone(),
        }
    }
}

impl Drop for Locator {
    fn drop(&mut self) {
        //TODO: fix drop, we need to finish the executor (if it is needed)
        info!(target: "command_executor", "Drop started");
//        self.send(Command::Exit).unwrap();
//        self.sender.disconnect();
        // Option worker type and this kludge is workaround for rust
//        self.worker.take().unwrap().join().unwrap();
        info!(target: "command_executor", "Drop finished");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_executor_can_be_created() {
        let _command_executor = Locator::new();
        assert!(true, "No crashes on CommandExecutor::new");
    }

    #[test]
    fn command_executor_can_be_dropped() {
        fn drop_test() {
            let _command_executor = Locator::new();
        }

        drop_test();
        assert!(true, "No crashes on CommandExecutor::drop");
    }

    #[test]
    fn command_executor_can_get_instance() {
        let ref _command_executor: Locator = *Locator::instance();
        // Deadlock if another one instance will be requested (try to uncomment the next line)
        // let ref other_ce: CommandExecutor = *CommandExecutor::instance();
    }
}
