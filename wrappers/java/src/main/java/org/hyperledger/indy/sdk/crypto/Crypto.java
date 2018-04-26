package org.hyperledger.indy.sdk.crypto;

import com.sun.jna.Callback;
import com.sun.jna.Pointer;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.crypto.CryptoResults.AuthDecryptResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import java.util.concurrent.CompletableFuture;

/**
 * crypto.rs API
 */

/**
 * High level wrapper around crypto SDK functions.
 */
public class Crypto extends IndyJava.API {

	private Crypto() {

	}

	/*
	 * STATIC CALLBACKS
	 */

	/**
	 * Callback used when createKey completes.
	 */
	private static Callback createKeyCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String verkey) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = verkey;
			future.complete(result);
		}
	};

	/**
	 * Callback used when setKeyMetadata completes.
	 */
	private static Callback setKeyMetadataCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/**
	 * Callback used when getKeyMetadata completes.
	 */
	private static Callback getKeyMetadataCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String metadata) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = metadata;
			future.complete(result);
		}
	};

	/**
	 * Callback used when cryptoSign completes.
	 */
	private static Callback cryptoSignCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, Pointer signature_raw, int signature_len) {

			CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			byte[] result = new byte[signature_len];
			signature_raw.read(0, result, 0, signature_len);
			future.complete(result);
		}
	};

	/**
	 * Callback used when cryptoVerify completes.
	 */
	private static Callback cryptoVerifyCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, boolean valid) {

			CompletableFuture<Boolean> future = (CompletableFuture<Boolean>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Boolean result = Boolean.valueOf(valid);
			future.complete(result);
		}
	};

	/**
	 * Callback used when cryptoBox completes.
	 */
	private static Callback authCrypCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, Pointer encrypted_msg_raw, int encrypted_msg_len, Pointer nonce_raw, int nonce_len) {

			CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			byte[] result = new byte[encrypted_msg_len];
			encrypted_msg_raw.read(0, result, 0, encrypted_msg_len);

			future.complete(result);
		}
	};

	/**
	 * Callback used when cryptoBoxOpen completes.
	 */
	private static Callback authDecryptCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String their_vk, Pointer decrypted_msg_raw, int decrypted_msg_len) {

			CompletableFuture<AuthDecryptResult> future = (CompletableFuture<AuthDecryptResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			byte[] decryptedMsg = new byte[decrypted_msg_len];
			decrypted_msg_raw.read(0, decryptedMsg, 0, decrypted_msg_len);

			AuthDecryptResult result = new AuthDecryptResult(their_vk, decryptedMsg);

			future.complete(result);
		}
	};

	/**
	 * Callback used when cryptoBoxSeal encrypt completes.
	 */
	private static Callback anonCryptCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, Pointer encrypted_msg_raw, int encrypted_msg_len) {

			CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			byte[] encryptedMsg = new byte[encrypted_msg_len];
			encrypted_msg_raw.read(0, encryptedMsg, 0, encrypted_msg_len);

			future.complete(encryptedMsg);
		}
	};

	/**
	 * Callback used when cryptoBoxSealOpen completes.
	 */
	private static Callback anonDecryptCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, Pointer decrypted_msg_raw, int decrypted_msg_len) {

			CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			byte[] result = new byte[decrypted_msg_len];
			decrypted_msg_raw.read(0, result, 0, decrypted_msg_len);
			future.complete(result);
		}
	};

	/*
	 * STATIC METHODS
	 */

	/**
	 * Creates keys pair and stores in the wallet.
	 *
	 * @param wallet  The wallet.
	 * @param keyJson Key information as json.
	 *                {
	 *                  "seed": string, // Optional (if not set random one will be used); Seed information that allows deterministic key creation.
	 *                  "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
	 *                }
	 * @return A future resolving to a verkey
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> createKey(
			Wallet wallet,
			String keyJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNull(keyJson, "keyJson");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_create_key(
				commandHandle,
				walletHandle,
				keyJson,
				createKeyCb);

		checkResult(result);

		return future;
	}

	/**
	 * Saves/replaces the meta information for the giving key in the wallet.
	 *
	 * @param wallet   The wallet.
	 * @param verkey   The key (verkey, key id) to store metadata.
	 * @param metadata The meta information that will be store with the key.
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> setKeyMetadata(
			Wallet wallet,
			String verkey,
			String metadata) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(verkey, "verkey");
		ParamGuard.notNull(metadata, "metadata");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_set_key_metadata(
				commandHandle,
				walletHandle,
				verkey,
				metadata,
				setKeyMetadataCb);

		checkResult(result);

		return future;
	}

	/**
	 * Retrieves the meta information for the giving key in the wallet.
	 *
	 * @param wallet The wallet.
	 * @param verkey The key (verkey, key id) to retrieve metadata.
	 * @return A future resolving to a metadata
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> getKeyMetadata(
			Wallet wallet,
			String verkey) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(verkey, "verkey");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_get_key_metadata(
				commandHandle,
				walletHandle,
				verkey,
				getKeyMetadataCb);

		checkResult(result);

		return future;
	}


	/**
	 * Signs a message with a key.
	 *
	 * Note to use DID keys with this function you can call keyForDid to get key id (verkey) for specific DID.
	 *
	 * @param wallet    The wallet.
	 * @param signerVk  Id (verkey) of my key. The key must be created by calling createKey or createAndStoreMyDid
	 * @param message   The message to be signed
	 * @return A future that resolves to a signature string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<byte[]> cryptoSign(
			Wallet wallet,
			String signerVk,
			byte[] message) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(signerVk, "signerVk");
		ParamGuard.notNull(message, "message");

		CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_crypto_sign(
				commandHandle,
				walletHandle,
				signerVk,
				message,
				message.length,
				cryptoSignCb);

		checkResult(result);

		return future;
	}

	/**
	 * Verify a signature with a verkey.
	 *
	 *  Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey) for specific DID.
	 *
	 * @param signerVk  Verkey of signer of the message
	 * @param message   Message that has been signed
	 * @param signature A signature to be verified
	 * @return A future that resolves to true if signature is valid, otherwise false.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Boolean> cryptoVerify(
			String signerVk,
			byte[] message,
			byte[] signature) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(signerVk, "theirVk");
		ParamGuard.notNull(message, "message");
		ParamGuard.notNull(signature, "signature");

		CompletableFuture<Boolean> future = new CompletableFuture<Boolean>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_crypto_verify(
				commandHandle,
				signerVk,
				message,
				message.length,
				signature,
				signature.length,
				cryptoVerifyCb);

		checkResult(result);

		return future;
	}

	/**
	 * Encrypt a message by authenticated-encryption scheme.
	 *
	 * Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
	 * Using Recipient's public key, Sender can compute a shared secret key.
	 * Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
	 * That shared secret key can be used to verify that the encrypted message was not tampered with,
	 * before eventually decrypting it.
	 *
	 * Recipient only needs Sender's public key, the nonce and the ciphertext to peform decryption.
	 * The nonce doesn't have to be confidential.
	 *
	 * Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
	 * for specific DID.
	 *
	 * @param wallet  The wallet.
	 * @param senderVk    id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did
	 * @param recipientVk id (verkey) of their key
	 * @param message a message to be signed
	 * @return A future that resolves to a encrypted message as an array of bytes.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<byte[]> authCrypt(
			Wallet wallet,
			String senderVk,
			String recipientVk,
			byte[] message) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(senderVk, "myVk");
		ParamGuard.notNullOrWhiteSpace(recipientVk, "theirVk");
		ParamGuard.notNull(message, "message");

		CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_crypto_auth_crypt(
				commandHandle,
				walletHandle,
				senderVk,
				recipientVk,
				message,
				message.length,
				authCrypCb);

		checkResult(result);

		return future;
	}

	/**
	 * Decrypt a message by authenticated-encryption scheme.
	 *
	 * Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
	 * Using Recipient's public key, Sender can compute a shared secret key.
	 * Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
	 * That shared secret key can be used to verify that the encrypted message was not tampered with,
	 * before eventually decrypting it.
	 *
	 * Recipient only needs Sender's public key, the nonce and the ciphertext to peform decryption.
	 * The nonce doesn't have to be confidential.
	 *
	 * Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
	 * for specific DID.
	 *
	 * @param wallet       The wallet.
	 * @param recipientVk  Id (verkey) of my key. The key must be created by calling createKey or createAndStoreMyDid
	 * @param encryptedMsg Encrypted message
	 * @return A future that resolves to a object containing sender verkey and decrypted message.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<AuthDecryptResult> authDecrypt(
			Wallet wallet,
			String recipientVk,
			byte[] encryptedMsg) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(recipientVk, "myVk");
		ParamGuard.notNull(encryptedMsg, "encryptedMsg");

		CompletableFuture<AuthDecryptResult> future = new CompletableFuture<AuthDecryptResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_crypto_auth_decrypt(
				commandHandle,
				walletHandle,
				recipientVk,
				encryptedMsg,
				encryptedMsg.length,
				authDecryptCb);

		checkResult(result);

		return future;
	}

	/**
	 * Encrypts a message by anonymous-encryption scheme.
	 *
	 * Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
	 * Only the Recipient can decrypt these messages, using its private key.
	 * While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
	 *
	 * Note to use DID keys with this function you can call keyForDid to get key id (verkey)
	 * for specific DID.
	 *
	 * @param recipientVk verkey of message recipient
	 * @param message a message to be signed
	 * @return A future that resolves to an encrypted message as an array of bytes.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<byte[]> anonCrypt(
			String recipientVk,
			byte[] message) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(recipientVk, "theirVk");
		ParamGuard.notNull(message, "message");

		CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
		int commandHandle = addFuture(future);
		
		int result = LibIndy.api.indy_crypto_anon_crypt(
				commandHandle,
				recipientVk,
				message,
				message.length,
				anonCryptCb);

		checkResult(result);

		return future;
	}

	/**
	 * Decrypts a message by anonymous-encryption scheme.
	 *
	 * Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
	 * Only the Recipient can decrypt these messages, using its private key.
	 * While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
	 *
	 * Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
	 * for specific DID.
	 *
	 * @param wallet       The wallet.
	 * @param recipientVk  Id (verkey) of my key. The key must be created by calling createKey or createAndStoreMyDid
	 * @param encryptedMsg encrypted message
	 * @return A future that resolves to a decrypted message as an array of bytes.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<byte[]> anonDecrypt(
			Wallet wallet,
			String recipientVk,
			byte[] encryptedMsg) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(recipientVk, "myVk");
		ParamGuard.notNull(encryptedMsg, "encryptedMsg");

		CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_crypto_anon_decrypt(
				commandHandle,
				walletHandle,
				recipientVk,
				encryptedMsg,
				encryptedMsg.length,
				anonDecryptCb);

		checkResult(result);

		return future;
	}
}
