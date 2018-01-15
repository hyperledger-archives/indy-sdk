package org.hyperledger.indy.sdk.pool;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;

import com.sun.jna.Callback;

/**
 * pool.rs API
 */

/**
 * High level wrapper around SDK Pool functionality.
 */
public class Pool extends IndyJava.API implements AutoCloseable {

	private final int poolHandle;

	private Pool(int poolHandle) {

		this.poolHandle = poolHandle;
	}

	/**
	 * Gets the handle for the pool instance.
	 * 
	 * @return The handle for the pool instance.
	 */
	public int getPoolHandle() {

		return this.poolHandle;
	}

	/*
	 * STATIC CALLBACKS
	 */

	/**
	 * Callback used when createPoolLedgerConfig completes.
	 */
	private static Callback createPoolLedgerConfigCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/**
	 * Callback used when openPoolLedger completes.
	 */
	private static Callback openPoolLedgerCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, int pool_handle) {

			CompletableFuture<Pool> future = (CompletableFuture<Pool>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Pool pool = new Pool(pool_handle);

			Pool result = pool;
			future.complete(result);
		}
	};

	/**
	 * Callback used when refreshPoolLedger completes.
	 */
	private static Callback refreshPoolLedgerCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/**
	 * Callback used when closePoolLedger completes.
	 */
	private static Callback closePoolLedgerCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/**
	 * Callback used when deletePoolLedgerConfig completes.
	 */
	private static Callback deletePoolLedgerConfigCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};
	
	/*
	 * STATIC METHODS
	 */

	/**
	 * Creates a new local pool ledger configuration that can be used later to connect pool nodes.
	 * 
	 * @param configName Name of the pool ledger configuration.
	 * @param config Pool configuration json. if NULL, then default config will be used.
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> createPoolLedgerConfig(
			String configName,
			String config) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(configName, "configName");		
		
		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_create_pool_ledger_config(
				commandHandle, 
				configName, 
				config, 
				createPoolLedgerConfigCb);

		checkResult(result);

		return future;
	}

	/**
	 * Opens pool ledger and performs connecting to pool nodes.
	 * 
	 * @param configName Name of the pool ledger configuration.
	 * @param config Runtime pool configuration json. If NULL, then default config will be used.
	 * @return A future that resolves to an opened Pool instance.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Pool> openPoolLedger(
			String configName,
			String config) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(configName, "configName");	
		
		CompletableFuture<Pool> future = new CompletableFuture<Pool>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_open_pool_ledger(
				commandHandle, 
				configName, 
				config, 
				openPoolLedgerCb);

		checkResult(result);

		return future;
	}

	/**
	 * Refreshes a local copy of a pool ledger and updates pool nodes connections.
	 * 
	 * @param pool The pool to refresh.
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	private static CompletableFuture<Void> refreshPoolLedger(
			Pool pool) throws IndyException {

		ParamGuard.notNull(pool, "pool");	
		
		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int handle = pool.getPoolHandle();

		int result = LibIndy.api.indy_refresh_pool_ledger(
				commandHandle, 
				handle, 
				refreshPoolLedgerCb);

		checkResult(result);

		return future;
	}

	/**
	 * Closes opened pool ledger, opened nodes connections and frees allocated resources.
	 * 
	 * @param pool The pool to close.
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	private static CompletableFuture<Void> closePoolLedger(
			Pool pool) throws IndyException {

		ParamGuard.notNull(pool, "pool");	
		
		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int handle = pool.getPoolHandle();

		int result = LibIndy.api.indy_close_pool_ledger(
				commandHandle, 
				handle, 
				closePoolLedgerCb);

		checkResult(result);

		return future;
	}

	/**
	 * Deletes created pool ledger configuration.
	 * 
	 * @param configName Name of the pool ledger configuration to delete.
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> deletePoolLedgerConfig(
			String configName) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(configName, "configName");	
		
		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_delete_pool_ledger_config(
				commandHandle, 
				configName, 
				deletePoolLedgerConfigCb);

		checkResult(result);

		return future;
	}

	/*
	 * INSTANCE METHODS
	 */

	/**
	 * Refreshes a local copy of a pool ledger and updates pool nodes connections.
	 * 
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public CompletableFuture<Void> refreshPoolLedger(
			) throws IndyException {

		return refreshPoolLedger(this);
	}

	/**
	 * Closes opened pool ledger, opened nodes connections and frees allocated resources.
	 * 
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public CompletableFuture<Void> closePoolLedger(
			) throws IndyException {

		return closePoolLedger(this);
	}

	@Override
	public void close() throws InterruptedException, ExecutionException, IndyException {
		closePoolLedger().get();
	}
}
