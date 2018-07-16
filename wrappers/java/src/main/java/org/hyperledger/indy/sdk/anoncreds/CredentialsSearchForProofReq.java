package org.hyperledger.indy.sdk.anoncreds;

import com.sun.jna.Callback;
import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.wallet.Wallet;

import java.util.concurrent.CompletableFuture;

public class CredentialsSearchForProofReq extends IndyJava.API implements AutoCloseable {
	private final int searchHandle;

	private CredentialsSearchForProofReq(int searchHandle) {
		this.searchHandle = searchHandle;
	}

	/**
	 * Callback used when proverSearchCredentials completes.
	 */
	private static Callback proverSearchCredentialsForProofReqCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, int search_handle) {

			CompletableFuture<CredentialsSearchForProofReq> future = (CompletableFuture<CredentialsSearchForProofReq>) removeFuture(xcommand_handle);
			if (!checkCallback(future, err)) return;

			CredentialsSearchForProofReq result = new CredentialsSearchForProofReq(search_handle);
			future.complete(result);
		}
	};

	/**
	 * Callback used when a function returning Void completes.
	 */
	private static Callback voidCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (!checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	public static CompletableFuture<CredentialsSearchForProofReq> open(
			Wallet wallet,
			String proofReqJson,
			String extraQueryJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(proofReqJson, "proofReqJson");

		CompletableFuture<CredentialsSearchForProofReq> future = new CompletableFuture<CredentialsSearchForProofReq>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_search_credentials_for_proof_req(
				commandHandle,
				walletHandle,
				proofReqJson,
				extraQueryJson,
				proverSearchCredentialsForProofReqCb);

		checkResult(result);

		return future;
	}

	public CompletableFuture<String> fetchNextCredentials(
			String itemRef,
			int count
	) throws IndyException {
		ParamGuard.notNullOrWhiteSpace(itemRef, "itemRef");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_prover_fetch_credentials_for_proof_req(
				commandHandle,
				searchHandle,
				itemRef,
				count,
				Anoncreds.stringCb);

		checkResult(result);

		return future;
	}

	public CompletableFuture<Void> closeSearch() throws IndyException {
		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_prover_close_credentials_search_for_proof_req(
				commandHandle,
				searchHandle,
				voidCb);

		checkResult(result);

		return future;
	}

	@Override
	public void close() throws Exception {
		closeSearch().get();
	}
}
