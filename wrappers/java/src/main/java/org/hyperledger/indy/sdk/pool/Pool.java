package org.hyperledger.indy.sdk.pool;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Future;

import org.hyperledger.indy.sdk.LibSovrin;
import org.hyperledger.indy.sdk.SovrinException;
import org.hyperledger.indy.sdk.SovrinJava;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.CreatePoolLedgerConfigJSONParameter;
import org.hyperledger.indy.sdk.pool.PoolJSONParameters.OpenPoolLedgerJSONParameter;
import org.hyperledger.indy.sdk.pool.PoolResults.ClosePoolLedgerResult;
import org.hyperledger.indy.sdk.pool.PoolResults.CreatePoolLedgerConfigResult;
import org.hyperledger.indy.sdk.pool.PoolResults.DeletePoolLedgerConfigResult;
import org.hyperledger.indy.sdk.pool.PoolResults.OpenPoolLedgerResult;
import org.hyperledger.indy.sdk.pool.PoolResults.RefreshPoolLedgerResult;

import com.sun.jna.Callback;

/**
 * pool.rs API
 */
public class Pool extends SovrinJava.API {

	private final int poolHandle;

	Pool(int poolHandle) {

		this.poolHandle = poolHandle;
	}

	public int getPoolHandle() {
		
		return this.poolHandle;
	}
	
	/*
	 * STATIC METHODS
	 */
	
	public static Future<CreatePoolLedgerConfigResult> createPoolLedgerConfig(
			String configName,
			CreatePoolLedgerConfigJSONParameter config) throws SovrinException {

		final CompletableFuture<CreatePoolLedgerConfigResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				CreatePoolLedgerConfigResult result = new CreatePoolLedgerConfigResult();
				future.complete(result);
			}
		};

		int result = LibSovrin.api.sovrin_create_pool_ledger_config(
				FIXED_COMMAND_HANDLE, 
				configName, 
				config == null ? null : config.toJson(), 
				callback);

		checkResult(result);

		return future;
	}

	public static Future<OpenPoolLedgerResult> openPoolLedger(
			String configName,
			OpenPoolLedgerJSONParameter config) throws SovrinException {

		final CompletableFuture<OpenPoolLedgerResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int pool_handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Pool pool = new Pool(pool_handle);

				OpenPoolLedgerResult result = new OpenPoolLedgerResult(pool);
				future.complete(result);
			}
		};

		int result = LibSovrin.api.sovrin_open_pool_ledger(
				FIXED_COMMAND_HANDLE, 
				configName, 
				config == null ? null : config.toJson(), 
				callback);

		checkResult(result);

		return future;
	}

	private static Future<RefreshPoolLedgerResult> refreshPoolLedger(
			int handle) throws SovrinException {

		final CompletableFuture<RefreshPoolLedgerResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				RefreshPoolLedgerResult result = new RefreshPoolLedgerResult();
				future.complete(result);
			}
		};

		int result = LibSovrin.api.sovrin_refresh_pool_ledger(
				FIXED_COMMAND_HANDLE, 
				handle, 
				callback);

		checkResult(result);

		return future;
	}

	private static Future<ClosePoolLedgerResult> closePoolLedger(
			int handle) throws SovrinException {

		final CompletableFuture<ClosePoolLedgerResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				ClosePoolLedgerResult result = new ClosePoolLedgerResult();
				future.complete(result);
			}
		};

		int result = LibSovrin.api.sovrin_refresh_pool_ledger(
				FIXED_COMMAND_HANDLE, 
				handle, 
				callback);

		checkResult(result);

		return future;
	}

	public static Future<DeletePoolLedgerConfigResult> deletePoolLedgerConfig(
			String configName) throws SovrinException {

		final CompletableFuture<DeletePoolLedgerConfigResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				DeletePoolLedgerConfigResult result = new DeletePoolLedgerConfigResult();
				future.complete(result);
			}
		};

		int result = LibSovrin.api.sovrin_delete_pool_ledger_config(
				FIXED_COMMAND_HANDLE, 
				configName, 
				callback);

		checkResult(result);

		return future;
	}

	/*
	 * INSTANCE METHODS
	 */

	public Future<RefreshPoolLedgerResult> refreshPoolLedger(
			) throws SovrinException {

		return refreshPoolLedger(this.poolHandle);
	}

	public Future<ClosePoolLedgerResult> closePoolLedger(
			) throws SovrinException {

		return closePoolLedger(this.poolHandle);
	}
}
