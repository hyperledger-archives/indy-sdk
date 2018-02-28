package org.hyperledger.indy.sdk.anoncreds;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateClaimResult;
import org.hyperledger.indy.sdk.wallet.Wallet;

import com.sun.jna.Callback;

/**
 * anoncreds.rs API
 */

/**
 * Functionality for anonymous credentials
 */
public class Anoncreds extends IndyJava.API {

	private Anoncreds() {

	}

	/*
	 * STATIC CALLBACKS
	 */

	/**
	 * Callback used when issuerCreateAndStoreClaimDef completes.
	 */
	private static Callback issuerCreateAndStoreClaimDefCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String claim_def_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = claim_def_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerCreateAndStoreRevocReg completes.
	 */
	private static Callback issuerCreateAndStoreRevocRegCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = revoc_reg_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerCreateClaimOffer completes.
	 */
	private static Callback issuerCreateClaimOfferCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String claim_offer_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = claim_offer_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerCreateClaim completes.
	 */
	private static Callback issuerCreateClaimCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_update_json, String xclaim_json) {

			CompletableFuture<IssuerCreateClaimResult> future = (CompletableFuture<IssuerCreateClaimResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			IssuerCreateClaimResult result = new IssuerCreateClaimResult(revoc_reg_update_json, xclaim_json);
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerRevokeClaim completes.
	 */
	private static Callback issuerRevokeClaimCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_update_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = revoc_reg_update_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverStoreClaimOffer completes.
	 */
	private static Callback proverStoreClaimOfferCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverGetClaimOffers completes.
	 */
	private static Callback proverGetClaimOffersCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String claim_offers_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = claim_offers_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverCreateMasterSecret completes.
	 */
	private static Callback proverCreateMasterSecretCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};


	/**
	 * Callback used when proverCreateAndStoreClaimReq completes.
	 */
	private static Callback proverCreateAndStoreClaimReqCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String claim_req_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = claim_req_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverStoreClaim completes.
	 */
	private static Callback proverStoreClaimCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverGetClaims completes.
	 */
	private static Callback proverGetClaimsCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String claimsJson) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = claimsJson;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverGetClaimsForProofReq completes.
	 */
	private static Callback proverGetClaimsForProofReqCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String claimsJson) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = claimsJson;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverCreateProof completes.
	 */
	private static Callback proverCreateProofCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String proofJson) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = proofJson;
			future.complete(result);
		}
	};

	/**
	 * Callback used when verifierVerifyProof completes.
	 */
	private static Callback verifierVerifyProofCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, Boolean valid) {

			CompletableFuture<Boolean> future = (CompletableFuture<Boolean>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Boolean result = valid;
			future.complete(result);
		}
	};


	
	/*
	 * STATIC METHODS
	 */

	/**
	 * Create keys (both primary and revocation) for the given schema and signature type (currently only CL signature type is supported).
	 *
	 * @param wallet         The wallet.
	 * @param issuerDid      The DID of the issuer.
	 * @param schemaJson     The JSON schema for the claim.
	 * @param signatureType  The signature type.
	 * @param createNonRevoc Whether or not to create a non-revokable claim definition.
	 * @return A future resolving to a JSON string containing the claim definition.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> issuerCreateAndStoreClaimDef(
			Wallet wallet,
			String issuerDid,
			String schemaJson,
			String signatureType,
			boolean createNonRevoc) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(issuerDid, "issuerDid");
		ParamGuard.notNullOrWhiteSpace(schemaJson, "schemaJson");

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

	/**
	 * Create a new revocation registry for the given claim definition
	 *
	 * @param wallet      The wallet.
	 * @param issuerDid   The DID of the issuer.
	 * @param schemaJson  The schema to use.
	 * @param maxClaimNum The maximum claim numbber.
	 * @return A future resolving to a JSON string containing the revocation registry.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> issuerCreateAndStoreRevocReg(
			Wallet wallet,
			String issuerDid,
			String schemaJson,
			int maxClaimNum) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(issuerDid, "issuerDid");
		ParamGuard.notNull(schemaJson, "schemaJson");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_and_store_revoc_reg(
				commandHandle,
				walletHandle,
				issuerDid,
				schemaJson,
				maxClaimNum,
				issuerCreateAndStoreRevocRegCb);

		checkResult(result);

		return future;
	}

	/**
	 * Create claim offer in Wallet.
	 *
	 * @param wallet         The wallet.
	 * @param schemaJson     The JSON schema for the claim.
	 * @param issuerDid      The DID of the issuer.
	 * @param proverDid      The DID of the target user.
	 * @return A future resolving to a JSON string containing the claim offer.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> issuerCreateClaimOffer(
			Wallet wallet,
			String schemaJson,
			String issuerDid,
			String proverDid) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(schemaJson, "schemaJson");
		ParamGuard.notNullOrWhiteSpace(issuerDid, "issuerDid");
		ParamGuard.notNullOrWhiteSpace(proverDid, "proverDid");


		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_claim_offer(
				commandHandle,
				walletHandle,
				schemaJson,
				issuerDid,
				proverDid,
				issuerCreateClaimOfferCb);

		checkResult(result);

		return future;
	}

	/**
	 * Signs a given claim for the given user by a given key (claim def).
	 *
	 * @param wallet         The wallet.
	 * @param claimReqJson   a claim request with a blinded secret
	 * @param claimJson      a claim containing attribute values for each of requested attribute names.
	 * @param userRevocIndex index of a new user in the revocation registry (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)
	 * @return A future resolving to a revocation registry update json with a newly issued claim
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<IssuerCreateClaimResult> issuerCreateClaim(
			Wallet wallet,
			String claimReqJson,
			String claimJson,
			int userRevocIndex) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(claimReqJson, "claimReqJson");
		ParamGuard.notNullOrWhiteSpace(claimJson, "claimJson");

		CompletableFuture<IssuerCreateClaimResult> future = new CompletableFuture<IssuerCreateClaimResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_claim(
				commandHandle,
				walletHandle,
				claimReqJson,
				claimJson,
				userRevocIndex,
				issuerCreateClaimCb);

		checkResult(result);

		return future;
	}

	/**
	 * Revokes a user identified by a revoc_id in a given revoc-registry.
	 *
	 * @param wallet         A wallet.
	 * @param issuerDid      The DID of the issuer.
	 * @param schemaJson     The schema to use.
	 * @param userRevocIndex index of the user in the revocation registry
	 * @return A future resolving to a revocation registry update json with a revoked claim
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> issuerRevokeClaim(
			Wallet wallet,
			String issuerDid,
			String schemaJson,
			int userRevocIndex) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(issuerDid, "issuerDid");
		ParamGuard.notNull(schemaJson, "schemaJson");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_revoke_claim(
				commandHandle,
				walletHandle,
				issuerDid,
				schemaJson,
				userRevocIndex,
				issuerRevokeClaimCb);

		checkResult(result);

		return future;
	}

	/**
	 * Stores a claim offer from the given issuer in a secure storage.
	 *
	 * @param wallet         A wallet.
	 * @param claimOfferJson claim offer as a json containing information about the issuer and a claim
	 * @return A future that does not resolve any value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> proverStoreClaimOffer(
			Wallet wallet,
			String claimOfferJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(claimOfferJson, "claimOfferJson");

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

	/**
	 * Gets all stored claim offers (see prover_store_claim_offer).
	 *
	 * @param wallet     A wallet.
	 * @param filterJson optional filter to get claim offers for specific Issuer, claim_def or schema only only
	 * @return A future that resolves to a json with a list of claim offers for the filter.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverGetClaimOffers(
			Wallet wallet,
			String filterJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(filterJson, "filterJson");

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

	/**
	 * Creates a master secret with a given name and stores it in the wallet.
	 *
	 * @param wallet           A wallet.
	 * @param masterSecretName a new master secret name.
	 * @return A future that does not resolve any value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> proverCreateMasterSecret(
			Wallet wallet,
			String masterSecretName) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(masterSecretName, "masterSecretName");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_create_master_secret(
				commandHandle,
				walletHandle,
				masterSecretName,
				proverCreateMasterSecretCb);

		checkResult(result);

		return future;
	}

	/**
	 * Creates a clam request json for the given claim offer and stores it in a secure wallet.
	 *
	 * @param wallet           A wallet.
	 * @param proverDid        The DID of the prover.
	 * @param claimOfferJson   claim offer as a json containing information about the issuer and a claim
	 * @param claimDefJson     claim definition json associated with issuer_did and schema_seq_no in the claim_offer
	 * @param masterSecretName the name of the master secret stored in the wallet
	 * @return A future that resolves to a claim request json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverCreateAndStoreClaimReq(
			Wallet wallet,
			String proverDid,
			String claimOfferJson,
			String claimDefJson,
			String masterSecretName) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(proverDid, "proverDid");
		ParamGuard.notNullOrWhiteSpace(claimOfferJson, "claimOfferJson");
		ParamGuard.notNullOrWhiteSpace(claimDefJson, "claimDefJson");
		ParamGuard.notNullOrWhiteSpace(masterSecretName, "masterSecretName");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_create_and_store_claim_req(
				commandHandle,
				walletHandle,
				proverDid,
				claimOfferJson,
				claimDefJson,
				masterSecretName,
				proverCreateAndStoreClaimReqCb);

		checkResult(result);

		return future;
	}

	/**
	 * Updates the claim by a master secret and stores in a secure wallet.
	 *
	 * @param wallet     A Wallet.
	 * @param claim      The claim to store.
	 * @param revRegJson revocation registry associated with issuer_did and schema_key in the claim_offer
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> proverStoreClaim(
			Wallet wallet,
			String claim,
			String revRegJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(claim, "claim");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_store_claim(
				commandHandle,
				walletHandle,
				claim,
				revRegJson,
				proverStoreClaimCb);

		checkResult(result);

		return future;
	}

	/**
	 * Gets human readable claims according to the filter.
	 *
	 * @param wallet A wallet.
	 * @param filter filter for claims
	 * @return A future that resolves to a claims json
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverGetClaims(
			Wallet wallet,
			String filter) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(filter, "filter");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_get_claims(
				commandHandle,
				walletHandle,
				filter,
				proverGetClaimsCb);

		checkResult(result);

		return future;
	}

	/**
	 * Gets human readable claims matching the given proof request.
	 *
	 * @param wallet       A wallet.
	 * @param proofRequest proof request json
	 * @return A future that resolves to a json with claims for the given pool request.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverGetClaimsForProofReq(
			Wallet wallet,
			String proofRequest) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(proofRequest, "proofRequest");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_get_claims_for_proof_req(
				commandHandle,
				walletHandle,
				proofRequest,
				proverGetClaimsForProofReqCb);

		checkResult(result);

		return future;
	}

	/**
	 * Creates a proof according to the given proof request.
	 *
	 * @param wallet          A wallet.
	 * @param proofRequest    proof request json as come from the verifier
	 * @param requestedClaims either a claim or self-attested attribute for each requested attribute
	 * @param schemas         all schema jsons participating in the proof request
	 * @param masterSecret    the name of the master secret stored in the wallet
	 * @param claimDefs       all claim definition jsons participating in the proof request
	 * @param revocRegs       all revocation registry jsons participating in the proof request
	 * @return A future resolving to a Proof json
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverCreateProof(
			Wallet wallet,
			String proofRequest,
			String requestedClaims,
			String schemas,
			String masterSecret,
			String claimDefs,
			String revocRegs) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(proofRequest, "proofRequest");
		ParamGuard.notNullOrWhiteSpace(requestedClaims, "requestedClaims");
		ParamGuard.notNullOrWhiteSpace(schemas, "schemas");
		ParamGuard.notNullOrWhiteSpace(masterSecret, "masterSecret");
		ParamGuard.notNullOrWhiteSpace(claimDefs, "claimDefs");
		ParamGuard.notNullOrWhiteSpace(revocRegs, "revocRegs");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_create_proof(
				commandHandle,
				walletHandle,
				proofRequest,
				requestedClaims,
				schemas,
				masterSecret,
				claimDefs,
				revocRegs,
				proverCreateProofCb);

		checkResult(result);

		return future;
	}

	/**
	 * Verifies a proof (of multiple claim).
	 *
	 * @param proofRequest initial proof request as sent by the verifier
	 * @param proof        proof json
	 * @param schemas      all schema jsons participating in the proof
	 * @param claimDefs    all claim definition jsons participating in the proof
	 * @param revocRegs    all revocation registry jsons participating in the proof
	 * @return A future resolving to true if signature is valid, otherwise false.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Boolean> verifierVerifyProof(
			String proofRequest,
			String proof,
			String schemas,
			String claimDefs,
			String revocRegs) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(proofRequest, "proofRequest");
		ParamGuard.notNullOrWhiteSpace(proof, "proof");
		ParamGuard.notNullOrWhiteSpace(schemas, "schemas");
		ParamGuard.notNullOrWhiteSpace(claimDefs, "claimDefs");
		ParamGuard.notNullOrWhiteSpace(revocRegs, "revocRegs");

		CompletableFuture<Boolean> future = new CompletableFuture<Boolean>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_verifier_verify_proof(
				commandHandle,
				proofRequest,
				proof,
				schemas,
				claimDefs,
				revocRegs,
				verifierVerifyProofCb);

		checkResult(result);

		return future;
	}
}