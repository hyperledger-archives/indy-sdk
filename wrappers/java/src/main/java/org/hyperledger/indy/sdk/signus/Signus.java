package org.hyperledger.indy.sdk.signus;

import java.nio.ByteBuffer;
import java.util.Arrays;
import java.util.concurrent.CompletableFuture;

import com.sun.jna.Pointer;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.signus.SignusResults.ReplaceKeysResult;
import org.hyperledger.indy.sdk.signus.SignusResults.EncryptResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import com.sun.jna.Callback;

/**
 * signus.rs API
 */

/**
 * High level wrapper around signus SDK functions.
 */
public class Signus extends IndyJava.API {

	private Signus() {

	}

	/*
	 * STATIC CALLBACKS
	 */

	/**
	 * Callback used when createAndStoreMyDid completes.
	 */
	private static Callback createAndStoreMyDidCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String did, String verkey, String pk) {

			CompletableFuture<CreateAndStoreMyDidResult> future = (CompletableFuture<CreateAndStoreMyDidResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			CreateAndStoreMyDidResult result = new CreateAndStoreMyDidResult(did, verkey, pk);
			future.complete(result);
		}
	};

	/**
	 * Callback used when replaceKeys completes.
	 */
	private static Callback replaceKeysCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String verkey, String pk) {

			CompletableFuture<ReplaceKeysResult> future = (CompletableFuture<ReplaceKeysResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			ReplaceKeysResult result = new ReplaceKeysResult(verkey, pk);
			future.complete(result);
		}
	};

	/**
	 * Callback used when storeTheirDid completes.
	 */
	private static Callback storeTheirDidCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/**
	 * Callback used when sign completes.
	 */
	private static Callback signCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, Pointer signaturePtr, int signatureLen) {

			CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			byte[] signature = new byte[signatureLen];
			signaturePtr.read(0, signature, 0, signatureLen);

			future.complete(signature);
		}
	};

	/**
	 * Callback used when verifySignature completes.
	 */
	private static Callback verifySignatureCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, boolean valid) {

			CompletableFuture<Boolean> future = (CompletableFuture<Boolean>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Boolean result = Boolean.valueOf(valid);
			future.complete(result);
		}
	};

	/**
	 * Callback used when encrypt completes.
	 */
	private static Callback encryptCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, Pointer encryptedMsgPtr, int encryptedMsgLen, Pointer noncePtr, int nonceLen) {

			CompletableFuture<EncryptResult> future = (CompletableFuture<EncryptResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			byte[] encryptedMsg = new byte[encryptedMsgLen];
			encryptedMsgPtr.read(0, encryptedMsg, 0, encryptedMsgLen);

			byte[] nonce = new byte[nonceLen];
			noncePtr.read(0, nonce, 0, nonceLen);

			EncryptResult result = new EncryptResult(encryptedMsg, nonce);
			future.complete(result);
		}
	};

	/**
	 * Callback used when decrypt completes.
	 */
	private static Callback decryptCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, Pointer decryptedMsgPtr, int decryptedMsgLen) {

			CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			byte[] decryptedMsg = new byte[decryptedMsgLen];
			decryptedMsgPtr.read(0, decryptedMsg, 0, decryptedMsgLen);

			future.complete(decryptedMsg);
		}
	};

	/*
	 * STATIC METHODS
	 */

	/**
	 * Creates keys (signing and encryption keys) for a new DID owned by the caller.
	 * 
	 * @param wallet The wallet.
	 * @param didJson Identity information as json.
	 * @return A future that resolves to a CreateAndStoreMyDidResult instance.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<CreateAndStoreMyDidResult> createAndStoreMyDid(
			Wallet wallet,
			String didJson) throws IndyException {

		CompletableFuture<CreateAndStoreMyDidResult> future = new CompletableFuture<CreateAndStoreMyDidResult>();
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

	/**
	 * Generated new signing and encryption keys for an existing DID owned by the caller.
	 * 
	 * @param wallet The wallet.
	 * @param did The DID
	 * @param identityJson identity information as json.
	 * @return A future that resolves to a ReplaceKeysResult instance.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */	
	public static CompletableFuture<ReplaceKeysResult> replaceKeys(
			Wallet wallet,
			String did,
			String identityJson) throws IndyException {

		CompletableFuture<ReplaceKeysResult> future = new CompletableFuture<ReplaceKeysResult>();
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

	/**
	 * Saves their DID for a pairwise connection in a secured Wallet so that it can be used to verify transaction.
	 * 
	 * @param wallet The wallet.
	 * @param identityJson Identity information as json.
	 * @return A future that does not resolve any value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> storeTheirDid(
			Wallet wallet,
			String identityJson) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void>();
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

	/**
	 * Signs a message by a signing key associated with my DID. The DID with a signing key.
	 * 
	 * @param wallet The wallet.
	 * @param did signing DID
	 * @param msg a message to be signed
	 * @return A future that resolves to a a signature string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<byte[]> sign(
			Wallet wallet,
			String did,
			byte[] msg) throws IndyException {

		CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_sign(
				commandHandle, 
				walletHandle, 
				did,
				msg,
				msg.length,
				signCb);

		checkResult(result);

		return future;
	}

	/**
	 * Verify a signature created by a key associated with a DID.
	 * 
	 * @param wallet The wallet.
	 * @param pool The pool.
	 * @param did DID that signed the message
	 * @param msg message
	 * @param signature a signature to be verified
	 * @return A future that resolves to true if signature is valid, otherwise false.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Boolean> verifySignature(
			Wallet wallet,
			Pool pool,
			String did,
			byte[] msg,
			byte[] signature) throws IndyException {

		CompletableFuture<Boolean> future = new CompletableFuture<Boolean>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();
		int poolHandle = pool.getPoolHandle();

		int result = LibIndy.api.indy_verify_signature(
				commandHandle, 
				walletHandle, 
				poolHandle,
				did,
				msg,
				msg.length,
				signature,
				signature.length,
				verifySignatureCb);

		checkResult(result);

		return future;
	}

	/**
	 * Encrypts a message by a public key associated with a DID.
	 * 
	 * @param wallet The wallet.
	 * @param pool The pool.
	 * @param myDid encrypting DID
	 * @param did encrypting DID
	 * @param msg a message to be signed
	 * @return A future that resolves to a JSON string containing an encrypted message and nonce.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<EncryptResult> encrypt(
			Wallet wallet,
			Pool pool,
			String myDid,
			String did,
			byte[] msg) throws IndyException {

		CompletableFuture<EncryptResult> future = new CompletableFuture<EncryptResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();
		int poolHandle = pool.getPoolHandle();

		int result = LibIndy.api.indy_encrypt(
				commandHandle, 
				walletHandle, 
				poolHandle, 
				myDid,
				did,
				msg,
				msg.length,
				encryptCb);

		checkResult(result);

		return future;
	}

	/**
	 * Decrypts a message encrypted by a public key associated with my DID.
	 * 
	 * @param wallet The wallet.
	 * @param myDid DID
	 * @param did DID that signed the message
	 * @param encryptedMsg encrypted message
	 * @param nonce nonce that encrypted message
	 * @return A future that resolves to a JSON string containing the decrypted message.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<byte[]> decrypt(
			Wallet wallet,
			String myDid,
			String did,
			byte[] encryptedMsg,
			byte[] nonce) throws IndyException {

		CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_decrypt(
				commandHandle, 
				walletHandle, 
				myDid,
				did,
				encryptedMsg,
				encryptedMsg.length,
				nonce,
				nonce.length,
				decryptCb);

		checkResult(result);

		return future;
	}
}
