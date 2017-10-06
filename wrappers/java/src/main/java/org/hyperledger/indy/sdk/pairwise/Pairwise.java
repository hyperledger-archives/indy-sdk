package org.hyperledger.indy.sdk.pairwise;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.signus.SignusResults.CreateAndStoreMyDidResult;
import org.hyperledger.indy.sdk.signus.SignusResults.EncryptResult;
import org.hyperledger.indy.sdk.signus.SignusResults.ReplaceKeysStartResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import com.sun.jna.Callback;
import com.sun.jna.Pointer;

/**
 * pairwise.rs API
 */

/**
 * High level wrapper around pairwise SDK functions.
 */
public class Pairwise extends IndyJava.API {

	private Pairwise() {

	}

	/*
	 * STATIC CALLBACKS
	 */

	/**
	 * Callback used when isPairwiseExists completes.
	 */
	private static Callback isPairwiseExistsCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, boolean exists) {

			CompletableFuture<Boolean> future = (CompletableFuture<Boolean>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Boolean result = Boolean.valueOf(exists);
			future.complete(result);
		}
	};

	/**
	 * Callback used when createPairwise completes.
	 */
	private static Callback createPairwiseCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/**
	 * Callback used when listPairwise completes.
	 */
	private static Callback listPairwiseCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String list_pairwise) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = list_pairwise;
			future.complete(result);
		}
	};

	/**
	 * Callback used when getPairwise completes.
	 */
	private static Callback getPairwiseCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String pairwise_info) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = pairwise_info;
			future.complete(result);
		}
	};

	/**
	 * Callback used when setPairwiseMetadata completes.
	 */
	private static Callback setPairwiseMetadataCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
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

	/**
	 * Check if pairwise is exists.
	 *
	 * @param wallet   The wallet.
	 * @param theirDid encrypted DID.
	 * @return A future that resolves to true - if pairwise is exists, false - otherwise.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Boolean> isPairwiseExists(
			Wallet wallet,
			String theirDid) throws IndyException {

		CompletableFuture<Boolean> future = new CompletableFuture<Boolean>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_is_pairwise_exists(
				commandHandle,
				walletHandle,
				theirDid,
				isPairwiseExistsCb);

		checkResult(result);

		return future;
	}

	/**
	 * Creates pairwise.
	 *
	 * @param wallet   The wallet.
	 * @param theirDid encrypted DID
	 * @param myDid    encrypted DID
	 * @param metadata Optional: extra information for pairwise
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> createPairwise(
			Wallet wallet,
			String theirDid,
			String myDid,
			String metadata) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_create_pairwise(
				commandHandle,
				walletHandle,
				theirDid,
				myDid,
				metadata,
				createPairwiseCb);

		checkResult(result);

		return future;
	}

	/**
	 * Get list of saved pairwise.
	 *
	 * @param wallet The wallet.
	 * @return A future that resolves to string value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> listPairwise(
			Wallet wallet) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_list_pairwise(
				commandHandle,
				walletHandle,
				listPairwiseCb);

		checkResult(result);

		return future;
	}

	/**
	 * Gets pairwise information for specific their_did.
	 *
	 * @param wallet   The wallet.
	 * @param theirDid encrypted DID
	 * @return A future that resolves to string value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> getPairwise(
			Wallet wallet,
			String theirDid) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_get_pairwise(
				commandHandle,
				walletHandle,
				theirDid,
				getPairwiseCb);

		checkResult(result);

		return future;
	}

	/**
	 * Save some data in the Wallet for pairwise associated with Did.
	 *
	 * @param wallet   The wallet.
	 * @param theirDid encoded Did
	 * @param metadata some extra information for pairwise
	 * @return A future that resolves no value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> setPairwiseMetadata(
			Wallet wallet,
			String theirDid,
			String metadata) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_set_pairwise_metadata(
				commandHandle,
				walletHandle,
				theirDid,
				metadata,
				setPairwiseMetadataCb);

		checkResult(result);

		return future;
	}
}
