package org.hyperledger.indy.sdk.signus;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.signus.SignusResults.ReplaceKeysResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import com.sun.jna.Callback;

/**
 * signus.rs API
 */
public class Signus extends IndyJava.API {

	private Signus() {

	}

	/*
	 * STATIC CALLBACKS
	 */

	private static Callback createAndStoreMyDidCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String did, String verkey, String pk) {

			CompletableFuture<CreateAndStoreMyDidResult> future = (CompletableFuture<CreateAndStoreMyDidResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			CreateAndStoreMyDidResult result = new CreateAndStoreMyDidResult(did, verkey, pk);
			future.complete(result);
		}
	};

	private static Callback replaceKeysCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String verkey, String pk) {

			CompletableFuture<ReplaceKeysResult> future = (CompletableFuture<ReplaceKeysResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			ReplaceKeysResult result = new ReplaceKeysResult(verkey, pk);
			future.complete(result);
		}
	};

	private static Callback storeTheirDidCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback signCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String signature) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = signature;
			future.complete(result);
		}
	};

	private static Callback verifySignatureCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, boolean valid) {

			CompletableFuture<Boolean> future = (CompletableFuture<Boolean>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Boolean result = Boolean.valueOf(valid);
			future.complete(result);
		}
	};

	private static Callback encryptCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String encryptedMsg) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = encryptedMsg;
			future.complete(result);
		}
	};

	private static Callback decryptCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String decryptedMsg) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = decryptedMsg;
			future.complete(result);
		}
	};

	/*
	 * STATIC METHODS
	 */

	public static CompletableFuture<CreateAndStoreMyDidResult> createAndStoreMyDid(
			Wallet wallet,
			String didJson) throws IndyException {

		CompletableFuture<CreateAndStoreMyDidResult> future = new CompletableFuture<CreateAndStoreMyDidResult> ();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_create_and_store_my_did(
				commandHandle, 
				walletHandle, 
				didJson,
				createAndStoreMyDidCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<ReplaceKeysResult> replaceKeys(
			Wallet wallet,
			String did,
			String identityJson) throws IndyException {

		CompletableFuture<ReplaceKeysResult> future = new CompletableFuture<ReplaceKeysResult> ();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_replace_keys(
				commandHandle, 
				walletHandle, 
				did,
				identityJson,
				replaceKeysCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> storeTheirDid(
			Wallet wallet,
			String identityJson) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void> ();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_store_their_did(
				commandHandle, 
				walletHandle, 
				identityJson,
				storeTheirDidCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> sign(
			Wallet wallet,
			String did,
			String msg) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_sign(
				commandHandle, 
				walletHandle, 
				did,
				msg,
				signCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Boolean> verifySignature(
			Wallet wallet,
			Pool pool,
			String did,
			String signedMsg) throws IndyException {

		CompletableFuture<Boolean> future = new CompletableFuture<Boolean> ();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();
		int poolHandle = pool.getPoolHandle();

		int result = LibIndy.api.indy_verify_signature(
				commandHandle, 
				walletHandle, 
				poolHandle,
				did,
				signedMsg,
				verifySignatureCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> encrypt(
			Wallet wallet,
			String did,
			String msg) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_encrypt(
				commandHandle, 
				walletHandle, 
				did,
				msg,
				encryptCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> decrypt(
			Wallet wallet,
			String did,
			String encryptedMsg) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_decrypt(
				commandHandle, 
				walletHandle, 
				did,
				encryptedMsg,
				decryptCb);

		checkResult(result);

		return future;
	}
}
