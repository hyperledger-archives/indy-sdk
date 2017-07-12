package org.hyperledger.indy.sdk.ledger;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.pool.Pool;
import org.hyperledger.indy.sdk.wallet.Wallet;

import com.sun.jna.Callback;

/**
 * ledger.rs API
 */
public class Ledger extends IndyJava.API {

	private Ledger() {

	}

	/* 
	 * STATIC CALLBACKS
	 */

	private static Callback signAndSubmitRequestCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_result_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_result_json;
			future.complete(result);
		}
	};

	private static Callback submitRequestCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_result_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_result_json;
			future.complete(result);
		}
	};

	private static Callback buildGetDdoRequestCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	private static Callback buildNymRequestCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	private static Callback buildAttribRequestCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	private static Callback buildGetAttribRequestCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	private static Callback buildGetNymRequestCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	private static Callback buildSchemaRequestCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	private static Callback buildGetSchemaRequestCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	private static Callback buildClaimDefTxnCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	private static Callback buildGetClaimDefTxnCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
		public void callback(int xcommand_handle, int err, String request_json) {

			CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
			if (! checkCallback(future, err)) return;

			String result = request_json;
			future.complete(result);
		}
	};

	private static Callback buildNodeRequestCb = new Callback() {

		@SuppressWarnings({ "unused", "unchecked" })
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

	public static CompletableFuture<String> signAndSubmitRequest(
			Pool pool,
			Wallet wallet,
			String submitterDid,
			String requestJson) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
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

	public static CompletableFuture<String> submitRequest(
			Pool pool,
			String requestJson) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
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

	public static CompletableFuture<String> buildGetDdoRequest(
			String submitterDid,
			String targetDid,
			String requestJson) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_ddo_request(
				commandHandle, 
				submitterDid,
				targetDid,
				buildGetDdoRequestCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildNymRequest(
			String submitterDid,
			String targetDid,
			String verkey,
			String alias,
			String role) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
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

	public static CompletableFuture<String> buildAttribRequest(
			String submitterDid,
			String targetDid,
			String hash,
			String raw,
			String enc) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
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

	public static CompletableFuture<String> buildGetAttribRequest(
			String submitterDid,
			String targetDid,
			String data) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_attrib_request(
				commandHandle, 
				submitterDid,
				targetDid,
				data,
				buildGetAttribRequestCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildGetNymRequest(
			String submitterDid,
			String targetDid) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_nym_request(
				commandHandle, 
				submitterDid,
				targetDid,
				buildGetNymRequestCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildSchemaRequest(
			String submitterDid,
			String data) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_schema_request(
				commandHandle, 
				submitterDid,
				data,
				buildSchemaRequestCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildGetSchemaRequest(
			String submitterDid,
			String data) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_schema_request(
				commandHandle, 
				submitterDid,
				data,
				buildGetSchemaRequestCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildClaimDefTxn(
			String submitterDid,
			String xref,
			String data) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_claim_def_txn(
				commandHandle, 
				submitterDid,
				xref,
				data,
				buildClaimDefTxnCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildGetClaimDefTxn(
			String submitterDid,
			String xref) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
		int commandHandle = addFuture(future);

		int result = LibIndy.api.indy_build_get_claim_def_txn(
				commandHandle, 
				submitterDid,
				xref,
				buildGetClaimDefTxnCb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildNodeRequest(
			String submitterDid,
			String targetDid,
			String data) throws IndyException {

		CompletableFuture<String> future = new CompletableFuture<String> ();
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
}
