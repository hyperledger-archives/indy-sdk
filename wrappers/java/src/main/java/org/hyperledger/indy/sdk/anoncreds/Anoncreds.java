package org.hyperledger.indy.sdk.anoncreds;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreRevocRegResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateClaimResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import com.sun.jna.Callback;

/**
 * anoncreds.rs API
 */
public class Anoncreds extends IndyJava.API {

	private Anoncreds() {

	}

	/*
	 * STATIC METHODS
	 */

	public static CompletableFuture<String> issuerCreateAndStoreClaimDef(
			Wallet wallet,
			String issuerDid,
			String schemaJson, 
			String signatureType, 
			boolean createNonRevoc) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String claim_def_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = claim_def_json, claim_def_uuid;
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_and_store_claim_def(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				issuerDid,
				schemaJson,
				signatureType,
				createNonRevoc,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<IssuerCreateAndStoreRevocRegResult> issuerCreateAndStoreRevocReg(
			Wallet wallet,
			String issuerDid,
			int schemaSeqNo, 
			int maxClaimNum) throws IndyException {

		final CompletableFuture<IssuerCreateAndStoreRevocRegResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String revoc_reg_json, String revoc_reg_uuid) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				IssuerCreateAndStoreRevocRegResult result = new IssuerCreateAndStoreRevocRegResult(revoc_reg_json, revoc_reg_uuid);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_and_store_revoc_reg(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				issuerDid,
				schemaSeqNo,
				maxClaimNum,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<IssuerCreateClaimResult> issuerCreateClaim(
			Wallet wallet,
			String claimReqJson, 
			String claimJson,
			int revocRegSeqNo,
			int userRevocIndex) throws IndyException {

		final CompletableFuture<IssuerCreateClaimResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String revoc_reg_update_json, String xclaim_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				IssuerCreateClaimResult result = new IssuerCreateClaimResult(revoc_reg_update_json, xclaim_json);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_claim(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				claimReqJson,
				claimJson,
				revocRegSeqNo,
				userRevocIndex,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> issuerRevokeClaim(
			Wallet wallet,
			int revocRegSeqNo, 
			int userRevocIndex) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String revoc_reg_update_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = revoc_reg_update_json;
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_revoke_claim(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				revocRegSeqNo,
				userRevocIndex,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> proverStoreClaimOffer(
			Wallet wallet,
			String claimOfferJson) throws IndyException {

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

		int result = LibIndy.api.indy_prover_store_claim_offer(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				claimOfferJson,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> proverGetClaimOffers(
			Wallet wallet,
			String filterJson) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String claim_offers_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = claim_offers_json;
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_get_claim_offers(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				filterJson,
				cb);

		checkResult(result);

		return future;
	}
}
