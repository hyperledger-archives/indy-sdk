package org.hyperledger.indy.sdk.wallet;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.ErrorCode;
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
	 * STATIC CALLBACKS
	 */

	private static Callback createWalletCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback openWalletCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, int handle) {

			CompletableFuture<Wallet> future = (CompletableFuture<Wallet>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Wallet wallet = new Wallet(handle);

			Wallet result = wallet;
			future.complete(result);
		}
	};

	private static Callback closeWalletCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback deleteWalletCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback walletSetSeqNoForValueCb = new Callback() {

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

	/* IMPLEMENT LATER
	 * public CompletableFuture<...> registerWalletType(
				...) throws IndyException;*/

	public static CompletableFuture<Void> createWallet(
			String poolName,
			String name,
			String xtype,
			String config,
			String credentials) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_create_wallet(
				commandHandle, 
				poolName, 
				name,
				xtype,
				config,
				credentials,
				createWalletCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Wallet> openWallet(
			String name,
			String runtimeConfig,
			String credentials) throws IndyException {

		CompletableFuture<Wallet> future = new CompletableFuture<Wallet> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_open_wallet(
				commandHandle, 
				name,
				runtimeConfig,
				credentials,
				openWalletCb);

		checkResult(result);

		return future;
	}

	private static CompletableFuture<Void> closeWallet(
			Wallet wallet) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void> ();
		int commandHandle = addFuture(future);

		int handle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_close_wallet(
				commandHandle, 
				handle, 
				closeWalletCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> deleteWallet(
			String name,
			String credentials) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_delete_wallet(
				commandHandle, 
				name,
				credentials,
				deleteWalletCb);

		checkResult(result);

		return future;
	}

	private static CompletableFuture<Void> walletSetSeqNoForValue(
			Wallet wallet, 
			String walletKey,
			String configName) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void> ();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_wallet_set_seq_no_for_value(
				commandHandle, 
				walletHandle,
				walletKey, 
				walletSetSeqNoForValueCb);

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
