package org.hyperledger.indy.sdk.ledger;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;

import org.hyperledger.indy.sdk.ledger.LedgerResults.ParseResponseResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.ParseRegistryResponseResult;

import com.sun.jna.Callback;

/**
 * ledger.rs API
 */

/**
 * Functionality related to the ledger.
 */
public class Ledger extends IndyJava.API {

	private Ledger() {

	}

	/* 
	 * STATIC CALLBACKS
	 */

	/**
	 * Callback used when signAndSubmitRequest completes.
	 */
	private static Callback signAndSubmitRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_result_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkResult(future, err)) return;

			String result = request_result_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when submitRequest completes.
	 */
	private static Callback submitRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_result_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkResult(future, err)) return;

			String result = request_result_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when signRequest completes.
	 */
	private static Callback signRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String signed_request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkResult(future, err)) return;

			String result = signed_request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildRequest completes.
	 */
	private static Callback buildRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkResult(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when parseRequest completes.
	 */
	private static Callback parseResponseCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String id, String object_json) {

			CompletableFuture<ParseResponseResult> future = (CompletableFuture<ParseResponseResult>) removeFuture(xcommand_handle);
			if (! checkResult(future, err)) return;

			ParseResponseResult result = new ParseResponseResult(id, object_json);
			future.complete(result);
		}
	};

	/**
	 * Callback used when parseRegistryRequest completes.
	 */
	private static Callback parseRegistryResponseCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String id, String object_json, long timestamp) {

			CompletableFuture<ParseRegistryResponseResult> future = (CompletableFuture<ParseRegistryResponseResult>) removeFuture(xcommand_handle);
			if (! checkResult(future, err)) return;

			ParseRegistryResponseResult result = new ParseRegistryResponseResult(id, object_json, timestamp);
			future.complete(result);
		}
	};


	/*
	 * STATIC METHODS
	 */

	/**
	 * Signs and submits request message to validator pool.
	 * <p>
	 * Adds submitter information to passed request json, signs it with submitter
	 * sign key (see wallet_sign), and sends signed request message
	 * to validator pool (see write_request).
	 *
	 * @param pool         A Pool.
	 * @param wallet       A Wallet.
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param requestJson  Request data json.
	 * @return A future resolving to a request result as json..
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> signAndSubmitRequest(
			Pool pool,
			Wallet wallet,
			String submitterDid,
			String requestJson) throws IndyException {

		ParamGuard.notNull(pool, "pool");
		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(requestJson, "requestJson");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int poolHandle = pool.getPoolHandle();
		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_sign_and_submit_request(
				commandHandle,
				poolHandle,
				walletHandle,
				submitterDid,
				requestJson,
				signAndSubmitRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Publishes request message to validator pool (no signing, unlike sign_and_submit_request).
	 * <p>
	 * The request is sent to the validator pool as is. It's assumed that it's already prepared.
	 *
	 * @param pool        The Pool to publish to.
	 * @param requestJson Request data json.
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> submitRequest(
			Pool pool,
			String requestJson) throws IndyException {

		ParamGuard.notNull(pool, "pool");
		ParamGuard.notNullOrWhiteSpace(requestJson, "requestJson");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int poolHandle = pool.getPoolHandle();

		int result = LibIndy.api.indy_submit_request(
				commandHandle,
				poolHandle,
				requestJson,
				submitRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Send action to particular nodes of validator pool.
	 *
	 * The list of requests can be send:
	 *     POOL_RESTART
	 *     GET_VALIDATOR_INFO
	 *
	 * The request is sent to the nodes as is. It's assumed that it's already prepared.
	 *
	 * @param pool        The Pool to publish to.
	 * @param requestJson Request data json.
	 * @param nodes      (Optional) List of node names to send the request.
	 *                   ["Node1", "Node2",...."NodeN"]
	 * @param timeout    (Optional) Time to wait respond from nodes (override the default timeout) (in sec).
	 *                   Pass -1 to use default timeout
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> submitAction(
			Pool pool,
			String requestJson,
			String nodes,
			int timeout) throws IndyException {

		ParamGuard.notNull(pool, "pool");
		ParamGuard.notNullOrWhiteSpace(requestJson, "requestJson");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int poolHandle = pool.getPoolHandle();

		int result = LibIndy.api.indy_submit_action(
				commandHandle,
				poolHandle,
				requestJson,
				nodes,
				timeout,
				submitRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Signs request message.
	 * <p>
	 * dds submitter information to passed request json, signs it with submitter
	 * sign key (see wallet_sign).
	 *
	 * @param wallet       A Wallet.
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param requestJson  Request data json.
	 * @return A future resolving to a signed request json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> signRequest(
			Wallet wallet,
			String submitterDid,
			String requestJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(requestJson, "requestJson");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_sign_request(
				commandHandle,
				walletHandle,
				submitterDid,
				requestJson,
				signRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Multi signs request message.
	 * <p>
	 * Adds submitter information to passed request json, signs it with submitter
	 * sign key (see wallet_sign).
	 *
	 * @param wallet       A Wallet.
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param requestJson  Request data json.
	 * @return A future resolving to a signed request json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> multiSignRequest(
			Wallet wallet,
			String submitterDid,
			String requestJson) throws IndyException {

		ParamGuard.notNull(wallet, "wallet");
		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(requestJson, "requestJson");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_multi_sign_request(
				commandHandle,
				walletHandle,
				submitterDid,
				requestJson,
				signRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a request to get a DDO.
	 *
	 * @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
	 * @param targetDid    Id of Identity stored in secured Wallet.
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetDdoRequest(
			String submitterDid,
			String targetDid) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(targetDid, "targetDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_ddo_request(
				commandHandle,
				submitterDid,
				targetDid,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a NYM request. Request to create a new NYM record for a specific user.
	 *
	 * @param submitterDid DID of the submitter stored in secured Wallet.
	 * @param targetDid    Target DID as base58-encoded string for 16 or 32 bit DID value.
	 * @param verkey       Target identity verification key as base58-encoded string.
	 * @param alias        NYM's alias.
	 * @param role         Role of a user NYM record:
	 *                     null (common USER)
	 *                     TRUSTEE
	 *                     STEWARD
	 *                     TRUST_ANCHOR
	 *                     NETWORK_MONITOR
	 *                     empty string to reset role
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildNymRequest(
			String submitterDid,
			String targetDid,
			String verkey,
			String alias,
			String role) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(targetDid, "targetDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_nym_request(
				commandHandle,
				submitterDid,
				targetDid,
				verkey,
				alias,
				role,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds an ATTRIB request. Request to add attribute to a NYM record.
	 *
	 * @param submitterDid DID of the submitter stored in secured Wallet.
	 * @param targetDid    Target DID as base58-encoded string for 16 or 32 bit DID value.
	 * @param hash         (Optional) Hash of attribute data.
	 * @param raw          (Optional) Json, where key is attribute name and value is attribute value.
	 * @param enc          (Optional) Encrypted value attribute data.
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildAttribRequest(
			String submitterDid,
			String targetDid,
			String hash,
			String raw,
			String enc) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(targetDid, "targetDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_attrib_request(
				commandHandle,
				submitterDid,
				targetDid,
				hash,
				raw,
				enc,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a GET_ATTRIB request. Request to get information about an Attribute for the specified DID.
	 *
	 * @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
	 * @param targetDid    Target DID as base58-encoded string for 16 or 32 bit DID value.
	 * @param raw          (Optional) Requested attribute name.
	 * @param hash         (Optional) Requested attribute hash.
	 * @param enc          (Optional) Requested attribute encrypted value.
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetAttribRequest(
			String submitterDid,
			String targetDid,
			String raw,
			String hash,
			String enc) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(targetDid, "targetDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_attrib_request(
				commandHandle,
				submitterDid,
				targetDid,
				raw,
				hash,
				enc,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a GET_NYM request. Request to get information about a DID (NYM).
	 *
	 * @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
	 * @param targetDid    Target DID as base58-encoded string for 16 or 32 bit DID value.
	 * @return A future resolving to a request result as json..
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetNymRequest(
			String submitterDid,
			String targetDid) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(targetDid, "targetDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_nym_request(
				commandHandle,
				submitterDid,
				targetDid,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a SCHEMA request. Request to add Credential's schema.
	 *
	 * @param submitterDid DID of the submitter stored in secured Wallet.
	 * @param data         Credential schema.
	 *                     {
	 *                         id: identifier of schema
	 *                         attrNames: array of attribute name strings (the number of attributes should be less or equal than 125)
	 *                         name: Schema's name string
	 *                         version: Schema's version string,
	 *                         ver: Version of the Schema json
	 *                     }
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildSchemaRequest(
			String submitterDid,
			String data) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(data, "data");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_schema_request(
				commandHandle,
				submitterDid,
				data,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a GET_SCHEMA request. Request to get Credential's Schema.
	 *
	 * @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
	 * @param id           Schema ID in ledger
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetSchemaRequest(
			String submitterDid,
			String id) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(id, "id");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_schema_request(
				commandHandle,
				submitterDid,
				id,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Parse a GET_SCHEMA response to get Schema in the format compatible with Anoncreds API
	 *
	 * @param getSchemaResponse response of GET_SCHEMA request.
	 * @return A future resolving to a Schema Id and Schema json.
	 * {
	 *     id: identifier of schema
	 *     attrNames: array of attribute name strings
	 *     name: Schema's name string
	 *     version: Schema's version string
	 *     ver: Version of the Schema json
	 * }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<ParseResponseResult> parseGetSchemaResponse(
			String getSchemaResponse) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(getSchemaResponse, "data");

		CompletableFuture<ParseResponseResult> future = new CompletableFuture<ParseResponseResult>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_parse_get_schema_response(
				commandHandle,
				getSchemaResponse,
				parseResponseCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds an CRED_DEF request. Request to add a credential definition (in particular, public key),
	 * that Issuer creates for a particular Credential Schema.
	 *
	 * @param submitterDid DID of the submitter stored in secured Wallet.
	 * @param data         Credential definition json
	 * {
	 *     id: string - identifier of credential definition
	 *     schemaId: string - identifier of stored in ledger schema
	 *     type: string - type of the credential definition. CL is the only supported type now.
	 *     tag: string - allows to distinct between credential definitions for the same issuer and schema
	 *     value: Dictionary with Credential Definition's data: {
	 *         primary: primary credential public key,
	 *         Optional[revocation]: revocation credential public key
	 *     },
	 *     ver: Version of the CredDef json
	 * }
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildCredDefRequest(
			String submitterDid,
			String data) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(data, "data");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_cred_def_request(
				commandHandle,
				submitterDid,
				data,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a GET_CRED_DEF request. Request to get a credential definition (in particular, public key),
	 * that Issuer creates for a particular Credential Schema.
	 *
	 * @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
	 * @param id           Credential Definition ID in ledger.
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetCredDefRequest(
			String submitterDid,
			String id) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(id, "id");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_cred_def_request(
				commandHandle,
				submitterDid,
				id,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Parse a GET_CRED_DEF response to get Credential Definition in the format compatible with Anoncreds API.
	 *
	 * @param getCredDefResponse response of GET_CRED_DEF request.
	 * @return A future resolving to a Credential Definition Id and Credential Definition json.
	 * {
	 *     id: string - identifier of credential definition
	 *     schemaId: string - identifier of stored in ledger schema
	 *     type: string - type of the credential definition. CL is the only supported type now.
	 *     tag: string - allows to distinct between credential definitions for the same issuer and schema
	 *     value: Dictionary with Credential Definition's data: {
	 *         primary: primary credential public key,
	 *         Optional[revocation]: revocation credential public key
	 *     },
	 *     ver: Version of the Credential Definition json
	 * }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<ParseResponseResult> parseGetCredDefResponse(
			String getCredDefResponse) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(getCredDefResponse, "getCredDefResponse");

		CompletableFuture<ParseResponseResult> future = new CompletableFuture<ParseResponseResult>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_parse_get_cred_def_response(
				commandHandle,
				getCredDefResponse,
				parseResponseCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.
	 *
	 * @param submitterDid DID of the submitter stored in secured Wallet.
	 * @param targetDid    Target Node's DID. It differs from submitter_did field.
	 * @param data         Data associated with the Node: {
	 *     alias: string - Node's alias
	 *     blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
	 *     blskey_pop: string - (Optional) BLS key proof of possession as base58-encoded string.
	 *     client_ip: string - (Optional) Node's client listener IP address.
	 *     client_port: string - (Optional) Node's client listener port.
	 *     node_ip: string - (Optional) The IP address other Nodes use to communicate with this Node.
	 *     node_port: string - (Optional) The port other Nodes use to communicate with this Node.
	 *     services: array["string"] - (Optional) The service of the Node. VALIDATOR is the only supported one now.
	 * }
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildNodeRequest(
			String submitterDid,
			String targetDid,
			String data) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(targetDid, "targetDid");
		ParamGuard.notNullOrWhiteSpace(data, "data");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_node_request(
				commandHandle,
				submitterDid,
				targetDid,
				data,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a GET_VALIDATOR_INFO request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetValidatorInfoRequest(
			String submitterDid) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_validator_info_request(
				commandHandle,
				submitterDid,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a GET_TXN request. Request to get any transaction by its seq_no.
	 *
	 * @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
	 * @param ledgerType  (Optional) type of the ledger the requested transaction belongs to:
	 *    DOMAIN - used default,
	 *    POOL,
	 *    CONFIG
	 *    any number
	 * @param seqNo         requested transaction sequence number as it's stored on Ledger.
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetTxnRequest(
			String submitterDid,
			String ledgerType,
			int seqNo) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_txn_request(
				commandHandle,
				submitterDid,
				ledgerType,
				seqNo,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a POOL_CONFIG request. Request to change Pool's configuration.
	 *
	 * @param submitterDid DID of the submitter stored in secured Wallet.
	 * @param writes       Whether any write requests can be processed by the pool
	 *                     (if false, then pool goes to read-only state). True by default.
	 * @param force        Whether we should apply transaction (for example, move pool to read-only state)
	 *                     without waiting for consensus of this transaction.
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildPoolConfigRequest(
			String submitterDid,
			boolean writes,
			boolean force) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_pool_config_request(
				commandHandle,
				submitterDid,
				writes,
				force,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a POOL_RESTART request.
	 *
	 * @param submitterDid Id of Identity that sender transaction
	 * @param action       Action that pool has to do after received transaction. Can be "start" or "cancel"
	 * @param datetime     Restart time in datetime format. Skip to restart as early as possible.
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildPoolRestartRequest(
			String submitterDid,
			String action,
			String datetime) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_pool_restart_request(
				commandHandle,
				submitterDid,
				action,
				datetime,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee).
	 * It upgrades the specified Nodes (either all nodes in the Pool, or some specific ones).
	 *
	 * @param submitterDid  DID of the submitter stored in secured Wallet.
	 * @param name          Human-readable name for the upgrade.
	 * @param version       The version of indy-node package we perform upgrade to.
	 *                      Must be greater than existing one (or equal if reinstall flag is True).
	 * @param action        Either start or cancel.
	 * @param sha256        sha256 hash of the package.
	 * @param timeout       (Optional) Limits upgrade time on each Node.
	 * @param schedule      (Optional) Schedule of when to perform upgrade on each node. Map Node DIDs to upgrade time.
	 * @param justification (Optional) justification string for this particular Upgrade.
	 * @param reinstall     Whether it's allowed to re-install the same version. False by default.
	 * @param force         Whether we should apply transaction (schedule Upgrade) without waiting for consensus of this transaction.
	 * @param package_      (Optional) Package to be upgraded.
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildPoolUpgradeRequest(
			String submitterDid,
			String name,
			String version,
			String action,
			String sha256,
			int timeout,
			String schedule,
			String justification,
			boolean reinstall,
			boolean force,
			String package_) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_pool_upgrade_request(
				commandHandle,
				submitterDid,
				name,
				version,
				action,
				sha256,
				timeout,
				schedule,
				justification,
				reinstall,
				force,
				package_,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
	 * to an exists credential definition.
	 *
	 * @param submitterDid DID of the submitter stored in secured Wallet.
	 * @param data         Revocation Registry data:
	 *     {
	 *         "id": string - ID of the Revocation Registry,
	 *         "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
	 *         "tag": string - Unique descriptive ID of the Registry,
	 *         "credDefId": string - ID of the corresponding CredentialDefinition,
	 *         "value": Registry-specific data {
	 *             "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
	 *             "maxCredNum": number - Maximum number of credentials the Registry can serve.
	 *             "tailsHash": string - Hash of tails.
	 *             "tailsLocation": string - Location of tails file.
	 *             "publicKeys": {public_keys} - Registry's public key.
	 *         },
	 *         "ver": string - version of revocation registry definition json.
	 *     }
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildRevocRegDefRequest(
			String submitterDid,
			String data) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_revoc_reg_def_request(
				commandHandle,
				submitterDid,
				data,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
	 * that Issuer creates for a particular Credential Definition.
	 *
	 * @param submitterDid (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
	 * @param id           ID of Revocation Registry Definition in ledger.
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetRevocRegDefRequest(
			String submitterDid,
			String id) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(id, "id");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_revoc_reg_def_request(
				commandHandle,
				submitterDid,
				id,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Parse a GET_REVOC_REG_DEF response to get Revocation Registry Definition in the format compatible with Anoncreds API.
	 *
	 * @param getRevocRegDefResponse response of GET_REVOC_REG_DEF request.
	 * @return A future resolving to a Revocation Registry Definition Id and Revocation Registry Definition json.
	 * {
	 *     "id": string - ID of the Revocation Registry,
	 *     "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
	 *     "tag": string - Unique descriptive ID of the Registry,
	 *     "credDefId": string - ID of the corresponding CredentialDefinition,
	 *     "value": Registry-specific data {
	 *         "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
	 *         "maxCredNum": number - Maximum number of credentials the Registry can serve.
	 *         "tailsHash": string - Hash of tails.
	 *         "tailsLocation": string - Location of tails file.
	 *         "publicKeys": {public_keys} - Registry's public key.
	 *     },
	 *     "ver": string - version of revocation registry definition json.
	 * }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<ParseResponseResult> parseGetRevocRegDefResponse(
			String getRevocRegDefResponse) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(getRevocRegDefResponse, "data");

		CompletableFuture<ParseResponseResult> future = new CompletableFuture<ParseResponseResult>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_parse_get_revoc_reg_def_response(
				commandHandle,
				getRevocRegDefResponse,
				parseResponseCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
	 * the new accumulator value and issued/revoked indices.
	 * This is just a delta of indices, not the whole list.
	 * So, it can be sent each time a new credential is issued/revoked.
	 *
	 * @param submitterDid  DID of the submitter stored in secured Wallet.
	 * @param revocRegDefId ID of the corresponding RevocRegDef.
	 * @param revDefType    Revocation Registry type (only CL_ACCUM is supported for now).
	 * @param value         Registry-specific data: {
	 *     value: {
	 *         prevAccum: string - previous accumulator value.
	 *         accum: string - current accumulator value.
	 *         issued: array[number] - an array of issued indices.
	 *         revoked: array[number] an array of revoked indices.
	 *     },
	 *     ver: string - version revocation registry entry json
	 *
	 * }
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildRevocRegEntryRequest(
			String submitterDid,
			String revocRegDefId,
			String revDefType,
			String value) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(revocRegDefId, "revocRegDefId");
		ParamGuard.notNullOrWhiteSpace(revDefType, "revDefType");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_revoc_reg_entry_request(
				commandHandle,
				submitterDid,
				revocRegDefId,
				revDefType,
				value,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry
	 * by ID. The state is defined by the given timestamp.
	 *
	 * @param submitterDid  (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
	 * @param revocRegDefId ID of the corresponding Revocation Registry Definition in ledger.
	 * @param timestamp     Requested time represented as a total number of seconds from Unix Epoch
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetRevocRegRequest(
			String submitterDid,
			String revocRegDefId,
			long timestamp) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(revocRegDefId, "id");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_revoc_reg_request(
				commandHandle,
				submitterDid,
				revocRegDefId,
				timestamp,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Parse a GET_REVOC_REG response to get Revocation Registry in the format compatible with Anoncreds API.
	 *
	 * @param getRevocRegResponse response of GET_REVOC_REG request.
	 * @return A future resolving to a Revocation Registry Definition Id, Revocation Registry json and Timestamp.
	 * {
	 *     "value": Registry-specific data {
	 *         "accum": string - current accumulator value.
	 *     },
	 *     "ver": string - version revocation registry json
	 * }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<ParseRegistryResponseResult> parseGetRevocRegResponse(
			String getRevocRegResponse) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(getRevocRegResponse, "data");

		CompletableFuture<ParseRegistryResponseResult> future = new CompletableFuture<ParseRegistryResponseResult>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_parse_get_revoc_reg_response(
				commandHandle,
				getRevocRegResponse,
				parseRegistryResponseCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
	 * The Delta is defined by from and to timestamp fields.
	 * If from is not specified, then the whole state till to will be returned.
	 *
	 * @param submitterDid  (Optional) DID of the read request sender (if not provided then default Libindy DID will be used).
	 * @param revocRegDefId ID of the corresponding Revocation Registry Definition in ledger.
	 * @param from          Requested time represented as a total number of seconds from Unix Epoch
	 * @param to            Requested time represented as a total number of seconds from Unix Epoch
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetRevocRegDeltaRequest(
			String submitterDid,
			String revocRegDefId,
			long from,
			long to) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(revocRegDefId, "id");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_revoc_reg_delta_request(
				commandHandle,
				submitterDid,
				revocRegDefId,
				from,
				to,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Parse a GET_REVOC_REG_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.
	 *
	 * @param getRevocRegDeltaResponse response of GET_REVOC_REG_DELTA request.
	 * @return A future resolving to a Revocation Registry Definition Id, Revocation Registry Delta json and Timestamp.
	 * {
	 *     "value": Registry-specific data {
	 *         prevAccum: string - previous accumulator value.
	 *         accum: string - current accumulator value.
	 *         issued: array[number] - an array of issued indices.
	 *         revoked: array[number] an array of revoked indices.
	 *     },
	 *     "ver": string
	 * }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<ParseRegistryResponseResult> parseGetRevocRegDeltaResponse(
			String getRevocRegDeltaResponse) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(getRevocRegDeltaResponse, "data");

		CompletableFuture<ParseRegistryResponseResult> future = new CompletableFuture<ParseRegistryResponseResult>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_parse_get_revoc_reg_delta_response(
				commandHandle,
				getRevocRegDeltaResponse,
				parseRegistryResponseCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Parse transaction response to fetch metadata.
	 * The important use case for this method is validation of Node's response freshens.
	 *
	 * Distributed Ledgers can reply with outdated information for consequence read request after write.
	 * To reduce pool load libindy sends read requests to one random node in the pool.
	 * Consensus validation is performed based on validation of nodes multi signature for current ledger Merkle Trie root.
	 * This multi signature contains information about the latest ldeger's transaction ordering time and sequence number that this method returns.
	 *
	 * If node that returned response for some reason is out of consensus and has outdated ledger
	 * it can be caught by analysis of the returned latest ledger's transaction ordering time and sequence number.
	 *
	 * There are two ways to filter outdated responses:
	 *     1) based on "seqNo" - sender knows the sequence number of transaction that he consider as a fresh enough.
	 *     2) based on "txnTime" - sender knows the timestamp that he consider as a fresh enough.
	 *
	 * Note: response of GET_VALIDATOR_INFO request isn't supported
	 *
	 * @param response response of write or get request.
	 * @return A future resolving to a Response Metadata.
	 * {
	 *     "seqNo": Option<u64> - transaction sequence number,
	 *     "txnTime": Option<u64> - transaction ordering time,
	 *     "lastSeqNo": Option<u64> - the latest transaction seqNo for particular Node,
	 *     "lastTxnTime": Option<u64> - the latest transaction ordering time for particular Node
	 * }
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> getResponseMetadata(
			String response) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(response, "response");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_get_response_metadata(
				commandHandle,
				response,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a AUTH_RULE request. Request to change authentication rules for a ledger transaction.
	 *
	 * @param submitterDid DID of the submitter stored in secured Wallet.
	 * @param txnType - ledger transaction alias or associated value.
	 * @param action - type of an action.
	 *     Can be either "ADD" (to add a new rule) or "EDIT" (to edit an existing one).
	 * @param field - transaction field.
	 * @param oldValue - old value of a field, which can be changed to a new_value (mandatory for EDIT action).
	 * @param newValue - new value that can be used to fill the field.
	 * @param constraint - set of constraints required for execution of an action in the following format:
	 *     {
	 *         constraint_id - [string] type of a constraint.
	 *             Can be either "ROLE" to specify final constraint or  "AND"/"OR" to combine constraints.
	 *         role - [string] role of a user which satisfy to constrain.
	 *         sig_count - [u32] the number of signatures required to execution action.
	 *         need_to_be_owner - [bool] if user must be an owner of transaction.
	 *         metadata - [object] additional parameters of the constraint.
	 *     }
	 * can be combined by
	 *     {
	 *         'constraint_id': "AND" or "OR"
	 *         'auth_constraints': [[constraint_1], [constraint_2]]
	 *     }
	 *
	 * Default ledger auth rules: https://github.com/hyperledger/indy-node/blob/master/docs/source/auth_rules.md
	 *
	 * More about AUTH_RULE request: https://github.com/hyperledger/indy-node/blob/master/docs/source/requests.md#auth_rule
	 *
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildAuthRuleRequest(
			String submitterDid,
			String txnType,
			String action,
			String field,
			String oldValue,
			String newValue,
			String constraint) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(txnType, "txnType");
		ParamGuard.notNullOrWhiteSpace(action, "action");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_auth_rule_request(
				commandHandle,
				submitterDid,
				txnType,
				action,
				field,
				oldValue,
				newValue,
				constraint,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}

	/**
	 * Builds a GET_AUTH_RULE request. Request to get authentication rules for a ledger transaction.
	 *
	 * NOTE: Either none or all transaction related parameters must be specified (`oldValue` can be skipped for `ADD` action).
	 *     * none - to get all authentication rules for all ledger transactions
	 *     * all - to get authentication rules for specific action (`oldValue` can be skipped for `ADD` action)
	 * 
	 * @param submitterDid (Optional) DID of the read request sender.
	 * @param txnType - (Optional) target ledger transaction alias or associated value.
	 * @param action - (Optional) type of action for which authentication rules will be applied.
	 *     Can be either "ADD" (to add new rule) or "EDIT" (to edit an existing one).
	 * @param field - (Optional) transaction field for which authentication rule will be applied.
	 * @param oldValue - (Optional) old value of field, which can be changed to a new_value (must be specified for EDIT action).
	 * @param newValue - (Optional) new value that can be used to fill the field.
	 *
	 * @return A future resolving to a request result as json.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetAuthRuleRequest(
			String submitterDid,
			String txnType,
			String action,
			String field,
			String oldValue,
			String newValue) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(txnType, "txnType");
		ParamGuard.notNullOrWhiteSpace(action, "action");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_auth_rule_request(
				commandHandle,
				submitterDid,
				txnType,
				action,
				field,
				oldValue,
				newValue,
				buildRequestCb);

		checkResult(future, result);

		return future;
	}
}
