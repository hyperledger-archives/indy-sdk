package org.hyperledger.indy.sdk.wallet;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Future;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.wallet.WalletResults.CloseWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.CreateWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.DeleteWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.OpenWalletResult;
import org.hyperledger.indy.sdk.wallet.WalletResults.WalletSetSeqNoForValueResult;

import com.sun.jna.Callback;

/**
 * wallet.rs API
 */
public class Wallet extends IndyJava.API {

	private final int walletHandle;

	private Wallet(int walletHandle) {

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
				...) throws IndyException;*/

	public static Future<CreateWalletResult> createWallet(
			String poolName,
			String name,
			String xtype,
			String config,
			String credentials) throws IndyException {

		final CompletableFuture<CreateWalletResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				CreateWalletResult result = new CreateWalletResult();
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_create_wallet(
				FIXED_COMMAND_HANDLE, 
				poolName, 
				name,
				xtype,
				config,
				credentials,
				cb);

		checkResult(result);

		return future;
	}

	public static Future<OpenWalletResult> openWallet(
			String name,
			String runtimeConfig,
			String credentials) throws IndyException {

		final CompletableFuture<OpenWalletResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Wallet wallet = new Wallet(handle);

				OpenWalletResult result = new OpenWalletResult(wallet);
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_open_wallet(
				FIXED_COMMAND_HANDLE, 
				name,
				runtimeConfig,
				credentials,
				cb);

		checkResult(result);

		return future;
	}

	private static Future<CloseWalletResult> closeWallet(
			Wallet wallet) throws IndyException {

		final CompletableFuture<CloseWalletResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				CloseWalletResult result = new CloseWalletResult();
				future.complete(result);
			}
		};

		int handle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_close_wallet(
				FIXED_COMMAND_HANDLE, 
				handle, 
				cb);

		checkResult(result);

		return future;
	}

	public static Future<DeleteWalletResult> deleteWallet(
			String name,
			String credentials) throws IndyException {

		final CompletableFuture<DeleteWalletResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				DeleteWalletResult result = new DeleteWalletResult();
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_delete_wallet(
				FIXED_COMMAND_HANDLE, 
				name,
				credentials,
				cb);

		checkResult(result);

		return future;
	}

	private static Future<WalletSetSeqNoForValueResult> walletSetSeqNoForValue(
			Wallet wallet, 
			String walletKey,
			String configName) throws IndyException {

		final CompletableFuture<WalletSetSeqNoForValueResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				WalletSetSeqNoForValueResult result = new WalletSetSeqNoForValueResult();
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_wallet_set_seq_no_for_value(
				FIXED_COMMAND_HANDLE, 
				walletHandle,
				walletKey, 
				cb);

		checkResult(result);

		return future;
	}

	/*
	 * INSTANCE METHODS
	 */

	public Future<CloseWalletResult> closeWallet(
			) throws IndyException {

		return closeWallet(this);
	}

	public Future<WalletSetSeqNoForValueResult> walletSetSeqNoForValue(
			String walletKey,
			String configName) throws IndyException {

		return walletSetSeqNoForValue(this, walletKey, configName);
	}
}
