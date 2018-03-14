package org.hyperledger.indy.sdk.ledger;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ParamGuard;
import org.hyperledger.indy.sdk.StringUtils;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;

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
			if (! checkCallback(future, err)) return;

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
			if (! checkCallback(future, err)) return;

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
			if (! checkCallback(future, err)) return;

			String result = signed_request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildGetDdoRequest completes.
	 */
	private static Callback buildGetDdoRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildNymRequest completes.
	 */
	private static Callback buildNymRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildAttribRequest completes.
	 */
	private static Callback buildAttribRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildGetAttribRequest completes.
	 */
	private static Callback buildGetAttribRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildGetNymRequest completes.
	 */
	private static Callback buildGetNymRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildSchemaRequest completes.
	 */
	private static Callback buildSchemaRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildGetSchemaRequest completes.
	 */
	private static Callback buildGetSchemaRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildClaimDefTxn completes.
	 */
	private static Callback buildClaimDefTxnCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildGetClaimDefTxn completes.
	 */
	private static Callback buildGetClaimDefTxnCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildNodeRequest completes.
	 */
	private static Callback buildNodeRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildGetTxnRequest completes.
	 */
	public static Callback buildGetTxnRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildPoolConfigRequest completes.
	 */
	public static Callback buildPoolConfigRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/**
	 * Callback used when buildPoolUpgradeRequest completes.
	 */
	public static Callback buildPoolUpgradeRequestCb = new Callback() {

		@SuppressWarnings({"unused", "unchecked"})
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	/*
	 * STATIC METHODS
	 */

	/**
	 * Signs and submits request message to validator pool.
	 *
	 * @param pool         A Pool.
	 * @param wallet       A Wallet.
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param requestJson  Request data json.
	 * @return A future resolving to a JSON request string.
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

		checkResult(result);

		return future;
	}

	/**
	 * Publishes request message to validator pool (no signing, unlike sign_and_submit_request).
	 *
	 * @param pool        The Pool to publish to.
	 * @param requestJson Request data json.
	 * @return A future resolving to a JSON request string.
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

		checkResult(result);

		return future;
	}

	/**
	 * Signs request message.
	 *
	 * @param wallet       A Wallet.
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param requestJson  Request data json.
	 * @return A future resolving to a JSON request string.
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

		checkResult(result);

		return future;
	}

	/**
	 * Builds a request to get a DDO.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param targetDid    Id of Identity stored in secured Wallet.
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetDdoRequest(
			String submitterDid,
			String targetDid) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(targetDid, "targetDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_ddo_request(
				commandHandle,
				submitterDid,
				targetDid,
				buildGetDdoRequestCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds a NYM request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param targetDid    Id of Identity stored in secured Wallet.
	 * @param verkey       verification key
	 * @param alias        alias
	 * @param role         Role of a user NYM record
	 * @return A future resolving to a JSON request string.
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
				buildNymRequestCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds an ATTRIB request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param targetDid    Id of Identity stored in secured Wallet.
	 * @param hash         Hash of attribute data
	 * @param raw          represented as json, where key is attribute name and value is it's value
	 * @param enc          Encrypted attribute data
	 * @return A future resolving to a JSON request string.
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
				buildAttribRequestCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds a GET_ATTRIB request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param targetDid    Id of Identity stored in secured Wallet.
	 * @param raw          represented as json, where key is attribute name and value is it's value
	 * @param hash         Hash of attribute data
	 * @param enc          Encrypted attribute data
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetAttribRequest(
			String submitterDid,
			String targetDid,
			String raw,
			String hash,
			String enc) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
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
				buildGetAttribRequestCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds a GET_NYM request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param targetDid    Id of Identity stored in secured Wallet.
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetNymRequest(
			String submitterDid,
			String targetDid) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(targetDid, "targetDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_nym_request(
				commandHandle,
				submitterDid,
				targetDid,
				buildGetNymRequestCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds a SCHEMA request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param data         name, version, type, attr_names (ip, port, keys)
	 * @return A future resolving to a JSON request string.
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
				buildSchemaRequestCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds a GET_SCHEMA request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param dest         Id of Identity stored in secured Wallet.
	 * @param data         name, version
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetSchemaRequest(
			String submitterDid,
			String dest,
			String data) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(dest, "dest");
		ParamGuard.notNullOrWhiteSpace(data, "data");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_schema_request(
				commandHandle,
				submitterDid,
				dest,
				data,
				buildGetSchemaRequestCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds an CLAIM_DEF request.
	 *
	 * @param submitterDid  Id of Identity stored in secured Wallet.
	 * @param xref          Seq. number of schema
	 * @param signatureType signature type (only CL supported now)
	 * @param data          components of a key in json: N, R, S, Z
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildClaimDefTxn(
			String submitterDid,
			int xref,
			String signatureType,
			String data) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(signatureType, "signatureType");
		ParamGuard.notNullOrWhiteSpace(data, "data");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_claim_def_txn(
				commandHandle,
				submitterDid,
				xref,
				signatureType,
				data,
				buildClaimDefTxnCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds a GET_CLAIM_DEF request.
	 *
	 * @param submitterDid  Id of Identity stored in secured Wallet.
	 * @param xref          Seq. number of schema
	 * @param signatureType signature type (only CL supported now)
	 * @param origin        issuer did
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetClaimDefTxn(
			String submitterDid,
			int xref,
			String signatureType,
			String origin) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");
		ParamGuard.notNullOrWhiteSpace(signatureType, "signatureType");
		ParamGuard.notNullOrWhiteSpace(origin, "origin");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_claim_def_txn(
				commandHandle,
				submitterDid,
				xref,
				signatureType,
				origin,
				buildGetClaimDefTxnCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds a NODE request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param targetDid    Id of Identity stored in secured Wallet.
	 * @param data         id of a target NYM record
	 * @return A future resolving to a JSON request string.
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
				buildNodeRequestCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds a GET_TXN request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param data         seq_no of transaction in ledger
	 * @return A future resolving to a JSON request string.
	 * @throws IndyException Thrown if an error occurs when calling the underlying SDK.
	 */
	public static CompletableFuture<String> buildGetTxnRequest(
			String submitterDid,
			int data) throws IndyException {

		ParamGuard.notNullOrWhiteSpace(submitterDid, "submitterDid");

		CompletableFuture<String> future = new CompletableFuture<String>();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_txn_request(
				commandHandle,
				submitterDid,
				data,
				buildGetTxnRequestCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds a POOL_CONFIG request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param writes
	 * @param force
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
				buildPoolConfigRequestCb);

		checkResult(result);

		return future;
	}

	/**
	 * Builds a POOL_UPGRADE request.
	 *
	 * @param submitterDid Id of Identity stored in secured Wallet.
	 * @param name
	 * @param version
	 * @param action
	 * @param sha256
	 * @param timeout
	 * @param schedule
	 * @param justification
	 * @param reinstall
	 * @param force
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
			boolean force) throws IndyException {

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
				buildPoolUpgradeRequestCb);

		checkResult(result);

		return future;
	}
}
