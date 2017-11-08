package org.hyperledger.indy.sdk.agent;

import java.util.concurrent.CompletableFuture;

import com.sun.jna.Pointer;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.agent.AgentResults.ParseMsgResult;

import com.sun.jna.Callback;

/**
 * agent.rs API
 */

/**
 * Agent related functionality.
 */
public class Agent extends IndyJava.API {

	private Agent() {

	}

	/*
	 * STATIC CALLBACKS
	 */

	/**
	 * Callback used when prepMsg completes.
	 */
	private static Callback prepMsgCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, Pointer encrypted_msg, int encrypted_len) {

			CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			byte[] result = new byte[encrypted_len];
			encrypted_msg.read(0, result, 0, encrypted_len);
			future.complete(result);
		}
	};

	/**
	 * Callback used when prepAnonymousMsg completes.
	 */
	private static Callback prepAnonymousMsgCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, Pointer encrypted_msg, int encrypted_len) {

			CompletableFuture<byte[]> future = (CompletableFuture<byte[]>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			byte[] result = new byte[encrypted_len];
			encrypted_msg.read(0, result, 0, encrypted_len);
			future.complete(result);
		}
	};

	/**
	 * Callback used when parseMsg completes.
	 */
	private static Callback parseMsgCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String senderKey, Pointer msg_data, int msg_len) {

			byte[] msg = new byte[msg_len];
			msg_data.read(0, msg, 0, msg_len);

			CompletableFuture<ParseMsgResult> future = (CompletableFuture<ParseMsgResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			ParseMsgResult result = new ParseMsgResult(senderKey, msg);
			future.complete(result);


		}
	};

	/*
	 * STATIC METHODS
	 */

	/**
	 * @param wallet       The wallet.
	 * @param senderKey
	 * @param recipientKey
	 * @param message      a message to be encrypted
	 * @return A future that resolves to an encrypted message.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<byte[]> prepMsg(
			Wallet wallet,
			String senderKey,
			String recipientKey,
			byte[] message) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(senderKey, "senderKey");
		ParamGuard.notNullOrWhiteSpace(recipientKey, "recipientKey");
		ParamGuard.notNull(message, "message");

		CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prep_msg(
				commandHandle,
				walletHandle,
				senderKey,
				recipientKey,
				message,
				message.length,
				prepMsgCb);

		checkResult(result);

		return future;
	}

	/**
	 * @param recipientKey
	 * @param message      a message to be encrypted
	 * @return A future that resolves to an encrypted message.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<byte[]> prepAnonymousMsg(
			String recipientKey,
			byte[] message) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(recipientKey, "recipientKey");
		ParamGuard.notNull(message, "message");

		CompletableFuture<byte[]> future = new CompletableFuture<byte[]>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_prep_anonymous_msg(
				commandHandle,
				recipientKey,
				message,
				message.length,
				prepMsgCb);

		checkResult(result);

		return future;
	}

	/**
	 * @param wallet       The wallet.
	 * @param recipientKey
	 * @param encryptedMsg encrypted message
	 * @return A future that resolves to senderKey and message.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<ParseMsgResult> parseMsg(
			Wallet wallet,
			String recipientKey,
			byte[] encryptedMsg) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(recipientKey, "recipientKey");
		ParamGuard.notNull(encryptedMsg, "encryptedMsg");

		CompletableFuture<ParseMsgResult> future = new CompletableFuture<ParseMsgResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_parse_msg(
				commandHandle,
				walletHandle,
				recipientKey,
				encryptedMsg,
				encryptedMsg.length,
				parseMsgCb);

		checkResult(result);

		return future;
	}
}
