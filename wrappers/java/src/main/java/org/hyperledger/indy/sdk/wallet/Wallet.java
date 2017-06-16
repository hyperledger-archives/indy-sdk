package org.hyperledger.indy.sdk.wallet;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Future;

import org.hyperledger.indy.sdk.LibSovrin;
import org.hyperledger.indy.sdk.SovrinException;
import org.hyperledger.indy.sdk.SovrinJava;
import org.hyperledger.indy.sdk.wallet.WalletResults.CloseWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.CreateWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.DeleteWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.OpenWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.WalletSetSeqNoForValueResult;

import com.sun.jna.Callback;

/**
 * wallet.rs API
 */
public class Wallet extends SovrinJava.API {

	private final int walletHandle;

	Wallet(int walletHandle) {

		this.walletHandle = walletHandle;
	}

	public int getWalletHandle() {
		
		return this.walletHandle;
	}

	/*
	 * STATIC METHODS
	 */

	/* IMPLEMENT LATER
	 * public Future<...> registerWalletType(
				...) throws SovrinException;*/

	public static Future<CreateWalletResult> createWallet(
			String poolName,
			String name,
			String xtype,
			String config,
			String credentials) throws SovrinException {

		final CompletableFuture<CreateWalletResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				CreateWalletResult result = new CreateWalletResult();
				future.complete(result);
			}
		};

		int result = LibSovrin.api.sovrin_create_wallet(
				FIXED_COMMAND_HANDLE, 
				poolName, 
				name,
				xtype,
				config,
				credentials,
				callback);

		checkResult(result);

		return future;
	}

	public static Future<OpenWalletResult> openWallet(
			String name,
			String runtimeConfig,
			String credentials) throws SovrinException {

		final CompletableFuture<OpenWalletResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Wallet wallet = new Wallet(handle);
				
				OpenWalletResult result = new OpenWalletResult(wallet);
				future.complete(result);
			}
		};
		
		int result = LibSovrin.api.sovrin_open_wallet(
				FIXED_COMMAND_HANDLE, 
				name,
				runtimeConfig,
				credentials,
				callback);

		checkResult(result);

		return future;
	}

	private static Future<CloseWalletResult> closeWallet(
			int handle) throws SovrinException {

		final CompletableFuture<CloseWalletResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				CloseWalletResult result = new CloseWalletResult();
				future.complete(result);
			}
		};

		int result = LibSovrin.api.sovrin_close_wallet(
				FIXED_COMMAND_HANDLE, 
				handle, 
				callback);

		checkResult(result);

		return future;
	}

	public static Future<DeleteWalletResult> deleteWallet(
			String name,
			String credentials) throws SovrinException {

		final CompletableFuture<DeleteWalletResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				DeleteWalletResult result = new DeleteWalletResult();
				future.complete(result);
			}
		};

		int result = LibSovrin.api.sovrin_delete_wallet(
				FIXED_COMMAND_HANDLE, 
				name,
				credentials,
				callback);

		checkResult(result);

		return future;
	}

	private static Future<WalletSetSeqNoForValueResult> walletSetSeqNoForValue(
			int walletHandle, 
			String walletKey,
			String configName) throws SovrinException {

		final CompletableFuture<WalletSetSeqNoForValueResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				WalletSetSeqNoForValueResult result = new WalletSetSeqNoForValueResult();
				future.complete(result);
			}
		};

		int result = LibSovrin.api.sovrin_wallet_set_seq_no_for_value(
				FIXED_COMMAND_HANDLE, 
				walletHandle,
				walletKey, 
				callback);

		checkResult(result);

		return future;
	}

	/*
	 * INSTANCE METHODS
	 */

	public Future<CloseWalletResult> closeWallet(
			) throws SovrinException {
		
		return closeWallet(this.walletHandle);
	}

	public Future<WalletSetSeqNoForValueResult> walletSetSeqNoForValue(
			String walletKey,
			String configName) throws SovrinException {
		
		return walletSetSeqNoForValue(this.walletHandle, walletKey, configName);
	}
}
