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
	 * STATIC CALLBACKS
	 */

	private static Callback issuerCreateAndStoreClaimDefCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String claim_def_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = claim_def_json, claim_def_uuid;
			future.complete(result);
		}
	};

	private static Callback issuerCreateAndStoreRevocRegCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_json, String revoc_reg_uuid) {

			CompletableFuture<IssuerCreateAndStoreRevocRegResult> future = (CompletableFuture<IssuerCreateAndStoreRevocRegResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			IssuerCreateAndStoreRevocRegResult result = new IssuerCreateAndStoreRevocRegResult(revoc_reg_json, revoc_reg_uuid);
			future.complete(result);
		}
	};

	private static Callback issuerCreateClaimCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_update_json, String xclaim_json) {

			CompletableFuture<IssuerCreateClaimResult> future = (CompletableFuture<IssuerCreateClaimResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			IssuerCreateClaimResult result = new IssuerCreateClaimResult(revoc_reg_update_json, xclaim_json);
			future.complete(result);
		}
	};

	private static Callback issuerRevokeClaimCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_update_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = revoc_reg_update_json;
			future.complete(result);
		}
	};

	private static Callback proverStoreClaimOfferCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	private static Callback proverGetClaimOffersCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String claim_offers_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = claim_offers_json;
			future.complete(result);
		}
	};
	
	/*
	 * STATIC METHODS
	 */

	public static CompletableFuture<String> issuerCreateAndStoreClaimDef(
			Wallet wallet,
			String issuerDid,
			String schemaJson, 
			String signatureType, 
			boolean createNonRevoc) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_and_store_claim_def(
				commandHandle, 
				walletHandle, 
				issuerDid,
				schemaJson,
				signatureType,
				createNonRevoc,
				issuerCreateAndStoreClaimDefCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<IssuerCreateAndStoreRevocRegResult> issuerCreateAndStoreRevocReg(
			Wallet wallet,
			String issuerDid,
			int schemaSeqNo, 
			int maxClaimNum) throws IndyException {

		CompletableFuture<IssuerCreateAndStoreRevocRegResult> future = new CompletableFuture<IssuerCreateAndStoreRevocRegResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_and_store_revoc_reg(
				commandHandle, 
				walletHandle, 
				issuerDid,
				schemaSeqNo,
				maxClaimNum,
				issuerCreateAndStoreRevocRegCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<IssuerCreateClaimResult> issuerCreateClaim(
			Wallet wallet,
			String claimReqJson, 
			String claimJson,
			int revocRegSeqNo,
			int userRevocIndex) throws IndyException {

		CompletableFuture<IssuerCreateClaimResult> future = new CompletableFuture<IssuerCreateClaimResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_claim(
				commandHandle, 
				walletHandle, 
				claimReqJson,
				claimJson,
				revocRegSeqNo,
				userRevocIndex,
				issuerCreateClaimCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> issuerRevokeClaim(
			Wallet wallet,
			int revocRegSeqNo, 
			int userRevocIndex) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_revoke_claim(
				commandHandle, 
				walletHandle, 
				revocRegSeqNo,
				userRevocIndex,
				issuerRevokeClaimCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> proverStoreClaimOffer(
			Wallet wallet,
			String claimOfferJson) throws IndyException {

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_store_claim_offer(
				commandHandle, 
				walletHandle, 
				claimOfferJson,
				proverStoreClaimOfferCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> proverGetClaimOffers(
			Wallet wallet,
			String filterJson) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_get_claim_offers(
				commandHandle, 
				walletHandle, 
				filterJson,
				proverGetClaimOffersCb);

		checkResult(result);

		return future;
	}
}
