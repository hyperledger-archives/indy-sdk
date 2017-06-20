package org.hyperledger.indy.sdk.signus;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Future;

import org.hyperledger.indy.sdk.LibSovrin;
import org.hyperledger.indy.sdk.SovrinException;
import org.hyperledger.indy.sdk.SovrinJava;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.SignusJSONParameters.CreateAndStoreMyDidJSONParameter;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.signus.SignusResults.DecryptResult;
import org.hyperledger.indy.sdk.signus.SignusResults.EncryptResult;
import org.hyperledger.indy.sdk.signus.SignusResults.ReplaceKeysResult;
import org.hyperledger.indy.sdk.signus.SignusResults.SignResult;
import org.hyperledger.indy.sdk.signus.SignusResults.StoreTheirDidResult;
import org.hyperledger.indy.sdk.signus.SignusResults.VerifySignatureResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import com.sun.jna.Callback;

/**
 * signus.rs API
 */
public class Signus extends SovrinJava.API {

	private Signus() {

	}

	/*
	 * STATIC METHODS
	 */

	public static Future<CreateAndStoreMyDidResult> createAndStoreMyDid(
			Wallet wallet,
			CreateAndStoreMyDidJSONParameter didJson) throws SovrinException {

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

		int result = LibSovrin.api.sovrin_create_and_store_my_did(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				didJson == null ? null : didJson.toJson(),
				cb);

		checkResult(result);

		return future;
	}

	public static Future<ReplaceKeysResult> replaceKeys(
			Wallet wallet,
			String did,
			String identityJson) throws SovrinException {

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

		int result = LibSovrin.api.sovrin_replace_keys(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				did,
				identityJson,
				cb);

		checkResult(result);

		return future;
	}

	public static Future<StoreTheirDidResult> storeTheirDid(
			Wallet wallet,
			String identityJson) throws SovrinException {

		final CompletableFuture<StoreTheirDidResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				StoreTheirDidResult result = new StoreTheirDidResult();
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibSovrin.api.sovrin_store_their_did(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				identityJson,
				cb);

		checkResult(result);

		return future;
	}

	public static Future<SignResult> sign(
			Wallet wallet,
			String did,
			String msg) throws SovrinException {

		final CompletableFuture<SignResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String signature) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				SignResult result = new SignResult(signature);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibSovrin.api.sovrin_sign(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				did,
				msg,
				cb);

		checkResult(result);

		return future;
	}

	public static Future<VerifySignatureResult> verifySignature(
			Wallet wallet,
			Pool pool,
			String did,
			String signedMsg) throws SovrinException {

		final CompletableFuture<VerifySignatureResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, boolean valid) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				VerifySignatureResult result = new VerifySignatureResult(valid);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();
		int poolHandle = pool.getPoolHandle();

		int result = LibSovrin.api.sovrin_verify_signature(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				poolHandle,
				did,
				signedMsg,
				cb);

		checkResult(result);

		return future;
	}

	public static Future<EncryptResult> encrypt(
			Wallet wallet,
			String did,
			String msg) throws SovrinException {

		final CompletableFuture<EncryptResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String encryptedMsg) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				EncryptResult result = new EncryptResult(encryptedMsg);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibSovrin.api.sovrin_encrypt(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				did,
				msg,
				cb);

		checkResult(result);

		return future;
	}

	public static Future<DecryptResult> decrypt(
			Wallet wallet,
			String did,
			String encryptedMsg) throws SovrinException {

		final CompletableFuture<DecryptResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String decryptedMsg) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				DecryptResult result = new DecryptResult(decryptedMsg);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibSovrin.api.sovrin_decrypt(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				did,
				encryptedMsg,
				cb);

		checkResult(result);

		return future;
	}
}
