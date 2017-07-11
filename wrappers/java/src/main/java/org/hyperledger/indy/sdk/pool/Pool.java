package org.hyperledger.indy.sdk.pool;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Future;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.IndyJava;
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
public class Pool extends IndyJava.API {

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
			CreatePoolLedgerConfigJSONParameter config) throws IndyException {

		final CompletableFuture<CreatePoolLedgerConfigResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				CreatePoolLedgerConfigResult result = new CreatePoolLedgerConfigResult();
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_create_pool_ledger_config(
				FIXED_COMMAND_HANDLE, 
				configName, 
				config == null ? null : config.toJson(), 
				callback);

		checkResult(result);

		return future;
	}

	public static Future<OpenPoolLedgerResult> openPoolLedger(
			String configName,
			OpenPoolLedgerJSONParameter config) throws IndyException {

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

		int result = LibIndy.api.indy_open_pool_ledger(
				FIXED_COMMAND_HANDLE, 
				configName, 
				config == null ? null : config.toJson(), 
				callback);

		checkResult(result);

		return future;
	}

	private static Future<RefreshPoolLedgerResult> refreshPoolLedger(
			int handle) throws IndyException {

		final CompletableFuture<RefreshPoolLedgerResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				RefreshPoolLedgerResult result = new RefreshPoolLedgerResult();
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_refresh_pool_ledger(
				FIXED_COMMAND_HANDLE, 
				handle, 
				callback);

		checkResult(result);

		return future;
	}

	private static Future<ClosePoolLedgerResult> closePoolLedger(
			int handle) throws IndyException {

		final CompletableFuture<ClosePoolLedgerResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				ClosePoolLedgerResult result = new ClosePoolLedgerResult();
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_close_pool_ledger(
				FIXED_COMMAND_HANDLE, 
				handle, 
				callback);

		checkResult(result);

		return future;
	}

	public static Future<DeletePoolLedgerConfigResult> deletePoolLedgerConfig(
			String configName) throws IndyException {

		final CompletableFuture<DeletePoolLedgerConfigResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				DeletePoolLedgerConfigResult result = new DeletePoolLedgerConfigResult();
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_delete_pool_ledger_config(
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
			) throws IndyException {

		return refreshPoolLedger(this.poolHandle);
	}

	public Future<ClosePoolLedgerResult> closePoolLedger(
			) throws IndyException {

		return closePoolLedger(this.poolHandle);
	}
}
