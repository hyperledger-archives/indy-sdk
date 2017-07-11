package org.hyperledger.indy.sdk.wallet;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;

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
	 * public CompletableFuture<...> registerWalletType(
				...) throws IndyException;*/

	public static CompletableFuture<Void> createWallet(
			String poolName,
			String name,
			String xtype,
			String config,
			String credentials) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
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

	public static CompletableFuture<Wallet> openWallet(
			String name,
			String runtimeConfig,
			String credentials) throws IndyException {

		final CompletableFuture<Wallet> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, int handle) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Wallet result = new Wallet(handle);
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

	private static CompletableFuture<Void> closeWallet(
			Wallet wallet) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
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

	public static CompletableFuture<Void> deleteWallet(
			String name,
			String credentials) throws IndyException {

		final CompletableFuture<Void> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Void result = null;
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

	private static CompletableFuture<Void> walletSetSeqNoForValue(
			Wallet wallet, 
			String walletKey,
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

	public CompletableFuture<Void> closeWallet(
			) throws IndyException {

		return closeWallet(this);
	}

	public CompletableFuture<Void> walletSetSeqNoForValue(
			String walletKey,
			String configName) throws IndyException {

		return walletSetSeqNoForValue(this, walletKey, configName);
	}
}
