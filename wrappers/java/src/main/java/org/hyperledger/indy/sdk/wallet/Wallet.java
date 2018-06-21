package org.hyperledger.indy.sdk.wallet;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;

import com.sun.jna.Callback;

/**
 * wallet.rs API
 */
/**
 * High level wrapper for wallet SDK functions.
 */
public class Wallet extends IndyJava.API implements AutoCloseable {

	private final int walletHandle;

	private Wallet(int walletHandle) {

		this.walletHandle = walletHandle;
	}

	/**
	 * Gets the handle for the wallet.
	 *
	 * @return The handle for the wallet.
	 */
	public int getWalletHandle() {

		return this.walletHandle;
	}

	/*
	 * STATIC CALLBACKS
	 */

	/**
	 * Callback used when function returning void completes.
	 */
	private static Callback voidCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/**
	 * Callback used when openWallet completes.
	 */
	private static Callback openWalletCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, int handle) {

			CompletableFuture<Wallet> future = (CompletableFuture<Wallet>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Wallet wallet = new Wallet(handle);

			Wallet result = wallet;
			future.complete(result);
		}
	};

	/*
	 * STATIC METHODS
	 */

	private static final List<WalletType> REGISTERED_WALLET_TYPES = Collections.synchronizedList(new ArrayList<WalletType>());

	/**
	 * Registers custom wallet implementation.
	 *
	 * @param xtype Wallet type name.
	 * @param walletType An instance of a WalletType subclass
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if a call to the underlying SDK fails.
	 * @throws InterruptedException Thrown...???
	 */
	public static CompletableFuture<Void> registerWalletType(
		String xtype,
		WalletType walletType) throws IndyException, InterruptedException {

		ParamGuard.notNullOrWhiteSpace(xtype, "xtype");
		ParamGuard.notNull(walletType, "walletType");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		REGISTERED_WALLET_TYPES.add(walletType);

		int result = LibIndy.api.indy_register_wallet_storage( //TODO:FIXME
				commandHandle,
				xtype,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				null,
				voidCb);

		checkResult(result);

		return future;
	}

	/**
	 * Creates a new secure wallet with the given unique name.
	 *
	 * @param poolName Name of the pool that corresponds to this wallet.
	 * @param name Name of the wallet.
	 * @param xtype Type of the wallet. Defaults to 'default'.
	 * @param config Wallet configuration json. List of supported keys are defined by wallet type.
	 * @param credentials Wallet credentials json: {
	 *    "key": <string>
	 * }
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if a call to the underlying SDK fails.
	 */
	public static CompletableFuture<Void> createWallet(
			String poolName,
			String name,
			String xtype,
			String config,
			String credentials) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(poolName, "poolName");
		ParamGuard.notNullOrWhiteSpace(name, "name");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_create_wallet(
				commandHandle,
				poolName,
				name,
				xtype,
				config,
				credentials,
				voidCb);

		checkResult(result);

		return future;
	}

	/**
	 * Opens the wallet with specific name.
	 *
	 * @param name Name of the wallet.
	 * @param runtimeConfig Runtime wallet configuration json. if NULL, then default runtime_config will be used.
	 * @param credentials Wallet credentials json: {
	 *    "key": <string>
	 * }
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if a call to the underlying SDK fails.
	 */
	public static CompletableFuture<Wallet> openWallet(
			String name,
			String runtimeConfig,
			String credentials) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(name, "name");

		CompletableFuture<Wallet> future = new CompletableFuture<Wallet>();
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

	/**
	 * Closes the specified open wallet and frees allocated resources.
	 *
	 * @param wallet The wallet to close.
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if a call to the underlying SDK fails.
	 */
	private static CompletableFuture<Void> closeWallet(
			Wallet wallet) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int handle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_close_wallet(
				commandHandle,
				handle,
				voidCb);

		checkResult(result);

		return future;
	}

	/**
	 * Deletes an existing wallet.
	 *
	 * @param name Name of the wallet to delete.
	 * @param credentials Wallet credentials json: {
	 *    "key": <string>
	 * }
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if a call to the underlying SDK fails.
	 */
	public static CompletableFuture<Void> deleteWallet(
			String name,
			String credentials) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(name, "name");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_delete_wallet(
				commandHandle,
				name,
				credentials,
				voidCb);

		checkResult(result);

		return future;
	}

	/**
	 * Exports opened wallet to the file.
	 *
	 * @param wallet The wallet to export.
	 * @param exportConfigJson: JSON containing settings for input operation.
	 *   {
	 *     "path": path of the file that contains exported wallet content
	 *     "key": passphrase used to export key
	 *   }
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if a call to the underlying SDK fails.
	 */
	public static CompletableFuture<Void> exportWallet(
			Wallet wallet,
			String exportConfigJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNull(exportConfigJson, "exportConfigJson");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int handle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_export_wallet(
				commandHandle,
				handle,
				exportConfigJson,
				voidCb);

		checkResult(result);

		return future;
	}

	/**
	 * Creates a new secure wallet with the given unique name and then imports its content
	 * according to fields provided in import_config
	 * This can be seen as an indy_create_wallet call with additional content import
	 *
	 * @param poolName Name of the pool that corresponds to this wallet.
	 * @param name Name of the wallet.
	 * @param xtype Type of the wallet. Defaults to 'default'.
	 * @param config Wallet configuration json. List of supported keys are defined by wallet type.
	 * @param credentials Wallet credentials json: {
	 *    "key": <string>
	 * }
	 * @param importConfigJson JSON containing settings for input operation: {
	 *     "path": path of the file that contains exported wallet content
	 *     "key": passphrase used to export key
	 *   }
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if a call to the underlying SDK fails.
	 */
	public static CompletableFuture<Void> importWallet(
			String poolName,
			String name,
			String xtype,
			String config,
			String credentials,
			String importConfigJson) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(poolName, "poolName");
		ParamGuard.notNullOrWhiteSpace(name, "name");
		ParamGuard.notNull(importConfigJson, "importConfigJson");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_import_wallet(
				commandHandle,
				poolName,
				name,
				xtype,
				config,
				credentials,
				importConfigJson,
				voidCb);

		checkResult(result);

		return future;
	}

	/*
	 * INSTANCE METHODS
	 */

	/**
	 * Closes the wallet and frees allocated resources.
	 *
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if a call to the underlying SDK fails.
	 */
	public CompletableFuture<Void> closeWallet(
			) throws IndyException {

		return closeWallet(this);
	}

	@Override
	public void close() throws InterruptedException, ExecutionException, IndyException {
		closeWallet().get();
	}
}