package org.hyperledger.indy.sdk.ledger;

import java.util.concurrent.CompletableFuture;

import org.hyperledger.indy.sdk.IndyException;
import org.hyperledger.indy.sdk.IndyJava;
import org.hyperledger.indy.sdk.LibIndy;
import org.hyperledger.indy.sdk.ledger.LedgerResults.BuildAttribRequestResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.BuildClaimDefTxnResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.BuildGetAttribRequestResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.BuildGetClaimDefTxnResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.BuildGetDdoRequestResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.BuildGetNymRequestResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.BuildGetSchemaRequestResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.BuildNodeRequestResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.BuildNymRequestResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.BuildSchemaRequestResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.SignAndSubmitRequestResult;
import org.hyperledger.indy.sdk.ledger.LedgerResults.SubmitRequestResult;
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

	public static CompletableFuture<SignAndSubmitRequestResult> signAndSubmitRequest(
			Pool pool,
			Wallet wallet,
			String submitterDid,
			String requestJson) throws IndyException {

		final CompletableFuture<SignAndSubmitRequestResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_result_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				SignAndSubmitRequestResult result = new SignAndSubmitRequestResult(request_result_json);
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

	public static CompletableFuture<SubmitRequestResult> submitRequest(
			Pool pool,
			String requestJson) throws IndyException {

		final CompletableFuture<SubmitRequestResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_result_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				SubmitRequestResult result = new SubmitRequestResult(request_result_json);
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

	public static CompletableFuture<BuildGetDdoRequestResult> buildGetDdoRequest(
			String submitterDid,
			String targetDid,
			String requestJson) throws IndyException {

		final CompletableFuture<BuildGetDdoRequestResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				BuildGetDdoRequestResult result = new BuildGetDdoRequestResult(request_json);
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

	public static CompletableFuture<BuildNymRequestResult> buildNymRequest(
			String submitterDid,
			String targetDid,
			String verkey,
			String alias,
			String role) throws IndyException {

		final CompletableFuture<BuildNymRequestResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				BuildNymRequestResult result = new BuildNymRequestResult(request_json);
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

	public static CompletableFuture<BuildAttribRequestResult> buildAttribRequest(
			String submitterDid,
			String targetDid,
			String hash,
			String raw,
			String enc) throws IndyException {

		final CompletableFuture<BuildAttribRequestResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				BuildAttribRequestResult result = new BuildAttribRequestResult(request_json);
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

	public static CompletableFuture<BuildGetAttribRequestResult> buildGetAttribRequest(
			String submitterDid,
			String targetDid,
			String data) throws IndyException {

		final CompletableFuture<BuildGetAttribRequestResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				BuildGetAttribRequestResult result = new BuildGetAttribRequestResult(request_json);
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

	public static CompletableFuture<BuildGetNymRequestResult> buildGetNymRequest(
			String submitterDid,
			String targetDid) throws IndyException {

		final CompletableFuture<BuildGetNymRequestResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				BuildGetNymRequestResult result = new BuildGetNymRequestResult(request_json);
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

	public static CompletableFuture<BuildSchemaRequestResult> buildSchemaRequest(
			String submitterDid,
			String data) throws IndyException {

		final CompletableFuture<BuildSchemaRequestResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				BuildSchemaRequestResult result = new BuildSchemaRequestResult(request_json);
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

	public static CompletableFuture<BuildGetSchemaRequestResult> buildGetSchemaRequest(
			String submitterDid,
			String data) throws IndyException {

		final CompletableFuture<BuildGetSchemaRequestResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				BuildGetSchemaRequestResult result = new BuildGetSchemaRequestResult(request_json);
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

	public static CompletableFuture<BuildClaimDefTxnResult> buildClaimDefTxn(
			String submitterDid,
			String xref,
			String data) throws IndyException {

		final CompletableFuture<BuildClaimDefTxnResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				BuildClaimDefTxnResult result = new BuildClaimDefTxnResult(request_json);
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

	public static CompletableFuture<BuildGetClaimDefTxnResult> buildGetClaimDefTxn(
			String submitterDid,
			String xref) throws IndyException {

		final CompletableFuture<BuildGetClaimDefTxnResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				BuildGetClaimDefTxnResult result = new BuildGetClaimDefTxnResult(request_json);
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

	public static CompletableFuture<BuildNodeRequestResult> buildNodeRequest(
			String submitterDid,
			String targetDid,
			String data) throws IndyException {

		final CompletableFuture<BuildNodeRequestResult> future = new CompletableFuture<> ();

		Callback cb = new Callback() {

			@SuppressWarnings("unused")
			public void callback(int xcommand_handle, int err, String request_json) {

				if (! checkCallback(future, xcommand_handle, err)) return;

				BuildNodeRequestResult result = new BuildNodeRequestResult(request_json);
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
