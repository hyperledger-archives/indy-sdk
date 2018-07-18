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
		public int indy_set_protocol_version(int command_handle, int protocol_version, Callback cb);

		// wallet.rs

		public int indy_register_wallet_storage(int command_handle, String type, Callback create, Callback open, Callback close,
		                                        Callback delete, Callback add_record, Callback update_record_value, Callback update_record_tags,
		                                        Callback add_record_tags, Callback delete_record_tags, Callback delete_record,
		                                        Callback get_record, Callback get_record_id, Callback get_record_type, Callback get_record_value,
		                                        Callback get_record_tags, Callback free_record, Callback search_records, Callback search_all_records,
		                                        Callback get_search_total_count, Callback fetch_search_next_record, Callback free_search, Callback cb);
		public int indy_create_wallet(int command_handle, String config, String credentials, Callback cb);
		public int indy_open_wallet(int command_handle, String config, String credentials, Callback cb);
		public int indy_close_wallet(int command_handle, int handle, Callback cb);
		public int indy_delete_wallet(int command_handle, String config, String credentials, Callback cb);
		public int indy_export_wallet(int command_handle, int handle, String exportConfigJson, Callback cb);
		public int indy_import_wallet(int command_handle, String config, String credentials, String importConfigJson, Callback cb);

		// ledger.rs

		public int indy_sign_and_submit_request(int command_handle, int pool_handle, int wallet_handle, String submitter_did, String request_json, Callback cb);
		public int indy_submit_request(int command_handle, int pool_handle, String request_json, Callback cb);
		public int indy_sign_request(int command_handle, int wallet_handle, String submitter_did, String request_json, Callback cb);
		public int indy_multi_sign_request(int command_handle, int wallet_handle, String submitter_did, String request_json, Callback cb);
		public int indy_build_get_ddo_request(int command_handle, String submitter_did, String target_did, Callback cb);
		public int indy_build_nym_request(int command_handle, String submitter_did, String target_did, String verkey, String alias, String role, Callback cb);
		public int indy_build_attrib_request(int command_handle, String submitter_did, String target_did, String hash, String raw, String enc, Callback cb);
		public int indy_build_get_attrib_request(int command_handle, String submitter_did, String target_did, String raw, String hash, String enc, Callback cb);
		public int indy_build_get_nym_request(int command_handle, String submitter_did, String target_did, Callback cb);
		public int indy_build_schema_request(int command_handle, String submitter_did, String data, Callback cb);
		public int indy_build_get_schema_request(int command_handle, String submitter_did, String id, Callback cb);
		public int indy_parse_get_schema_response(int command_handle, String get_schema_response, Callback cb);
		public int indy_build_cred_def_request(int command_handle, String submitter_did, String data, Callback cb);
		public int indy_build_get_cred_def_request(int command_handle, String submitter_did, String id, Callback cb);
		public int indy_parse_get_cred_def_response(int command_handle, String get_cred_def_response, Callback cb);
		public int indy_build_node_request(int command_handle, String submitter_did, String target_did, String data, Callback cb);
		public int indy_build_get_validator_info_request(int command_handle, String submitter_did, Callback cb);
		public int indy_build_get_txn_request(int command_handle, String submitter_did, String ledger_type, int data, Callback cb);
		public int indy_build_pool_config_request(int command_handle, String submitter_did, boolean writes, boolean force, Callback cb);
		public int indy_build_pool_restart_request(int command_handle, String submitter_did, String action, String datetime, Callback cb);
		public int indy_build_pool_upgrade_request(int command_handle, String submitter_did, String name, String version, String action, String sha256, int timeout, String schedule, String justification, boolean reinstall, boolean force, Callback cb);
		public int indy_build_revoc_reg_def_request(int command_handle, String submitter_did, String data, Callback cb);
		public int indy_build_get_revoc_reg_def_request(int command_handle, String submitter_did, String id, Callback cb);
		public int indy_parse_get_revoc_reg_def_response(int command_handle, String get_revoc_ref_def_response, Callback cb);
		public int indy_build_revoc_reg_entry_request(int command_handle, String submitter_did, String revoc_reg_def_id, String rev_def_type, String value, Callback cb);
		public int indy_build_get_revoc_reg_request(int command_handle, String submitter_did, String revoc_reg_def_id, long timestamp, Callback cb);
		public int indy_parse_get_revoc_reg_response(int command_handle, String get_revoc_reg_response, Callback cb);
		public int indy_build_get_revoc_reg_delta_request(int command_handle, String submitter_did, String revoc_reg_def_id, long from, long to, Callback cb);
		public int indy_parse_get_revoc_reg_delta_response(int command_handle, String get_revoc_reg_delta_response, Callback cb);

		// did.rs

		public int indy_create_and_store_my_did(int command_handle, int wallet_handle, String did_json, Callback cb);
		public int indy_replace_keys_start(int command_handle, int wallet_handle, String did, String identity_json, Callback cb);
		public int indy_replace_keys_apply(int command_handle, int wallet_handle, String did, Callback cb);
		public int indy_store_their_did(int command_handle, int wallet_handle, String identity_json, Callback cb);
		public int indy_key_for_did(int command_handle, int pool_handle, int wallet_handle, String did, Callback cb);
        public int indy_key_for_local_did(int command_handle, int wallet_handle, String did, Callback cb);
		public int indy_set_endpoint_for_did(int command_handle, int wallet_handle, String did, String address, String transportKey, Callback cb);
		public int indy_get_endpoint_for_did(int command_handle, int wallet_handle, int pool_handle, String did, Callback cb);
		public int indy_set_did_metadata(int command_handle, int wallet_handle, String did, String metadata, Callback cb);
		public int indy_get_did_metadata(int command_handle, int wallet_handle, String did, Callback cb);
		public int indy_get_my_did_with_meta(int command_handle, int wallet_handle, String did, Callback cb);
		public int indy_list_my_dids_with_meta(int command_handle, int wallet_handle, Callback cb);
		public int indy_abbreviate_verkey(int command_handle, String did, String full_verkey, Callback cb);

		// crypto.rs

		public int indy_create_key(int command_handle, int wallet_handle, String key_json, Callback cb);
		public int indy_set_key_metadata(int command_handle, int wallet_handle, String verkey, String metadata, Callback cb);
		public int indy_get_key_metadata(int command_handle, int wallet_handle, String verkey, Callback cb);
		public int indy_crypto_sign(int command_handle, int wallet_handle, String my_vk, byte[] message_raw, int message_len, Callback cb);
		public int indy_crypto_verify(int command_handle, String their_vk, byte[] message_raw, int message_len, byte[] signature_raw, int signature_len, Callback cb);
		public int indy_crypto_auth_crypt(int command_handle, int wallet_handle, String my_vk, String their_vk, byte[] message_raw, int message_len, Callback cb);
		public int indy_crypto_auth_decrypt(int command_handle, int wallet_handle, String my_vk, byte[] encrypted_msg_raw, int encrypted_msg_len, Callback cb);
		public int indy_crypto_anon_crypt(int command_handle, String their_vk, byte[] message_raw, int message_len, Callback cb);
		public int indy_crypto_anon_decrypt(int command_handle, int wallet_handle, String my_vk, byte[] encrypted_msg_raw, int encrypted_msg_len, Callback cb);

		// anoncreds.rs

		public int indy_issuer_create_schema(int command_handle, String issuer_did, String name, String version, String attr_names, Callback cb);
		public int indy_issuer_create_and_store_credential_def(int command_handle, int wallet_handle, String issuer_did, String schema_json, String tag, String signature_type, String config_json, Callback cb);
		public int indy_issuer_create_and_store_revoc_reg(int command_handle, int wallet_handle, String issuer_did, String revoc_def_type, String tag, String cred_def_id, String config_json, int blob_storage_writer_handle, Callback cb);
		public int indy_issuer_create_credential_offer(int command_handle, int wallet_handle, String cred_def_id, Callback cb);
		public int indy_issuer_create_credential(int command_handle, int wallet_handle, String cred_offer_json, String cred_req_json, String cred_values_json, String rev_reg_id, int blob_storage_reader_handle, Callback cb);
		public int indy_issuer_revoke_credential(int command_handle, int wallet_handle, int blob_storage_reader_handle, String rev_reg_id, String cred_revoc_id, Callback cb);
//		public int indy_issuer_recover_credential(int command_handle, int wallet_handle, int blob_storage_reader_handle, String rev_reg_id, String cred_revoc_id, Callback cb);
		public int indy_issuer_merge_revocation_registry_deltas(int command_handle, String rev_reg_delta_json, String other_rev_reg_delta_json, Callback cb);
		public int indy_prover_create_master_secret(int command_handle, int wallet_handle, String master_secret_id, Callback cb);
		public int indy_prover_create_credential_req(int command_handle, int wallet_handle, String prover_did, String cred_offer_json, String cred_def_json, String master_secret_id, Callback cb);
		public int indy_prover_store_credential(int command_handle, int wallet_handle, String cred_id, String cred_req_metadata_json, String cred_json, String cred_def_json, String rev_reg_def_json, Callback cb);
		public int indy_prover_get_credentials(int command_handle, int wallet_handle, String filter_json, Callback cb);
		public int indy_prover_get_credential(int command_handle, int wallet_handle, String cred_id, Callback cb);
		public int indy_prover_search_credentials(int command_handle, int wallet_handle, String query_json, Callback cb);
		public int indy_prover_fetch_credentials(int command_handle, int search_handle, int count, Callback cb);
		public int indy_prover_close_credentials_search(int command_handle, int search_handle, Callback cb);
		public int indy_prover_get_credentials_for_proof_req(int command_handle, int wallet_handle, String proof_request_json, Callback cb);
		public int indy_prover_search_credentials_for_proof_req(int command_handle, int wallet_handle, String proof_request_json, String extra_query_json, Callback cb);
		public int indy_prover_fetch_credentials_for_proof_req(int command_handle, int search_handle, String item_referent, int count, Callback cb);
		public int indy_prover_close_credentials_search_for_proof_req(int command_handle, int search_handle, Callback cb);
		public int indy_prover_create_proof(int command_handle, int wallet_handle, String proof_req_json, String requested_credentials_json, String master_secret_name, String schemas_json, String credential_defs_json, String rev_infos_json, Callback cb);
		public int indy_verifier_verify_proof(int command_handle, String proof_request_json, String proof_json, String schemas_json, String cred_defs_jsons, String rev_reg_defs_json, String revoc_regs_json, Callback cb);
		public int indy_create_revocation_state(int command_handle, int blob_storage_reader_handle, String rev_reg_def_json, String rev_reg_delta_json, long timestamp, String cred_rev_id, Callback cb);
		public int indy_update_revocation_state(int command_handle, int blob_storage_reader_handle, String rev_state_json, String rev_reg_def_json, String rev_reg_delta_json, long timestamp, String cred_rev_id, Callback cb);

		// pairwise.rs

		public int indy_is_pairwise_exists(int command_handle, int wallet_handle, String their_did, Callback cb);
		public int indy_create_pairwise(int command_handle, int wallet_handle, String their_did, String my_did, String metadata, Callback cb);
		public int indy_list_pairwise(int command_handle, int wallet_handle, Callback cb);
		public int indy_get_pairwise(int command_handle, int wallet_handle, String their_did, Callback cb);
		public int indy_set_pairwise_metadata(int command_handle, int wallet_handle, String their_did, String metadata, Callback cb);

		// blob_storage.rs
		public int indy_open_blob_storage_reader(int command_handle, String type, String config_json, Callback cb);
		public int indy_open_blob_storage_writer(int command_handle, String type, String config_json, Callback cb);

		// non_secrets.rs
		public int indy_add_wallet_record(int command_handle, int wallet_handle, String type, String id, String value, String tags_json, Callback cb);
		public int indy_update_wallet_record_value(int command_handle, int wallet_handle, String type, String id, String value, Callback cb);
		public int indy_update_wallet_record_tags(int command_handle, int wallet_handle, String type, String id, String tags_json, Callback cb);
		public int indy_add_wallet_record_tags(int command_handle, int wallet_handle, String type, String id, String tags_json, Callback cb);
		public int indy_delete_wallet_record_tags(int command_handle, int wallet_handle, String type, String id, String tag_names_json, Callback cb);
		public int indy_delete_wallet_record(int command_handle, int wallet_handle, String type, String id, Callback cb);
		public int indy_get_wallet_record(int command_handle, int wallet_handle, String type, String id, String options_json, Callback cb);
		public int indy_open_wallet_search(int command_handle, int wallet_handle, String type, String query_json, String options_json, Callback cb);
		public int indy_fetch_wallet_search_next_records(int command_handle, int wallet_handle, int wallet_search_handle, int count, Callback cb);
		public int indy_close_wallet_search(int command_handle, int wallet_search_handle, Callback cb);
		// payments.rs
		int indy_create_payment_address(int command_handle, int wallet_handle, String payment_method, String config, Callback cb);
		int indy_list_payment_addresses(int command_handle, int wallet_handle, Callback cb);
		int indy_add_request_fees(int command_handle, int wallet_handle, String submitter_did, String req_json, String inputs_json, String outputs_json, Callback cb);
		int indy_parse_response_with_fees(int command_handle, String payment_method, String resp_json, Callback cb);
		int indy_build_get_utxo_request(int command_handle, int wallet_handle, String submitter_did, String payment_method, Callback cb);
		int indy_parse_get_utxo_response(int command_handle, String payment_method, String resp_json, Callback cb);
		int indy_build_payment_req(int command_handle, int wallet_handle, String submitter_did, String inputs_json, String outputs_json, Callback cb);
		int indy_parse_payment_response(int command_handle, String payment_method, String resp_json, Callback cb);
		int indy_build_mint_req(int command_handle, int wallet_handle, String submitter_did, String outputs_json, Callback cb);
		int indy_build_set_txn_fees_req(int command_handle, int wallet_handle, String submitter_did, String payment_method, String fees_json, Callback cb);
		int indy_build_get_txn_fees_req(int command_handle, int wallet_handle, String submitter_did, String payment_method, Callback cb);
		int indy_parse_get_txn_fees_response(int command_handle, String payment_method, String resp_json, Callback cb);

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
	 * @param searchPath The path to the directory containing the C-Callable library file.
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
