package org.hyperledger.indy.sdk.signus;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters.CreateAndStoreMyDidJSONParameter;
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
	 * STATIC METHODS
	 */

	public static CompletableFuture<CreateAndStoreMyDidResult> createAndStoreMyDid(
			Wallet wallet,
			CreateAndStoreMyDidJSONParameter didJson) throws IndyException {

		final CompletableFuture<CreateAndStoreMyDidResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String did, String verkey, String pk) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				CreateAndStoreMyDidResult result = new CreateAndStoreMyDidResult(did, verkey, pk);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_create_and_store_my_did(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				didJson == null ? null : didJson.toJson(),
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<ReplaceKeysResult> replaceKeys(
			Wallet wallet,
			String did,
			String identityJson) throws IndyException {

		final CompletableFuture<ReplaceKeysResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String verkey, String pk) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				ReplaceKeysResult result = new ReplaceKeysResult(verkey, pk);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_replace_keys(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				did,
				identityJson,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> storeTheirDid(
			Wallet wallet,
			String identityJson) throws IndyException {

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

		int result = LibIndy.api.indy_store_their_did(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				identityJson,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> sign(
			Wallet wallet,
			String did,
			String msg) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String signature) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = signature;
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_sign(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				did,
				msg,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Boolean> verifySignature(
			Wallet wallet,
			Pool pool,
			String did,
			String signedMsg) throws IndyException {

		final CompletableFuture<Boolean> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, boolean valid) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				Boolean result = Boolean.valueOf(valid);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();
		int poolHandle = pool.getPoolHandle();

		int result = LibIndy.api.indy_verify_signature(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				poolHandle,
				did,
				signedMsg,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> encrypt(
			Wallet wallet,
			String did,
			String msg) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String encryptedMsg) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = encryptedMsg;
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_encrypt(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				did,
				msg,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> decrypt(
			Wallet wallet,
			String did,
			String encryptedMsg) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String decryptedMsg) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = decryptedMsg;
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_decrypt(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				did,
				encryptedMsg,
				cb);

		checkResult(result);

		return future;
	}
}
