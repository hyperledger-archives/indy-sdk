package org.hyperledger.indy.sdk.anoncreds;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Future;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreClaimDefResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreRevocRegResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateClaimResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerRevokeClaimResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.ProverGetClaimOffersResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.ProverStoreClaimOfferResult;
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

	public static Future<IssuerCreateAndStoreClaimDefResult> issuerCreateAndStoreClaimDef(
			Wallet wallet,
			String schemaJson, 
			String signatureType, 
			boolean createNonRevoc) throws IndyException {

		final CompletableFuture<IssuerCreateAndStoreClaimDefResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String claim_def_json, String claim_def_uuid) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				IssuerCreateAndStoreClaimDefResult result = new IssuerCreateAndStoreClaimDefResult(claim_def_json, claim_def_uuid);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_and_store_claim_def(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				schemaJson,
				signatureType,
				createNonRevoc,
				callback);

		checkResult(result);

		return future;
	}

	public static Future<IssuerCreateAndStoreRevocRegResult> issuerCreateAndStoreRevocReg(
			Wallet wallet,
			int claimDefSeqNo, 
			int maxClaimNum) throws IndyException {

		final CompletableFuture<IssuerCreateAndStoreRevocRegResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

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
				claimDefSeqNo,
				maxClaimNum,
				callback);

		checkResult(result);

		return future;
	}

	public static Future<IssuerCreateClaimResult> issuerCreateClaim(
			Wallet wallet,
			String claimReqJson, 
			String claimJson,
			int revocRegSeqNo,
			int userRevocIndex) throws IndyException {

		final CompletableFuture<IssuerCreateClaimResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

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
				callback);

		checkResult(result);

		return future;
	}

	public static Future<IssuerRevokeClaimResult> issuerRevokeClaim(
			Wallet wallet,
			int claimDefSeqNo, 
			int revocRegSeqNo, 
			int userRevocIndex) throws IndyException {

		final CompletableFuture<IssuerRevokeClaimResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String revoc_reg_update_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				IssuerRevokeClaimResult result = new IssuerRevokeClaimResult(revoc_reg_update_json);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_revoke_claim(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				claimDefSeqNo,
				revocRegSeqNo,
				userRevocIndex,
				callback);

		checkResult(result);

		return future;
	}

	public static Future<ProverStoreClaimOfferResult> proverStoreClaimOffer(
			Wallet wallet,
			String claimOfferJson) throws IndyException {

		final CompletableFuture<ProverStoreClaimOfferResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				ProverStoreClaimOfferResult result = new ProverStoreClaimOfferResult();
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_store_claim_offer(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				claimOfferJson,
				callback);

		checkResult(result);

		return future;
	}

	public static Future<ProverGetClaimOffersResult> proverGetClaimOffers(
			Wallet wallet,
			String filterJson) throws IndyException {

		final CompletableFuture<ProverGetClaimOffersResult> future = new CompletableFuture<> ();

		Callback callback = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String claim_offers_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				ProverGetClaimOffersResult result = new ProverGetClaimOffersResult(claim_offers_json);
				future.complete(result);
			}
		};

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_get_claim_offers(
				FIXED_COMMAND_HANDLE, 
				walletHandle, 
				filterJson,
				callback);

		checkResult(result);

		return future;
	}
}
