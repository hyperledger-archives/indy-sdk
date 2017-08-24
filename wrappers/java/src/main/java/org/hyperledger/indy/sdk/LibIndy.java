package org.hyperledger.indy.sdk;

import java.io.File;

import com.sun.jna.Callback;
import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.NativeLibrary;

public abstract class LibIndy {

	public static final String LIBRARY_NAME = "indy";

	/*
	 * Native library interface
	 */

	/**
	 * JNA method signatures for calling SDK function.
	 */
	public interface API extends Library {

		// pool.rs

		public int indy_create_pool_ledger_config(int command_handle, String config_name, String config, Callback cb);
		public int indy_open_pool_ledger(int command_handle, String config_name, String config, Callback cb);
		public int indy_refresh_pool_ledger(int command_handle, int handle, Callback cb);
		public int indy_close_pool_ledger(int command_handle, int handle, Callback cb);
		public int indy_delete_pool_ledger_config(int command_handle, String config_name, Callback cb);

		// wallet.rs

		public int indy_register_wallet_type(int command_handle, String xtype, Callback create, Callback open, Callback set, Callback get, Callback get_not_expired, Callback list, Callback close, Callback delete, Callback free, Callback cb);
		public int indy_create_wallet(int command_handle, String pool_name, String name, String xtype, String config, String credentials, Callback cb);
		public int indy_open_wallet(int command_handle, String name, String runtime_config, String credentials, Callback cb);
		public int indy_close_wallet(int command_handle, int handle, Callback cb);
		public int indy_delete_wallet(int command_handle, String name, String credentials, Callback cb);

		// ledger.rs

		public int indy_sign_and_submit_request(int command_handle, int pool_handle, int wallet_handle, String submitter_did, String request_json, Callback cb);
		public int indy_submit_request(int command_handle, int pool_handle, String request_json, Callback cb);
		public int indy_sign_request(int command_handle, int wallet_handle, String submitter_did, String request_json, Callback cb);
		public int indy_build_get_ddo_request(int command_handle, String submitter_did, String target_did, Callback cb);
		public int indy_build_nym_request(int command_handle, String submitter_did, String target_did, String verkey, String alias, String role, Callback cb);
		public int indy_build_attrib_request(int command_handle, String submitter_did, String target_did, String hash, String raw, String enc, Callback cb);
		public int indy_build_get_attrib_request(int command_handle, String submitter_did, String target_did, String data, Callback cb);
		public int indy_build_get_nym_request(int command_handle, String submitter_did, String target_did, Callback cb);
		public int indy_build_schema_request(int command_handle, String submitter_did, String data, Callback cb);
		public int indy_build_get_schema_request(int command_handle, String submitter_did, String dest, String data, Callback cb);
		public int indy_build_claim_def_txn(int command_handle, String submitter_did, int xref, String signature_type, String data, Callback cb);
		public int indy_build_get_claim_def_txn(int command_handle, String submitter_did, int xref, String signature_type, String origin, Callback cb);
		public int indy_build_node_request(int command_handle, String submitter_did, String target_did, String data, Callback cb);
		public int indy_build_get_txn_request(int command_handle, String submitter_did, int data, Callback cb);

		// signus.rs

		public int indy_create_and_store_my_did(int command_handle, int wallet_handle, String did_json, Callback cb);
		public int indy_replace_keys(int command_handle, int wallet_handle, String did, String identity_json, Callback cb);
		public int indy_store_their_did(int command_handle, int wallet_handle, String identity_json, Callback cb);
		public int indy_sign(int command_handle, int wallet_handle, String did, byte[] message_raw, int message_len, Callback cb);
		public int indy_verify_signature(int command_handle, int wallet_handle, int pool_handle, String did, byte[] message_raw, int message_len, byte[] signature_raw, int signature_len, Callback cb);
		public int indy_encrypt(int command_handle, int wallet_handle, int pool_handle, String my_did, String did, byte[] message_raw, int message_len, Callback cb);
		public int indy_decrypt(int command_handle, int wallet_handle, String myDid, String did, byte[] encrypted_msg_raw, int encrypted_msg_len, byte[] nonce_raw, int nonce_len, Callback cb);

		// anoncreds.rs

		public int indy_issuer_create_and_store_claim_def(int command_handle, int wallet_handle, String issuer_did, String schema_json, String signature_type, boolean create_non_revoc, Callback cb);
		public int indy_issuer_create_and_store_revoc_reg(int command_handle, int wallet_handle, String issuer_did, int schema_seq_no, int max_claim_num, Callback cb);
		public int indy_issuer_create_claim(int command_handle, int wallet_handle, String claim_req_json, String claim_json, int user_revoc_index, Callback cb);
		public int indy_issuer_revoke_claim(int command_handle, int wallet_handle, String issuer_did, int schema_seq_no, int user_revoc_index, Callback cb);
		public int indy_prover_store_claim_offer(int command_handle, int wallet_handle, String claim_offer_json, Callback cb);
		public int indy_prover_get_claim_offers(int command_handle, int wallet_handle, String filter_json, Callback cb);
		public int indy_prover_create_master_secret(int command_handle, int wallet_handle, String master_secret_name, Callback cb);
		public int indy_prover_create_and_store_claim_req(int command_handle, int wallet_handle, String prover_did, String claim_offer_json, String claim_def_json, String master_secret_name, Callback cb);
		public int indy_prover_store_claim(int command_handle, int wallet_handle, String claims_json, Callback cb);
		public int indy_prover_get_claims(int command_handle, int wallet_handle, String filter_json, Callback cb);
		public int indy_prover_get_claims_for_proof_req(int command_handle, int wallet_handle, String proof_request_json, Callback cb);
		public int indy_prover_create_proof(int command_handle, int wallet_handle, String proof_req_json, String requested_claims_json, String schemas_json, String master_secret_name, String claim_defs_json, String revoc_regs_json, Callback cb);
		public int indy_verifier_verify_proof(int command_handle, String proof_request_json, String proof_json, String schemas_json, String claim_defs_jsons, String revoc_regs_json, Callback cb);

		// agent.rs

		public int indy_agent_connect(int command_handle, int pool_handle, int wallet_handle, String sender_did, String receiver_did, Callback connection_cb, Callback message_cb);
		public int indy_agent_listen(int command_handle, String endpoint, Callback listener_cb, Callback connection_cb, Callback message_cb);
		public int indy_agent_add_identity(int command_handle, int listener_handle, int pool_handle, int wallet_handle, String did, Callback add_identity_cb);
		public int indy_agent_remove_identity(int command_handle, int listener_handle, int wallet_handle, String did, Callback rm_identity_cb);
		public int indy_agent_send(int command_handle, int connection_handle, String message, Callback cb);
		public int indy_agent_close_connection(int command_handle, int connection_handle, Callback cb);
		public int indy_agent_close_listener(int command_handle, int listener_handle, Callback cb);
	}

	/*
	 * Initialization
	 */

	public static API api = null;

	static {

		try {

			init();
		} catch (UnsatisfiedLinkError ex) {

			// Library could not be found in standard OS locations.
			// Call init(File file) explicitly with absolute library path.
		}
	}

	/**
	 * Initializes the API with the path to the C-Callable library.
	 * 
	 * @param path The path to the directory containing the C-Callable library file.
	 */
	public static void init(String searchPath) {

		NativeLibrary.addSearchPath(LIBRARY_NAME, searchPath);
		api = Native.loadLibrary(LIBRARY_NAME, API.class);
	}

	/**
	 * Initializes the API with the path to the C-Callable library.
	 * Warning: This is not platform-independent.
	 *
	 * @param file The absolute path to the C-Callable library file.
	 */
	public static void init(File file) {

		api = Native.loadLibrary(file.getAbsolutePath(), API.class);
	}

	/**
	 * Initializes the API with the default library.
	 */
	public static void init() {

		api = Native.loadLibrary(LIBRARY_NAME, API.class);
	}

	/**
	 * Indicates whether or not the API has been initialized.
	 * 
	 * @return true if the API is initialize, otherwise false.
	 */
	public static boolean isInitialized() {

		return api != null;
	}
}
