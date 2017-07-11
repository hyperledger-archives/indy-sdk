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
	 * STATIC METHODS
	 */

	public static CompletableFuture<String> signAndSubmitRequest(
			Pool pool,
			Wallet wallet,
			String submitterDid,
			String requestJson) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_result_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_result_json;
				future.complete(result);
			}
		};

		int poolHandle = pool.getPoolHandle();
		int walletHandle = wallet.getWalletHandle();

		int result = LibIndy.api.indy_sign_and_submit_request(
				FIXED_COMMAND_HANDLE, 
				poolHandle,
				walletHandle, 
				submitterDid,
				requestJson,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> submitRequest(
			Pool pool,
			String requestJson) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_result_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_result_json;
				future.complete(result);
			}
		};

		int poolHandle = pool.getPoolHandle();

		int result = LibIndy.api.indy_submit_request(
				FIXED_COMMAND_HANDLE, 
				poolHandle,
				requestJson,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildGetDdoRequest(
			String submitterDid,
			String targetDid,
			String requestJson) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_json;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_build_get_ddo_request(
				FIXED_COMMAND_HANDLE, 
				submitterDid,
				targetDid,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildNymRequest(
			String submitterDid,
			String targetDid,
			String verkey,
			String alias,
			String role) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_json;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_build_nym_request(
				FIXED_COMMAND_HANDLE, 
				submitterDid,
				targetDid,
				verkey,
				alias,
				role,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildAttribRequest(
			String submitterDid,
			String targetDid,
			String hash,
			String raw,
			String enc) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_json;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_build_attrib_request(
				FIXED_COMMAND_HANDLE, 
				submitterDid,
				targetDid,
				hash,
				raw,
				enc,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildGetAttribRequest(
			String submitterDid,
			String targetDid,
			String data) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_json;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_build_get_attrib_request(
				FIXED_COMMAND_HANDLE, 
				submitterDid,
				targetDid,
				data,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildGetNymRequest(
			String submitterDid,
			String targetDid) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_json;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_build_get_nym_request(
				FIXED_COMMAND_HANDLE, 
				submitterDid,
				targetDid,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildSchemaRequest(
			String submitterDid,
			String data) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_json;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_build_schema_request(
				FIXED_COMMAND_HANDLE, 
				submitterDid,
				data,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildGetSchemaRequest(
			String submitterDid,
			String data) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_json;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_build_get_schema_request(
				FIXED_COMMAND_HANDLE, 
				submitterDid,
				data,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildClaimDefTxn(
			String submitterDid,
			String xref,
			String data) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_json;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_build_claim_def_txn(
				FIXED_COMMAND_HANDLE, 
				submitterDid,
				xref,
				data,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildGetClaimDefTxn(
			String submitterDid,
			String xref) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_json;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_build_get_claim_def_txn(
				FIXED_COMMAND_HANDLE, 
				submitterDid,
				xref,
				cb);

		checkResult(result);

		return future;
	}

	public static CompletableFuture<String> buildNodeRequest(
			String submitterDid,
			String targetDid,
			String data) throws IndyException {

		final CompletableFuture<String> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				String result = request_json;
				future.complete(result);
			}
		};

		int result = LibIndy.api.indy_build_node_request(
				FIXED_COMMAND_HANDLE, 
				submitterDid,
				targetDid,
				data,
				cb);

		checkResult(result);

		return future;
	}
}
