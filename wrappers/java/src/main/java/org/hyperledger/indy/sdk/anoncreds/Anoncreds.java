package org.hyperledger.indy.sdk.anoncreds;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateSchemaResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreCredentialDefResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateAndStoreRevocRegResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.IssuerCreateCredentialResult;
import org.hyperledger.indy.sdk.anoncreds.AnoncredsResults.ProverCreateCredentialRequestResult;
import org.hyperledger.indy.sdk.blob_storage.BlobStorageWriter;
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
	 * Callback used when issuerCreateAndStoreCredentialDef completes.
	 */
	private static Callback issuerCreateAndStoreCredentialDefCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String credential_def_id, String credential_def_json) {

			CompletableFuture<IssuerCreateAndStoreCredentialDefResult> future = (CompletableFuture<IssuerCreateAndStoreCredentialDefResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			IssuerCreateAndStoreCredentialDefResult result = new IssuerCreateAndStoreCredentialDefResult(credential_def_id, credential_def_json);
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
	 * Callback used when issuerCreateCredentialOffer completes.
	 */
	private static Callback issuerCreateCredentialOfferCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String credential_offer_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = credential_offer_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerMergeRevocationRegistryDeltas completes.
	 */
	private static Callback issuerMergeRevocationRegistryDeltasCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String merged_rev_reg_delta) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = merged_rev_reg_delta;
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerCreateCredential completes.
	 */
	private static Callback issuerCreateCredentialCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String cred_json, String cred_rev_id, String revoc_reg_delta_json) {

			CompletableFuture<IssuerCreateCredentialResult> future = (CompletableFuture<IssuerCreateCredentialResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			IssuerCreateCredentialResult result = new IssuerCreateCredentialResult(cred_json, cred_rev_id, revoc_reg_delta_json);
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerRevokeCredential completes.
	 */
	private static Callback issuerRevokeCredentialCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_delta_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = revoc_reg_delta_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when issuerRecoverCredential completes.
	 */
	private static Callback issuerRecoverCredentialCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String revoc_reg_delta_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = revoc_reg_delta_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverCreateMasterSecret completes.
	 */
	private static Callback proverCreateMasterSecretCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String master_secret_id) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = master_secret_id;
			future.complete(result);
		}
	};


	/**
	 * Callback used when proverCreateCredentialReq completes.
	 */
	private static Callback proverCreateCredentialReqCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String credential_req_json, String credential_req_metadata_json) {

			CompletableFuture<ProverCreateCredentialRequestResult> future = (CompletableFuture<ProverCreateCredentialRequestResult>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			ProverCreateCredentialRequestResult result = new ProverCreateCredentialRequestResult(credential_req_json, credential_req_metadata_json);
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverStoreCredential completes.
	 */
	private static Callback proverStoreCredentialCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String outCredId) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = outCredId;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverGetCredentials completes.
	 */
	private static Callback proverGetCredentialsCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String credentialsJson) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = credentialsJson;
			future.complete(result);
		}
	};

	/**
	 * Callback used when proverGetCredentialsForProofReq completes.
	 */
	private static Callback proverGetCredentialsForProofReqCb = new Callback() {

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
	 * Callback used when createRevocationState completes.
	 */
	private static Callback createRevocationStateCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String rev_state_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = rev_state_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when updateRevocationState completes.
	 */
	private static Callback updateRevocationStateCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String updated_rev_state_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = updated_rev_state_json;
			future.complete(result);
		}
	};

	/*
	 * STATIC METHODS
	 */

	/**
	 * Create credential schema entity that describes credential attributes list and allows credentials
	 * interoperability.
	 *
	 * Schema is public and intended to be shared with all anoncreds workflow actors usually by publishing SCHEMA transaction
	 * to Indy distributed ledger.
	 *
	 * It is IMPORTANT for current version POST Schema in Ledger and after that GET it from Ledger
	 * with correct seq_no to save compatibility with Ledger.
	 * After that can call indy_issuer_create_and_store_credential_def to build corresponding Credential Definition.
	 *
	 * @param issuerDid The DID of the issuer.
	 * @param name      Human-readable name of schema.
	 * @param version   Version of schema.
	 * @param attrs:    List of schema attributes descriptions
	 * @return A future resolving to IssuerCreateSchemaResult object containing:
	 * schemaId: identifier of created schema
	 * schemaJson: schema as json
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<IssuerCreateSchemaResult> issuerCreateSchema(
			String issuerDid,
			String name,
			String version,
			String attrs) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(issuerDid, "issuerDid");
		ParamGuard.notNullOrWhiteSpace(name, "name");
		ParamGuard.notNullOrWhiteSpace(version, "version");
		ParamGuard.notNullOrWhiteSpace(attrs, "attrs");

		CompletableFuture<IssuerCreateSchemaResult> future = new CompletableFuture<IssuerCreateSchemaResult>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_issuer_create_schema(
				commandHandle,
				issuerDid,
				name,
				version,
				attrs,
				issuerCreateSchemaCb);

		checkResult(result);

		return future;
	}

	/**
	 * Create credential definition entity that encapsulates credentials issuer DID, credential schema, secrets used for signing credentials
	 * and secrets used for credentials revocation.
	 * 
	 * Credential definition entity contains private and public parts. Private part will be stored in the wallet. Public part
	 * will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing CRED_DEF transaction
	 * to Indy distributed ledger.
	 * 
	 * It is IMPORTANT for current version GET Schema from Ledger with correct seq_no to save compatibility with Ledger.
	 *
	 * @param wallet     The wallet.
	 * @param issuerDid  DID of the issuer signing cred_def transaction to the Ledger
	 * @param schemaJson Сredential schema as a json
	 * @param tag        Allows to distinct between credential definitions for the same issuer and schema
	 * @param type       Credential definition type (optional, 'CL' by default) that defines credentials signature and revocation math.
	 *                   Supported types are:
	 *                   - 'CL': Camenisch-Lysyanskaya credential signature type
	 * @param configJson Type-specific configuration of credential definition as json:
	 *                   - 'CL':
	 *                      - revocationSupport: whether to request non-revocation credential (optional, default false)
	 * @return A future resolving to IssuerCreateAndStoreCredentialDefResult containing:.
	 * credDefId: identifier of created credential definition.
	 * credDefJson: public part of created credential definition
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<IssuerCreateAndStoreCredentialDefResult> issuerCreateAndStoreCredentialDef(
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

		CompletableFuture<IssuerCreateAndStoreCredentialDefResult> future = new CompletableFuture<IssuerCreateAndStoreCredentialDefResult>();
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
				issuerCreateAndStoreCredentialDefCb);

		checkResult(result);

		return future;
	}

	/**
	 * Create a new revocation registry for the given credential definition as tuple of entities:
	 * - Revocation registry definition that encapsulates credentials definition reference, revocation type specific configuration and
	 * secrets used for credentials revocation
	 * - Revocation registry state that stores the information about revoked entities in a non-disclosing way. The state can be
	 * represented as ordered list of revocation registry entries were each entry represents the list of revocation or issuance operations.
	 * 
	 * Revocation registry definition entity contains private and public parts. Private part will be stored in the wallet. Public part
	 * will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing REVOC_REG_DEF transaction
	 * to Indy distributed ledger.
	 * 
	 * Revocation registry state is stored on the wallet and also intended to be shared as the ordered list of REVOC_REG_ENTRY transactions.
	 * This call initializes the state in the wallet and returns the initial entry.
	 * 
	 * Some revocation registry types (for example, 'CL_ACCUM') can require generation of binary blob called tails used to hide information about revoked credentials in public
	 * revocation registry and intended to be distributed out of leger (REVOC_REG_DEF transaction will still contain uri and hash of tails).
	 * This call requires access to pre-configured blob storage writer instance handle that will allow to write generated tails.
	 *
	 * @param wallet      The wallet.
	 * @param issuerDid   The DID of the issuer.
	 * @param type        Revocation registry type (optional, default value depends on credential definition type). Supported types are:
	 *                    - 'CL_ACCUM': Type-3 pairing based accumulator. Default for 'CL' credential definition type
	 * @param tag         Allows to distinct between revocation registries for the same issuer and credential definition
	 * @param credDefId   Id of stored in ledger credential definition
	 * @param configJson  type-specific configuration of revocation registry as json:
	 * - 'CL_ACCUM': {
	 *     "issuance_type": (optional) type of issuance. Currently supported:
	 *         1) ISSUANCE_BY_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over all indices;
	 *            Revocation Registry is updated only during revocation.
	 *         2) ISSUANCE_ON_DEMAND: nothing is issued initially accumulator is 1 (used by default);
	 *     "max_cred_num": maximum number of credentials the new registry can process (optional, default 100000)
	 * }
	 * @param tailsWriter Handle of blob storage to store tails
	 * @return A future resolving to:
	 * revocRegId: identifier of created revocation registry definition
	 * revocRegDefJson: public part of revocation registry definition
	 * revocRegEntryJson: revocation registry entry that defines initial state of revocation registry
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<IssuerCreateAndStoreRevocRegResult> issuerCreateAndStoreRevocReg(
			Wallet wallet,
			String issuerDid,
			String type,
			String tag,
			String credDefId,
			String configJson,
			BlobStorageWriter tailsWriter) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(issuerDid, "issuerDid");
		ParamGuard.notNullOrWhiteSpace(tag, "tag");
		ParamGuard.notNullOrWhiteSpace(credDefId, "credDefId");
		ParamGuard.notNullOrWhiteSpace(configJson, "configJson");

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
				tailsWriter.getBlobStorageWriterHandle(),
				issuerCreateAndStoreRevocRegCb);

		checkResult(result);

		return future;
	}

	/**
	 * Create credential offer that will be used by Prover for
	 * credential request creation. Offer includes nonce and key correctness proof
	 * for authentication between protocol steps and integrity checking.
	 *
	 * @param wallet    The wallet.
	 * @param credDefId Id of stored in ledger credential definition.
	 * @return A future resolving to a JSON string containing the credential offer.
	 *     {
	 *         "schema_id": string,
	 *         "cred_def_id": string,
	 *         // Fields below can depend on Cred Def type
	 *         "nonce": string,
	 *         "key_correctness_proof" : <key_correctness_proof>
	 *     }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> issuerCreateCredentialOffer(
			Wallet wallet,
			String credDefId) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(credDefId, "credDefId");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_credential_offer(
				commandHandle,
				walletHandle,
				credDefId,
				issuerCreateCredentialOfferCb);

		checkResult(result);

		return future;
	}

	/**
	 * Check Cred Request for the given Cred Offer and issue Credential for the given Cred Request.
	 * 
	 * Cred Request must match Cred Offer. The credential definition and revocation registry definition
	 * referenced in Cred Offer and Cred Request must be already created and stored into the wallet.
	 * 
	 * Information for this credential revocation will be store in the wallet as part of revocation registry under
	 * generated cred_revoc_id local for this wallet.
	 * 
	 * This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
	 * Note that it is possible to accumulate deltas to reduce ledger load.
	 *
	 * @param wallet                  The wallet.
	 * @param credOfferJson           Cred offer created by issuerCreateCredentialOffer
	 * @param credReqJson             Credential request created by proverCreateCredentialReq
	 * @param credValuesJson          Credential containing attribute values for each of requested attribute names.
	 *                                Example:
	 *                                {
	 *                                  "attr1" : {"raw": "value1", "encoded": "value1_as_int" },
	 *                                  "attr2" : {"raw": "value1", "encoded": "value1_as_int" }
	 *                                }
	 * @param revRegId                (Optional) id of stored in ledger revocation registry definition
	 * @param blobStorageReaderHandle Pre-configured blob storage reader instance handle that will allow to read revocation tails
	 * @return A future resolving to a IssuerCreateCredentialResult containing:
	 * credentialJson: Credential json containing signed credential values
	 *     {
	 *         "schema_id": string,
	 *         "cred_def_id": string,
	 *         "rev_reg_def_id", Optional<string>,
	 *         "values": <see credValuesJson above>,
	 *         // Fields below can depend on Cred Def type
	 *         "signature": <signature>,
	 *         "signature_correctness_proof": <signature_correctness_proof>
	 *     }
	 * credRevocId: local id for revocation info (Can be used for revocation of this cred)
	 * revocRegDeltaJson: Revocation registry delta json with a newly issued credential
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<IssuerCreateCredentialResult> issuerCreateCredential(
			Wallet wallet,
			String credOfferJson,
			String credReqJson,
			String credValuesJson,
			String revRegId,
			int blobStorageReaderHandle) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(credOfferJson, "credOfferJson");
		ParamGuard.notNullOrWhiteSpace(credReqJson, "credReqJson");
		ParamGuard.notNullOrWhiteSpace(credValuesJson, "credValuesJson");

		CompletableFuture<IssuerCreateCredentialResult> future = new CompletableFuture<IssuerCreateCredentialResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_create_credential(
				commandHandle,
				walletHandle,
				credOfferJson,
				credReqJson,
				credValuesJson,
				revRegId,
				blobStorageReaderHandle,
				issuerCreateCredentialCb);

		checkResult(result);

		return future;
	}

	/**
	 * Revoke a credential identified by a cred_revoc_id (returned by issuerCreateCredential).
	 *
	 * The corresponding credential definition and revocation registry must be already
	 * created an stored into the wallet.
	 *
	 * This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
	 * Note that it is possible to accumulate deltas to reduce ledger load.
	 *
	 * @param wallet                  A wallet.
	 * @param blobStorageReaderHandle Pre-configured blob storage reader instance handle that will allow to read revocation tails
	 * @param revRegId                Id of revocation registry stored in wallet.
	 * @param credRevocId             Local id for revocation info
	 * @return A future resolving to a revocation registry delta json with a revoked credential
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> issuerRevokeCredential(
			Wallet wallet,
			int blobStorageReaderHandle,
			String revRegId,
			String credRevocId) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNull(revRegId, "revRegId");
		ParamGuard.notNull(credRevocId, "credRevocId");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_issuer_revoke_credential(
				commandHandle,
				walletHandle,
				blobStorageReaderHandle,
				revRegId,
				credRevocId,
				issuerRevokeCredentialCb);

		checkResult(result);

		return future;
	}

//	/**
//	 * Recover a credential identified by a cred_revoc_id (returned by indy_issuer_create_credential).
//	 * <p>
//	 * The corresponding credential definition and revocation registry must be already
//	 * created an stored into the wallet.
//	 * <p>
//	 * This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
//	 * Note that it is possible to accumulate deltas to reduce ledger load.
//	 *
//	 * @param wallet                  A wallet.
//	 * @param blobStorageReaderHandle Pre-configured blob storage reader instance handle that will allow to read revocation tails
//	 * @param revRegId                Id of revocation registry stored in wallet.
//	 * @param credRevocId             Local id for revocation info
//	 * @return A future resolving to a revocation registry update json with a recovered credential
//	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
//	 */
//	public static CompletableFuture<String> issuerRecoverCredential(
//			Wallet wallet,
//			int blobStorageReaderHandle,
//			String revRegId,
//			String credRevocId) throws IndyException {
//
//		ParamGuard.notNull(wallet, "wallet");
//		ParamGuard.notNull(revRegId, "revRegId");
//		ParamGuard.notNull(credRevocId, "credRevocId");
//
//		CompletableFuture<String> future = new CompletableFuture<String>();
//		int commandHandle = addFuture(future);
//
//		int walletHandle = wallet.getWalletHandle();
//
//		int result = LibIndy.api.indy_issuer_recover_credential(
//				commandHandle,
//				walletHandle,
//				blobStorageReaderHandle,
//				revRegId,
//				credRevocId,
//				issuerRecoverCredentialCb);
//
//		checkResult(result);
//
//		return future;
//	}

	/**
	 * Merge two revocation registry deltas (returned by issuerCreateCredential or issuerRevokeCredential) to accumulate common delta.
	 * Send common delta to ledger to reduce the load.
	 *
	 * @param revRegDelta      Revocation registry delta json.
	 * @param otherRevRegDelta Revocation registry delta for which PrevAccum value  is equal to current accum value of revRegDelta.
	 * @return A future resolving to a merged revocation registry delta.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> issuerMergeRevocationRegistryDeltas(
			String revRegDelta,
			String otherRevRegDelta) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(revRegDelta, "revRegDelta");
		ParamGuard.notNullOrWhiteSpace(otherRevRegDelta, "otherRevRegDelta");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_issuer_merge_revocation_registry_deltas(
				commandHandle,
				revRegDelta,
				otherRevRegDelta,
				issuerMergeRevocationRegistryDeltasCb);

		checkResult(result);

		return future;
	}

	/**
	 * Creates a master secret with a given name and stores it in the wallet.
	 *
	 * @param wallet         A wallet.
	 * @param masterSecretId (Optional, if not present random one will be generated) New master id
	 * @return A future resolving to id of generated master secret.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverCreateMasterSecret(
			Wallet wallet,
			String masterSecretId) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_create_master_secret(
				commandHandle,
				walletHandle,
				masterSecretId,
				proverCreateMasterSecretCb);

		checkResult(result);

		return future;
	}

	/**
	 * Creates a clam request for the given credential offer.
	 * 
	 * The method creates a blinded master secret for a master secret identified by a provided name.
	 * The master secret identified by the name must be already stored in the secure wallet (see proverCreateMasterSecret)
	 * The blinded master secret is a part of the credential request.
	 *
	 * @param wallet              A wallet.
	 * @param proverDid           The DID of the prover.
	 * @param credentialOfferJson Credential offer as a json containing information about the issuer and a credential
	 * @param credentialDefJson   Credential definition json
	 * @param masterSecretId      The id of the master secret stored in the wallet
	 * @return A future that resolves to:
	 * * credReqJson: Credential request json for creation of credential by Issuer
	 *     {
	 *      "prover_did" : string,
	 *      "cred_def_id" : string,
	 *         // Fields below can depend on Cred Def type
	 *      "blinded_ms" : <blinded_master_secret>,
	 *      "blinded_ms_correctness_proof" : <blinded_ms_correctness_proof>,
	 *      "nonce": string
	 *    }
	 * credReqMetadataJson: Credential request metadata json for processing of received form Issuer credential.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<ProverCreateCredentialRequestResult> proverCreateCredentialReq(
			Wallet wallet,
			String proverDid,
			String credentialOfferJson,
			String credentialDefJson,
			String masterSecretId) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(proverDid, "proverDid");
		ParamGuard.notNullOrWhiteSpace(credentialOfferJson, "credentialOfferJson");
		ParamGuard.notNullOrWhiteSpace(credentialDefJson, "credentialDefJson");
		ParamGuard.notNullOrWhiteSpace(masterSecretId, "masterSecretId");

		CompletableFuture<ProverCreateCredentialRequestResult> future = new CompletableFuture<ProverCreateCredentialRequestResult>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_create_credential_req(
				commandHandle,
				walletHandle,
				proverDid,
				credentialOfferJson,
				credentialDefJson,
				masterSecretId,
				proverCreateCredentialReqCb);

		checkResult(result);

		return future;
	}

	/**
	 * Check credential provided by Issuer for the given credential request,
	 * updates the credential by a master secret and stores in a secure wallet.
	 *
	 * @param wallet              A Wallet.
	 * @param credId              (optional, default is a random one) Identifier by which credential will be stored in the wallet
	 * @param credReqJson         Credential request created by proverCreateCredentialReq
	 * @param credReqMetadataJson Credential request metadata created by proverCreateCredentialReq
	 * @param credJson            Credential json received from issuer
	 * @param credDefJson         Credential definition json
	 * @param revRegDefJson       Revocation registry definition json
	 * @return A future that  resolve to identifier by which credential is stored in the wallet.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverStoreCredential(
			Wallet wallet,
			String credId,
			String credReqJson,
			String credReqMetadataJson,
			String credJson,
			String credDefJson,
			String revRegDefJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(credReqJson, "credReqJson");
		ParamGuard.notNullOrWhiteSpace(credReqMetadataJson, "credReqMetadataJson");
		ParamGuard.notNullOrWhiteSpace(credJson, "credJson");
		ParamGuard.notNullOrWhiteSpace(credDefJson, "credDefJson");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_store_credential(
				commandHandle,
				walletHandle,
				credId,
				credReqJson,
				credReqMetadataJson,
				credJson,
				credDefJson,
				revRegDefJson,
				proverStoreCredentialCb);

		checkResult(result);

		return future;
	}

	/**
	 * Gets human readable credentials according to the filter.
	 *
	 * @param wallet A wallet.
	 * @param filter for credentials
	 *        {
	 *            "schema_id": string, (Optional)
	 *            "schema_issuer_did": string, (Optional)
	 *            "schema_name": string, (Optional)
	 *            "schema_version": string, (Optional)
	 *            "issuer_did": string, (Optional)
	 *            "cred_def_id": string, (Optional)
	 *        }
	 * @return A future that resolves to a credentials json
	 *     [{
	 *         "referent": string, // cred_id in the wallet
	 *         "values": <see credValuesJson above>,
	 *         "schema_id": string,
	 *         "cred_def_id": string,
	 *         "rev_reg_id": Optional<string>,
	 *         "cred_rev_id": Optional<string>
	 *     }]
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverGetCredentials(
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
				proverGetCredentialsCb);

		checkResult(result);

		return future;
	}

	/**
	 * Gets human readable credentials matching the given proof request.
	 *
	 * @param wallet       A wallet.
	 * @param proofRequest proof request json
	 *     {
	 *         "name": string,
	 *         "version": string,
	 *         "nonce": string,
	 *         "requested_attributes": { // set of requested attributes
	 *              "<attr_referent>": <attr_info>, // see below
	 *              ...,
	 *         },
	 *         "requested_predicates": { // set of requested predicates
	 *              "<predicate_referent>": <predicate_info>, // see below
	 *              ...,
	 *          },
	 *         "non_revoked": Optional<<non_revoc_interval>>, // see below,
	 *                        // If specified prover must proof non-revocation
	 *                        // for date in this interval for each attribute
	 *                        // (can be overridden on attribute level)
	 *     }
	 *     where
	 *     attr_referent: Describes requested attribute
	 *     {
	 *         "name": string, // attribute name, (case insensitive and ignore spaces)
	 *         "restrictions": Optional<[<attr_filter>]> // see below,
	 *                          // if specified, credential must satisfy to one of the given restriction.
	 *         "non_revoked": Optional<<non_revoc_interval>>, // see below,
	 *                        // If specified prover must proof non-revocation
	 *                        // for date in this interval this attribute
	 *                        // (overrides proof level interval)
	 *     }
	 *     predicate_referent: Describes requested attribute predicate
	 *     {
	 *         "name": attribute name, (case insensitive and ignore spaces)
	 *         "p_type": predicate type (Currently >= only)
	 *         "p_value": predicate value
	 *         "restrictions": Optional<[<attr_filter>]> // see below,
	 *                         // if specified, credential must satisfy to one of the given restriction.
	 *         "non_revoked": Optional<<non_revoc_interval>>, // see below,
	 *                        // If specified prover must proof non-revocation
	 *                        // for date in this interval this attribute
	 *                        // (overrides proof level interval)
	 *     }
	 *     non_revoc_interval: Defines non-revocation interval
	 *     {
	 *         "from": Optional<int>, // timestamp of interval beginning
	 *         "to": Optional<int>, // timestamp of interval ending
	 *     }  
	 *     filter: see filter above                   
	 * @return A future that resolves to a json with credentials for the given pool request.
	 *     {
	 *         "requested_attrs": {
	 *             "<attr_referent>": [{ cred_info: <credential_info>, interval: Optional<non_revoc_interval> }],
	 *             ...,
	 *         },
	 *         "requested_predicates": {
	 *             "requested_predicates": [{ cred_info: <credential_info>, timestamp: Optional<integer> }, { cred_info: <credential_2_info>, timestamp: Optional<integer> }],
	 *             "requested_predicate_2_referent": [{ cred_info: <credential_2_info>, timestamp: Optional<integer> }]
	 *         }
	 *     }, where credential is
	 *     {
	 *         "referent": <string>,
	 *         "attrs": [{"attr_name" : "attr_raw_value"}],
	 *         "schema_id": string,
	 *         "cred_def_id": string,
	 *         "rev_reg_id": Optional<int>,
	 *         "cred_rev_id": Optional<int>,
	 *     }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverGetCredentialsForProofReq(
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
				proverGetCredentialsForProofReqCb);

		checkResult(result);

		return future;
	}

	/**
	 * Creates a proof according to the given proof request.
	 *
	 * @param wallet               A wallet.
	 * @param proofRequest proof request json
	 *     {
	 *         "name": string,
	 *         "version": string,
	 *         "nonce": string,
	 *         "requested_attributes": { // set of requested attributes
	 *              "<attr_referent>": <attr_info>, // see below
	 *              ...,
	 *         },
	 *         "requested_predicates": { // set of requested predicates
	 *              "<predicate_referent>": <predicate_info>, // see below
	 *              ...,
	 *          },
	 *         "non_revoked": Optional<<non_revoc_interval>>, // see below,
	 *                        // If specified prover must proof non-revocation
	 *                        // for date in this interval for each attribute
	 *                        // (can be overridden on attribute level)
	 *     }
	 * @param requestedCredentials either a credential or self-attested attribute for each requested attribute
	 *     {
	 *         "self_attested_attributes": {
	 *             "self_attested_attribute_referent": string
	 *         },
 	 *         "requested_attributes": {
	 *             "requested_attribute_referent_1": {"cred_id": string, "timestamp": Optional<number>, revealed: <bool> }},
	 *             "requested_attribute_referent_2": {"cred_id": string, "timestamp": Optional<number>, revealed: <bool> }}
	 *         },
	 *         "requested_predicates": {
	 *             "requested_predicates_referent_1": {"cred_id": string, "timestamp": Optional<number> }},
	 *         }
	 *     }
	 * @param masterSecret         Id of the master secret stored in the wallet
	 * @param schemas              All schemas json participating in the proof request
	 *     {
	 *         <schema1_id>: <schema1_json>,
	 *         <schema2_id>: <schema2_json>,
	 *         <schema3_id>: <schema3_json>,
	 *     }
	 * @param credentialDefs       All credential definitions json participating in the proof request
	 *     {
	 *         "cred_def1_id": <credential_def1_json>,
	 *         "cred_def2_id": <credential_def2_json>,
	 *         "cred_def3_id": <credential_def3_json>,
	 *     }
	 * @param revStates            All revocation states json participating in the proof request
	 *     {
	 *         "rev_reg_def1_id": {
	 *             "timestamp1": <rev_state1>,
	 *             "timestamp2": <rev_state2>,
	 *         },
	 *         "rev_reg_def2_id": {
	 *             "timestamp3": <rev_state3>
	 *         },
	 *         "rev_reg_def3_id": {
	 *             "timestamp4": <rev_state4>
	 *         },
	 *     }
	 * @return A future resolving to a Proof json
	 * For each requested attribute either a proof (with optionally revealed attribute value) or
	 * self-attested attribute value is provided.
	 * Each proof is associated with a credential and corresponding schema_id, cred_def_id, rev_reg_id and timestamp.
	 * There is also aggregated proof part common for all credential proofs.
	 *     {
	 *         "requested": {
	 *             "revealed_attrs": {
	 *                 "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
	 *                 "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
	 *             },
	 *             "unrevealed_attrs": {
	 *                 "requested_attr3_id": {sub_proof_index: number}
	 *             },
	 *             "self_attested_attrs": {
	 *                 "requested_attr2_id": self_attested_value,
	 *             },
	 *             "requested_predicates": {
	 *                 "requested_predicate_1_referent": {sub_proof_index: int},
	 *                 "requested_predicate_2_referent": {sub_proof_index: int},
	 *             }
	 *         }
	 *         "proof": {
	 *             "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
	 *             "aggregated_proof": <aggregated_proof>
	 *         }
	 *         "identifiers": [{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}]
	 *     }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> proverCreateProof(
			Wallet wallet,
			String proofRequest,
			String requestedCredentials,
			String masterSecret,
			String schemas,
			String credentialDefs,
			String revStates) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(proofRequest, "proofRequest");
		ParamGuard.notNullOrWhiteSpace(requestedCredentials, "requestedCredentials");
		ParamGuard.notNullOrWhiteSpace(schemas, "schemas");
		ParamGuard.notNullOrWhiteSpace(masterSecret, "masterSecret");
		ParamGuard.notNullOrWhiteSpace(credentialDefs, "credentialDefs");
		ParamGuard.notNullOrWhiteSpace(revStates, "revStates");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_prover_create_proof(
				commandHandle,
				walletHandle,
				proofRequest,
				requestedCredentials,
				masterSecret,
				schemas,
				credentialDefs,
				revStates,
				proverCreateProofCb);

		checkResult(result);

		return future;
	}

	/**
	 * Verifies a proof (of multiple credential).
	 * All required schemas, public keys and revocation registries must be provided.
	 *
	 * @param proofRequest   proof request json
	 *     {
	 *         "name": string,
	 *         "version": string,
	 *         "nonce": string,
	 *         "requested_attributes": { // set of requested attributes
	 *              "<attr_referent>": <attr_info>, // see below
	 *              ...,
	 *         },
	 *         "requested_predicates": { // set of requested predicates
	 *              "<predicate_referent>": <predicate_info>, // see below
	 *              ...,
	 *          },
	 *         "non_revoked": Optional<<non_revoc_interval>>, // see below,
	 *                        // If specified prover must proof non-revocation
	 *                        // for date in this interval for each attribute
	 *                        // (can be overridden on attribute level)
	 *     }
	 * @param proof          Proof json
	 {     {
	 *         "requested": {
	 *             "revealed_attrs": {
	 *                 "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
	 *                 "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
	 *             },
	 *             "unrevealed_attrs": {
	 *                 "requested_attr3_id": {sub_proof_index: number}
	 *             },
	 *             "self_attested_attrs": {
	 *                 "requested_attr2_id": self_attested_value,
	 *             },
	 *             "requested_predicates": {
	 *                 "requested_predicate_1_referent": {sub_proof_index: int},
	 *                 "requested_predicate_2_referent": {sub_proof_index: int},
	 *             }
	 *         }
	 *         "proof": {
	 *             "proofs": [ <credential_proof>, <credential_proof>, <credential_proof> ],
	 *             "aggregated_proof": <aggregated_proof>
	 *         }
	 *         "identifiers": [{schema_id, cred_def_id, Optional<rev_reg_id>, Optional<timestamp>}]
	 *     }
	 * @param schemas        All schemas json participating in the proof request
	 *     {
	 *         <schema1_id>: <schema1_json>,
	 *         <schema2_id>: <schema2_json>,
	 *         <schema3_id>: <schema3_json>,
	 *     }
	 * @param credentialDefs  All credential definitions json participating in the proof request
	 *     {
	 *         "cred_def1_id": <credential_def1_json>,
	 *         "cred_def2_id": <credential_def2_json>,
	 *         "cred_def3_id": <credential_def3_json>,
	 *     }
	 * @param revocRegDefs   All revocation registry definitions json participating in the proof
	 *     {
	 *         "rev_reg_def1_id": <rev_reg_def1_json>,
	 *         "rev_reg_def2_id": <rev_reg_def2_json>,
	 *         "rev_reg_def3_id": <rev_reg_def3_json>,
	 *     }
	 * @param revocRegs      all revocation registries json participating in the proof
	 *     {
	 *         "rev_reg_def1_id": {
	 *             "timestamp1": <rev_reg1>,
	 *             "timestamp2": <rev_reg2>,
	 *         },
	 *         "rev_reg_def2_id": {
	 *             "timestamp3": <rev_reg3>
	 *         },
	 *         "rev_reg_def3_id": {
	 *             "timestamp4": <rev_reg4>
	 *         },
	 *     }
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

	/**
	 * Create revocation state for credential in the particular time moment.
	 *
	 * @param blobStorageReaderHandle Configuration of blob storage reader handle that will allow to read revocation tails
	 * @param revRegDef               Revocation registry definition json
	 * @param revRegDelta             Revocation registry definition delta json
	 * @param timestamp               Time represented as a total number of seconds from Unix Epoch
	 * @param credRevId               user credential revocation id in revocation registry
	 * @return A future that resolves to a revocation state json:
	 *     {
	 *         "rev_reg": <revocation registry>,
	 *         "witness": <witness>,
	 *         "timestamp" : integer
	 *     }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> createRevocationState(
			int blobStorageReaderHandle,
			String revRegDef,
			String revRegDelta,
			int timestamp,
			String credRevId) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(revRegDef, "revRegDef");
		ParamGuard.notNullOrWhiteSpace(revRegDelta, "revRegDelta");
		ParamGuard.notNullOrWhiteSpace(credRevId, "credRevId");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_create_revocation_state(
				commandHandle,
				blobStorageReaderHandle,
				revRegDef,
				revRegDelta,
				timestamp,
				credRevId,
				createRevocationStateCb);

		checkResult(result);

		return future;
	}

	/**
	 * Create new revocation state for a credential based on already state
	 * at the particular time moment (to reduce calculation time).
	 *
	 * @param blobStorageReaderHandle Configuration of blob storage reader handle that will allow to read revocation tails
	 * @param revState                Rrevocation registry state json
	 * @param revRegDef               Revocation registry definition json
	 * @param revRegDelta             Revocation registry definition delta json
	 * @param timestamp               Time represented as a total number of seconds from Unix Epoch
	 * @param credRevId               user credential revocation id in revocation registry
	 * @return A future that resolves to a revocation state json:
	 *     {
	 *         "rev_reg": <revocation registry>,
	 *         "witness": <witness>,
	 *         "timestamp" : integer
	 *     }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> updateRevocationState(
			int blobStorageReaderHandle,
			String revState,
			String revRegDef,
			String revRegDelta,
			int timestamp,
			String credRevId) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(revState, "revState");
		ParamGuard.notNullOrWhiteSpace(revRegDef, "revRegDef");
		ParamGuard.notNullOrWhiteSpace(revRegDelta, "revRegDelta");
		ParamGuard.notNullOrWhiteSpace(credRevId, "credRevId");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_update_revocation_info(
				commandHandle,
				blobStorageReaderHandle,
				revState,
				revRegDef,
				revRegDelta,
				timestamp,
				credRevId,
				updateRevocationStateCb);

		checkResult(result);

		return future;
	}
}