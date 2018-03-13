package org.hyperledger.indy.sdk.anoncreds;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateSchemaResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreClaimDefResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreRevocRegResult;
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
	 * Callback used when issuerCreateSchema completes.
	 */
	private static Callback issuerCreateSchemaCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String schema_id, String schema_json) {

			CompletableFuture<IssuerCreateSchemaResult> future = (CompletableFuture<IssuerCreateSchemaResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			IssuerCreateSchemaResult result = new IssuerCreateSchemaResult(schema_id, schema_json);
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerCreateAndStoreClaimDef completes.
	 */
	private static Callback issuerCreateAndStoreClaimDefCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String credential_def_id, String credential_def_json) {

			CompletableFuture<IssuerCreateAndStoreClaimDefResult> future = (CompletableFuture<IssuerCreateAndStoreClaimDefResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			IssuerCreateAndStoreClaimDefResult result = new IssuerCreateAndStoreClaimDefResult(credential_def_id, credential_def_json);
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerCreateAndStoreRevocReg completes.
	 */
	private static Callback issuerCreateAndStoreRevocRegCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_id, String revoc_reg_def_json, String revoc_reg_entry_json) {

			CompletableFuture<IssuerCreateAndStoreRevocRegResult> future = (CompletableFuture<IssuerCreateAndStoreRevocRegResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			IssuerCreateAndStoreRevocRegResult result = new IssuerCreateAndStoreRevocRegResult(revoc_reg_id, revoc_reg_def_json, revoc_reg_entry_json);
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerCreateClaimOffer completes.
	 */
	private static Callback issuerCreateClaimOfferCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String credential_offer_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = credential_offer_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerCreateClaim completes.
	 */
	private static Callback issuerCreateClaimCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_delta_json, String xcredential_json) {

			CompletableFuture<IssuerCreateClaimResult> future = (CompletableFuture<IssuerCreateClaimResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			IssuerCreateClaimResult result = new IssuerCreateClaimResult(revoc_reg_delta_json, xcredential_json);
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerRevokeClaim completes.
	 */
	private static Callback issuerRevokeClaimCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_delta_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = revoc_reg_delta_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerRecoverClaim completes.
	 */
	private static Callback issuerRecoverClaimCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_delta_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = revoc_reg_delta_json;
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
		public void callback(int xcommand_handle, int err, String credential_offers_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = credential_offers_json;
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
		public void callback(int xcommand_handle, int err, String credential_req_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = credential_req_json;
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
		public void callback(int xcommand_handle, int err, String credentialsJson) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = credentialsJson;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverGetClaimsForProofReq completes.
	 */
	private static Callback proverGetClaimsForProofReqCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String credentialsJson) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = credentialsJson;
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

	/**
	 * Callback used when createRevocationInfo completes.
	 */
	private static Callback createRevocationInfoCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String rev_info_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = rev_info_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when updateRevocationInfo completes.
	 */
	private static Callback updateRevocationInfoCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String updated_rev_info_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = updated_rev_info_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when storeRevocationInfo completes.
	 */
	private static Callback storeRevocationInfoCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err) {

			CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			Void result = null;
			future.complete(result);
		}
	};

	/**
	 * Callback used when getRevocationInfo completes.
	 */
	private static Callback getRevocationInfoCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String rev_info_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = rev_info_json;
			future.complete(result);
		}
	};

	/*
	 * STATIC METHODS
	 */

	/**
	 * Create credential schema.
	 *
	 * @param issuerDid  The DID of the issuer.
	 * @param name       Human-readable name of schema.
	 * @param version    Version of schema.
	 * @param attrNames: List of attributes schema contains.
	 * @return A future resolving to a JSON string containing the credential definition.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<IssuerCreateSchemaResult> issuerCreateSchema(
			String issuerDid,
			String name,
			String version,
			String attrNames) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(issuerDid, "issuerDid");
		ParamGuard.notNullOrWhiteSpace(name, "name");
		ParamGuard.notNullOrWhiteSpace(version, "version");
		ParamGuard.notNullOrWhiteSpace(attrNames, "attrNames");

		CompletableFuture<IssuerCreateSchemaResult> future = new CompletableFuture<IssuerCreateSchemaResult>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_issuer_create_schema(
				commandHandle,
				issuerDid,
				name,
				version,
				attrNames,
				issuerCreateSchemaCb);

		checkResult(result);

		return future;
	}

	/**
	 * Create keys (both primary and revocation) for the given schema and signature type (currently only CL signature type is supported).
	 *
	 * @param wallet         The wallet.
	 * @param issuerDid      The DID of the issuer.
	 * @param schemaJson     The JSON schema for the credential.
	 * @param tag
	 * @param type          (optional) Signature type. Currently only 'CL' is supported.
	 * @param configJson    Config json. {"support_revocation": boolean}
	 * @return A future resolving to a Id and JSON string containing the credential definition.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<IssuerCreateAndStoreClaimDefResult> issuerCreateAndStoreClaimDef(
			Wallet wallet,
			String issuerDid,
			String schemaJson,
			String tag,
			String type,
			String configJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(issuerDid, "issuerDid");
		ParamGuard.notNullOrWhiteSpace(schemaJson, "schemaJson");
		ParamGuard.notNullOrWhiteSpace(tag, "tag");
		ParamGuard.notNullOrWhiteSpace(configJson, "configJson");

		CompletableFuture<IssuerCreateAndStoreClaimDefResult> future = new CompletableFuture<IssuerCreateAndStoreClaimDefResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_and_store_credential_def(
				commandHandle,
				walletHandle,
				issuerDid,
				schemaJson,
				tag,
				type,
				configJson,
				issuerCreateAndStoreClaimDefCb);

		checkResult(result);

		return future;
	}

	/**
	 * Create a new revocation registry for the given credential definition
	 *
	 * @param wallet      The wallet.
	 * @param issuerDid   The DID of the issuer.
	 * @param type        (optional) Registry type. Currently only 'CL_ACCUM' is supported.
	 * @param tag
	 * @param credDefId   Id of stored in ledger credential definition
	 * @param configJson   {
	     "issuance_type": (optional) type of issuance. Currently supported:
	         1) ISSUANCE_BY_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over all indices;
	                                 Revocation Registry is updated only during revocation.
	         2) ISSUANCE_ON_DEMAND: nothing is issued initially accumulator is 1 (used by default);
	     "max_cred_num": maximum number of credentials the new registry can process.
	 }
	 * @param tailsWriterType
	 * @param tailsWriterConfig
	 * @return A future resolving to a JSON string containing the revocation registry.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<IssuerCreateAndStoreRevocRegResult> issuerCreateAndStoreRevocReg(
			Wallet wallet,
			String issuerDid,
			String type,
			String tag,
			String credDefId,
			String configJson,
			String tailsWriterType,
			String tailsWriterConfig) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(issuerDid, "issuerDid");
		ParamGuard.notNullOrWhiteSpace(tag, "tag");
		ParamGuard.notNullOrWhiteSpace(credDefId, "credDefId");
		ParamGuard.notNullOrWhiteSpace(configJson, "configJson");
		ParamGuard.notNullOrWhiteSpace(tailsWriterConfig, "tailsWriterConfig");

		CompletableFuture<IssuerCreateAndStoreRevocRegResult> future = new CompletableFuture<IssuerCreateAndStoreRevocRegResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_and_store_revoc_reg(
				commandHandle,
				walletHandle,
				issuerDid,
				type,
				tag,
				credDefId,
				configJson,
				tailsWriterType,
				tailsWriterConfig,
				issuerCreateAndStoreRevocRegCb);

		checkResult(result);

		return future;
	}

	/**
	 * Create credential offer in Wallet.
	 *
	 * @param wallet     The wallet.
	 * @param credDefId  Id of stored in ledger credential definition.
	 * @param issuerDid  The DID of the issuer.
	 * @param proverDid  The DID of the target user.
	 * @return A future resolving to a JSON string containing the credential offer.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> issuerCreateClaimOffer(
			Wallet wallet,
			String credDefId,
			String issuerDid,
			String proverDid) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(credDefId, "credDefId");
		ParamGuard.notNullOrWhiteSpace(issuerDid, "issuerDid");
		ParamGuard.notNullOrWhiteSpace(proverDid, "proverDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_credential_offer(
				commandHandle,
				walletHandle,
				credDefId,
				issuerDid,
				proverDid,
				issuerCreateClaimOfferCb);

		checkResult(result);

		return future;
	}

	/**
	 * Signs a given credential for the given user by a given key (credential def).
	 *
	 * @param wallet         The wallet.
	 * @param credentialReqJson   a credential request with a blinded secret
	 * @param credentialJson      a credential containing attribute values for each of requested attribute names.
	 * @param revRegId       (Optional) id of stored in ledger revocation registry definition
	 * @param tailsReaderHandle
	 * @param userRevocIndex index of a new user in the revocation registry (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)
	 * @return A future resolving to a revocation registry update json with a newly issued credential
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<IssuerCreateClaimResult> issuerCreateClaim(
			Wallet wallet,
			String credentialReqJson,
			String credentialJson,
			String revRegId,
			int tailsReaderHandle,
			int userRevocIndex) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(credentialReqJson, "credentialReqJson");
		ParamGuard.notNullOrWhiteSpace(credentialJson, "credentialJson");

		CompletableFuture<IssuerCreateClaimResult> future = new CompletableFuture<IssuerCreateClaimResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_credential(
				commandHandle,
				walletHandle,
				credentialReqJson,
				credentialJson,
				revRegId,
				tailsReaderHandle,
				userRevocIndex,
				issuerCreateClaimCb);

		checkResult(result);

		return future;
	}

	/**
	 * Revokes a user identified by a revoc_id in a given revoc-registry.
	 *
	 * @param wallet         A wallet.
	 * @param tailsReaderHandle
	 * @param revRegId       Id of revocation registry stored in wallet.
	 * @param userRevocIndex index of the user in the revocation registry
	 * @return A future resolving to a revocation registry update json with a revoked credential
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> issuerRevokeClaim(
			Wallet wallet,
			int tailsReaderHandle,
			String revRegId,
			int userRevocIndex) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNull(revRegId, "revRegId");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_revoke_credential(
				commandHandle,
				walletHandle,
				tailsReaderHandle,
				revRegId,
				userRevocIndex,
				issuerRevokeClaimCb);

		checkResult(result);

		return future;
	}

	/**
	 * Recover a user identified by a revoc_id in a given revoc-registry.
	 *
	 * @param wallet         A wallet.
	 * @param tailsReaderHandle
	 * @param revRegId       Id of revocation registry stored in wallet.
	 * @param userRevocIndex index of the user in the revocation registry
	 * @return A future resolving to a revocation registry update json with a revoked credential
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> issuerRecoverClaim(
			Wallet wallet,
			int tailsReaderHandle,
			String revRegId,
			int userRevocIndex) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNull(revRegId, "revRegId");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_recover_credential(
				commandHandle,
				walletHandle,
				tailsReaderHandle,
				revRegId,
				userRevocIndex,
				issuerRecoverClaimCb);

		checkResult(result);

		return future;
	}

	/**
	 * Stores a credential offer from the given issuer in a secure storage.
	 *
	 * @param wallet         A wallet.
	 * @param credentialOfferJson credential offer as a json containing information about the issuer and a credential
	 * @return A future that does not resolve any value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> proverStoreClaimOffer(
			Wallet wallet,
			String credentialOfferJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(credentialOfferJson, "credentialOfferJson");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_store_credential_offer(
				commandHandle,
				walletHandle,
				credentialOfferJson,
				proverStoreClaimOfferCb);

		checkResult(result);

		return future;
	}

	/**
	 * Gets all stored credential offers (see prover_store_credential_offer).
	 *
	 * @param wallet     A wallet.
	 * @param filterJson optional filter to get credential offers for specific Issuer, credential_def or schema only only
	 * @return A future that resolves to a json with a list of credential offers for the filter.
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

		int result = LibIndy.api.indy_prover_get_credential_offers(
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
	 * Creates a clam request json for the given credential offer and stores it in a secure wallet.
	 *
	 * @param wallet           A wallet.
	 * @param proverDid        The DID of the prover.
	 * @param credentialOfferJson   credential offer as a json containing information about the issuer and a credential
	 * @param credentialDefJson     credential definition json associated with issuer_did and schema_seq_no in the credential_offer
	 * @param masterSecretName the name of the master secret stored in the wallet
	 * @return A future that resolves to a credential request json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverCreateAndStoreClaimReq(
			Wallet wallet,
			String proverDid,
			String credentialOfferJson,
			String credentialDefJson,
			String masterSecretName) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(proverDid, "proverDid");
		ParamGuard.notNullOrWhiteSpace(credentialOfferJson, "credentialOfferJson");
		ParamGuard.notNullOrWhiteSpace(credentialDefJson, "credentialDefJson");
		ParamGuard.notNullOrWhiteSpace(masterSecretName, "masterSecretName");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_create_and_store_credential_req(
				commandHandle,
				walletHandle,
				proverDid,
				credentialOfferJson,
				credentialDefJson,
				masterSecretName,
				proverCreateAndStoreClaimReqCb);

		checkResult(result);

		return future;
	}

	/**
	 * Updates the credential by a master secret and stores in a secure wallet.
	 *
	 * @param wallet     A Wallet.
	 * @param id         Identifier by which credential will be stored in wallet
	 * @param credential      The credential to store.
	 * @param revRegDefJson revocation registry definition associated with issuer_did and schema_key in the credential_offer
	 * @return A future that does not resolve a value.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Void> proverStoreClaim(
			Wallet wallet,
			String id,
			String credential,
			String revRegDefJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(id, "id");
		ParamGuard.notNullOrWhiteSpace(credential, "credential");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_store_credential(
				commandHandle,
				walletHandle,
				id,
				credential,
				revRegDefJson,
				proverStoreClaimCb);

		checkResult(result);

		return future;
	}

	/**
	 * Gets human readable credentials according to the filter.
	 *
	 * @param wallet A wallet.
	 * @param filter filter for credentials
	 * @return A future that resolves to a credentials json
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

		int result = LibIndy.api.indy_prover_get_credentials(
				commandHandle,
				walletHandle,
				filter,
				proverGetClaimsCb);

		checkResult(result);

		return future;
	}

	/**
	 * Gets human readable credentials matching the given proof request.
	 *
	 * @param wallet       A wallet.
	 * @param proofRequest proof request json
	 * @return A future that resolves to a json with credentials for the given pool request.
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

		int result = LibIndy.api.indy_prover_get_credentials_for_proof_req(
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
	 * @param requestedClaims either a credential or self-attested attribute for each requested attribute
	 * @param schemas         all schema jsons participating in the proof request
	 * @param masterSecret    the name of the master secret stored in the wallet
	 * @param credentialDefs       all credential definition jsons participating in the proof request
	 * @param revInfos       all revocation registry jsons participating in the proof request
	 * @return A future resolving to a Proof json
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverCreateProof(
			Wallet wallet,
			String proofRequest,
			String requestedClaims,
			String schemas,
			String masterSecret,
			String credentialDefs,
			String revInfos) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(proofRequest, "proofRequest");
		ParamGuard.notNullOrWhiteSpace(requestedClaims, "requestedClaims");
		ParamGuard.notNullOrWhiteSpace(schemas, "schemas");
		ParamGuard.notNullOrWhiteSpace(masterSecret, "masterSecret");
		ParamGuard.notNullOrWhiteSpace(credentialDefs, "credentialDefs");
		ParamGuard.notNullOrWhiteSpace(revInfos, "revInfos");

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
				credentialDefs,
				revInfos,
				proverCreateProofCb);

		checkResult(result);

		return future;
	}

	/**
	 * Verifies a proof (of multiple credential).
	 *
	 * @param proofRequest initial proof request as sent by the verifier
	 * @param proof        proof json
	 * @param schemas      all schema jsons participating in the proof
	 * @param credentialDefs    all credential definition jsons participating in the proof
	 * @param revocRegDefs    all revocation registry definition jsons participating in the proof
	 * @param revocRegs    all revocation registry jsons participating in the proof
	 * @return A future resolving to true if signature is valid, otherwise false.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<Boolean> verifierVerifyProof(
			String proofRequest,
			String proof,
			String schemas,
			String credentialDefs,
			String revocRegDefs,
			String revocRegs) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(proofRequest, "proofRequest");
		ParamGuard.notNullOrWhiteSpace(proof, "proof");
		ParamGuard.notNullOrWhiteSpace(schemas, "schemas");
		ParamGuard.notNullOrWhiteSpace(credentialDefs, "credentialDefs");
		ParamGuard.notNullOrWhiteSpace(revocRegDefs, "revocRegDefs");
		ParamGuard.notNullOrWhiteSpace(revocRegs, "revocRegs");

		CompletableFuture<Boolean> future = new CompletableFuture<Boolean>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_verifier_verify_proof(
				commandHandle,
				proofRequest,
				proof,
				schemas,
				credentialDefs,
				revocRegDefs,
				revocRegs,
				verifierVerifyProofCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> createRevocationInfo(
			int tailsReaderHandle,
			String revRegDef,
			String revRegDelta,
			int timestamp,
			int revIdx) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(revRegDef, "revRegDef");
		ParamGuard.notNullOrWhiteSpace(revRegDelta, "revRegDelta");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_create_revocation_info(
				commandHandle,
				tailsReaderHandle,
				revRegDef,
				revRegDelta,
				timestamp,
				revIdx,
				createRevocationInfoCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> updateRevocationInfo(
			int tailsReaderHandle,
			String revInfo,
			String revRegDef,
			String revRegDelta,
			int timestamp,
			int revIdx) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(revInfo, "revInfo");
		ParamGuard.notNullOrWhiteSpace(revRegDef, "revRegDef");
		ParamGuard.notNullOrWhiteSpace(revRegDelta, "revRegDelta");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_update_revocation_info(
				commandHandle,
				tailsReaderHandle,
				revInfo,
				revRegDef,
				revRegDelta,
				timestamp,
				revIdx,
				updateRevocationInfoCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<Void> storeRevocationInfo(
			Wallet wallet,
			String id,
			String revocationInfo) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(id, "id");
		ParamGuard.notNullOrWhiteSpace(revocationInfo, "revocationInfo");

		CompletableFuture<Void> future = new CompletableFuture<Void>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_store_revocation_info(
				commandHandle,
				walletHandle,
				id,
				revocationInfo,
				storeRevocationInfoCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> getRevocationInfo(
			Wallet wallet,
			String id,
			int timestamp) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(id, "id");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_get_revocation_info(
				commandHandle,
				walletHandle,
				id,
				timestamp,
				getRevocationInfoCb);

		checkResult(result);

		return future;
	}

}