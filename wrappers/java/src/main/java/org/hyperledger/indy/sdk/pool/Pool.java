package org.hyperledger.indy.sdk.pool;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.CreatePoolLedgerConfigJSONParameter;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.OpenPoolLedgerJSONParameter;

import com.sun.jna.Callback;

/**
 * pool.rs API
 */
public class Pool extends IndyJava.API {

	private final int poolHandle;

	private Pool(int poolHandle) {

		this.poolHandle = poolHandle;
	}

	public int getPoolHandle() {

		return this.poolHandle;
	}

	/*
	 * STATIC CALLBACKS
	 */

	private static Callback createPoolLedgerConfigCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback openPoolLedgerCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, int pool_handle) {

			CompletableFuture<Pool> future = (CompletableFuture<Pool>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Pool pool = new Pool(pool_handle);

			Pool result = pool;
			future.complete(result);
		}
	};

	private static Callback refreshPoolLedgerCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback closePoolLedgerCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback deletePoolLedgerConfigCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
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

	public static CompletableFuture<Void> createPoolLedgerConfig(
			String configName,
			CreatePoolLedgerConfigJSONParameter config) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_create_pool_ledger_config(
				commandHandle, 
				configName, 
				config == null ? null : config.toJson(), 
				createPoolLedgerConfigCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Pool> openPoolLedger(
			String configName,
			OpenPoolLedgerJSONParameter config) throws IndyException {

		CompletableFuture<Pool> future = new CompletableFuture<> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_open_pool_ledger(
				commandHandle, 
				configName, 
				config == null ? null : config.toJson(), 
				openPoolLedgerCb);

		checkResult(result);

		return future;
	}

	private static CompletableFuture<Void> refreshPoolLedger(
			Pool pool) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<> ();
		int commandHandle = addFuture(future);

		int handle = pool.getPoolHandle();

		int result = LibIndy.api.indy_refresh_pool_ledger(
				commandHandle, 
				handle, 
				refreshPoolLedgerCb);

		checkResult(result);

		return future;
	}

	private static CompletableFuture<Void> closePoolLedger(
			Pool pool) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<> ();
		int commandHandle = addFuture(future);

		int handle = pool.getPoolHandle();

		int result = LibIndy.api.indy_close_pool_ledger(
				commandHandle, 
				handle, 
				closePoolLedgerCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> deletePoolLedgerConfig(
			String configName) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<> ();
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

	public CompletableFuture<Void> refreshPoolLedger(
			) throws IndyException {

		return refreshPoolLedger(this);
	}

	public CompletableFuture<Void> closePoolLedger(
			) throws IndyException {

		return closePoolLedger(this);
	}
}
