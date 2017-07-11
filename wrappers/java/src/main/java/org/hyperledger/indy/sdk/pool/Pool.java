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
	 * STATIC METHODS
	 */

	public static CompletableFuture<Void> createPoolLedgerConfig(
			String configName,
			CreatePoolLedgerConfigJSONParameter config) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_create_pool_ledger_config(
				FIXED_COMMAND_HANDLE, 
				configName, 
				config == null ? null : config.toJson(), 
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Pool> openPoolLedger(
			String configName,
			OpenPoolLedgerJSONParameter config) throws IndyException {

		final CompletableFuture<Pool> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int pool_handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Pool result = new Pool(pool_handle);
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_open_pool_ledger(
				FIXED_COMMAND_HANDLE, 
				configName, 
				config == null ? null : config.toJson(), 
				cb);

		checkResult(result);

		return future;
	}

	private static CompletableFuture<Void> refreshPoolLedger(
			Pool pool) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
				future.complete(result);
			}
		};

		int handle = pool.getPoolHandle();

		int result = LibIndy.api.indy_refresh_pool_ledger(
				FIXED_COMMAND_HANDLE, 
				handle, 
				cb);

		checkResult(result);

		return future;
	}

	private static CompletableFuture<Void> closePoolLedger(
			Pool pool) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
				future.complete(result);
			}
		};

		int handle = pool.getPoolHandle();

		int result = LibIndy.api.indy_close_pool_ledger(
				FIXED_COMMAND_HANDLE, 
				handle, 
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> deletePoolLedgerConfig(
			String configName) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_delete_pool_ledger_config(
				FIXED_COMMAND_HANDLE, 
				configName, 
				cb);

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
