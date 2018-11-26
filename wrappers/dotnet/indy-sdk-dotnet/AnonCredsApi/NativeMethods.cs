using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.AnonCredsApi
{
    internal static class NativeMethods
    {
        /// <summary>
        /// Create credential schema entity that describes credential attributes list and allows credentials
        /// interoperability.
        /// to Indy distributed ledger.
        ///
        /// Schema is public and intended to be shared with all anoncreds workflow actors usually by publishing SCHEMA transaction
        /// to Indy distributed ledger.
        ///
        /// It is IMPORTANT for current version POST Schema in Ledger and after that GET it from Ledger
        /// with correct seq_no to save compatibility with Ledger.
        /// After that can call indy_issuer_create_and_store_credential_def to build corresponding Credential Definition.
        ///
        /// </summary>
        /// <returns>The issuer create schema.</returns>
        /// <param name="command_handle">Command handle to map callback to user context</param>
        /// <param name="issuer_did">DID of schema issuer.</param>
        /// <param name="name">Name of the schema</param>
        /// <param name="version">Version of the schema</param>
        /// <param name="attrs">A list of schema attribute descriptions.</param>
        /// <param name="cb">Callback that takes command result as parameter</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_schema(int command_handle, string issuer_did, string name, string version, string attrs, IssuerCreateSchemaCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_issuer_create_schema function.
        /// </summary>
        internal delegate void IssuerCreateSchemaCompletedDelegate(int xcommand_handle, int err, string schema_id, string schema_json);

        /// <summary>
        /// Create credential definition entity that encapsulates credentials issuer DID, credential schema, secrets used for signing credentials
        /// and secrets used for credentials revocation.
        ///
        /// Credential definition entity contains private and public parts. Private part will be stored in the wallet. Public part
        /// will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing CRED_DEF transaction
        /// to Indy distributed ledger.
        ///
        /// It is IMPORTANT for current version GET Schema from Ledger with correct seq_no to save compatibility with Ledger.
        ///
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="issuer_did">a DID of the issuer signing claim_def transaction to the Ledger</param>
        /// <param name="schema_json">schema as a json</param>
        /// <param name="tag">Allows to distinct between credential definitions for the same issuer and schema</param>
        /// <param name="signature_type">Signature type (optional). Currently only 'CL' is supported.</param>
        /// <param name="config_json">type-specific configuration of credential definition as json:
        /// - 'CL':
        ///   - support_revocation: whether to request non-revocation credential (optional, default false)</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_and_store_credential_def(int command_handle, int wallet_handle, string issuer_did, string schema_json, string tag, string signature_type, string config_json, IssuerCreateAndStoreCredentialDefCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_issuer_create_and_store_claim_def function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="cred_def_id"></param>
        /// <param name="cred_def_json">claim definition json containing information about signature type, schema and issuer's public key.</param>
        internal delegate void IssuerCreateAndStoreCredentialDefCompletedDelegate(int xcommand_handle, int err, string cred_def_id, string cred_def_json);

        /// <summary>
        /// Indies the issuer create and store revoc reg.
        /// </summary>
        /// <returns>The issuer create and store revoc reg.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="wallet_handle">Wallet handle.</param>
        /// <param name="issuer_did">Issuer did.</param>
        /// <param name="revoc_def_type">Type.</param>
        /// <param name="tag">Tag.</param>
        /// <param name="cred_def_id">Cred def identifier.</param>
        /// <param name="config_json">Config json.</param>
        /// <param name="tails_writer_handle">Tails writer handle.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_and_store_revoc_reg(int command_handle, int wallet_handle, string issuer_did, string revoc_def_type, string tag, string cred_def_id, string config_json, int tails_writer_handle, IssuerCreateAndStoreRevocRegCompletedDelegate cb);

        internal delegate void IssuerCreateAndStoreRevocRegCompletedDelegate(int xcommand_handle, int err, string revoc_reg_id, string revoc_reg_def_json, string revoc_reg_entry_json);

        /// <summary>
        /// Indies the issuer create credential offer.
        /// </summary>
        /// <returns>The issuer create credential offer.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="wallet_handle">Wallet handle.</param>
        /// <param name="cred_def_id">Cred def identifier.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_credential_offer(int command_handle, int wallet_handle, string cred_def_id, IssuerCreateCredentialOfferCompletedDelegate cb);

        internal delegate void IssuerCreateCredentialOfferCompletedDelegate(int xcommand_handle, int err, string cred_offer_json);

        /// <summary>
        /// Indies the issuer create credential.
        /// </summary>
        /// <returns>The issuer create credential.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="wallet_handle">Wallet handle.</param>
        /// <param name="cred_offer_json">Cred offer json.</param>
        /// <param name="cred_req_json">Cred req json.</param>
        /// <param name="cred_values_json">Cred values json.</param>
        /// <param name="rev_reg_id">Rev reg identifier.</param>
        /// <param name="blob_storage_reader_handle">BLOB storage reader handle.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_credential(int command_handle, int wallet_handle, string cred_offer_json, string cred_req_json, string cred_values_json, string rev_reg_id, int blob_storage_reader_handle, IssuerCreateCredentialCompletedDelegate cb);

        internal delegate void IssuerCreateCredentialCompletedDelegate(int xcommand_handle, int err, string cred_json, string cred_revoc_id, string revoc_reg_delta_json);

        /// <summary>
        /// Revoke a credential identified by a cred_revoc_id (returned by indy_issuer_create_credential).
        /// </summary>
        /// <returns>The issuer revoke credential.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="wallet_handle">Wallet handle.</param>
        /// <param name="blob_storage_reader_cfg_handle">BLOB storage reader cfg handle.</param>
        /// <param name="rev_reg_id">Rev reg identifier.</param>
        /// <param name="cred_revoc_id">Cred revoc identifier.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_revoke_credential(int command_handle, int wallet_handle, int blob_storage_reader_cfg_handle, string rev_reg_id, string cred_revoc_id, IssuerRevokeCredentialCompletedDelegate cb);

        internal delegate void IssuerRevokeCredentialCompletedDelegate(int xcommand_handle, int err, string revoc_reg_delta_json);

        /// <summary>
        /// Indies the issuer merge revocation registry deltas.
        /// </summary>
        /// <returns>The issuer merge revocation registry deltas.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="rev_reg_delta_json">Rev reg delta json.</param>
        /// <param name="other_rev_reg_delta_json">Other rev reg delta json.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_merge_revocation_registry_deltas(int command_handle, string rev_reg_delta_json, string other_rev_reg_delta_json, IssuerMergeRevocationRegistryDeltasCompletedDelegate cb);

        internal delegate void IssuerMergeRevocationRegistryDeltasCompletedDelegate(int xcommand_handle, int err, string merged_rev_reg_delta);

        /// <summary>
        /// Creates a master secret with a given name and stores it in the wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="master_secret_id">a new master secret name</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_master_secret(int command_handle, int wallet_handle, string master_secret_id, ProverCreateMasterSecretCompletedDelegate cb);

        internal delegate void ProverCreateMasterSecretCompletedDelegate(int xcommand_handle, int err, string out_master_secret_id);

        /// <summary>
        /// Creates a clam request json for the given claim offer and stores it in a secure wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="prover_did">a DID of the prover</param>
        /// <param name="cred_offer_json">claim offer as a json containing information about the issuer and a claim</param>
        /// <param name="cred_def_json">claim definition json associated with issuer_did and schema_seq_no in the claim_offer</param>
        /// <param name="master_secret_id">the name of the master secret stored in the wallet</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_credential_req(int command_handle, int wallet_handle, string prover_did, string cred_offer_json, string cred_def_json, string master_secret_id, ProverCreateCredentialReqCompletedDelegate cb);

        internal delegate void ProverCreateCredentialReqCompletedDelegate(int xcommand_handle, int err, string cred_req_json, string cred_req_metadata);

        /// <summary>
        /// Indies the prover store credential.
        /// </summary>
        /// <returns>The prover store credential.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="wallet_handle">Wallet handle.</param>
        /// <param name="cred_id">Cred identifier.</param>
        /// <param name="cred_req_metadata_json">Cred req metadata json.</param>
        /// <param name="cred_json">Cred json.</param>
        /// <param name="cred_def_json">Cred def json.</param>
        /// <param name="rev_reg_def_json">Rev reg def json.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_store_credential(int command_handle, int wallet_handle, string cred_id, string cred_req_metadata_json, string cred_json, string cred_def_json, string rev_reg_def_json, ProverStoreCredentialCompletedDelegate cb);

        internal delegate void ProverStoreCredentialCompletedDelegate(int xcommand_handle, int err, string out_cred_id);

        /// <summary>
        /// Gets human readable credential by the given id.
        /// </summary>
        /// <returns>The prover get credential.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="wallet_handle">Wallet handle.</param>
        /// <param name="cred_id">Cred identifier.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_credential(int command_handle, int wallet_handle, string cred_id, ProverGetCredentialCompletedDelegate cb);

        internal delegate void ProverGetCredentialCompletedDelegate(int xcommand_handle, int err, string credential_json);

        /// <summary>
        /// Gets human readable claims according to the filter.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="filter_json">filter for claims</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_credentials(int command_handle, int wallet_handle, string filter_json, ProverGetCredentialsCompletedDelegate cb);

        internal delegate void ProverGetCredentialsCompletedDelegate(int xcommand_handle, int err, string matched_credentials_json);

        /// <summary>
        /// Search for credentials stored in wallet.
        /// Credentials can be filtered by tags created during saving of credential.
        /// </summary>
        /// <returns>The prover search credentials.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="wallet_handle">Wallet handle.</param>
        /// <param name="query_json">Query json.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_search_credentials(int command_handle, int wallet_handle, string query_json, ProverSearchCredentialsCompletedDelegate cb);

        internal delegate void ProverSearchCredentialsCompletedDelegate(int xcommand_handle, int err, int search_handle, int total_count);

        /// <summary>
        /// Fetch next credentials for search.
        /// </summary>
        /// <returns>The prover fetch credentials.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="search_handle">Search handle.</param>
        /// <param name="count">Count.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_fetch_credentials(int command_handle, int search_handle, int count, ProverFetchCredentialsCompletedDelegate cb);

        internal delegate void ProverFetchCredentialsCompletedDelegate(int xcommand_handle, int err, string credentials_json);

        /// <summary>
        /// Close credentials search (make search handle invalid)
        /// </summary>
        /// <returns>The prover close credentials search.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="search_handle">Search handle.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_close_credentials_search(int command_handle, int search_handle, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Gets human readable claims matching the given proof request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="proof_request_json">proof request json</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_credentials_for_proof_req(int command_handle, int wallet_handle, string proof_request_json, ProverGetCredentialsForProofCompletedDelegate cb);

        internal delegate void ProverGetCredentialsForProofCompletedDelegate(int xcommand_handle, int err, string credentials_json);

        /// <summary>
        /// Search for credentials matching the given proof request.
        /// </summary>
        /// <returns>The prover search credentials for proof req.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="wallet_handle">Wallet handle.</param>
        /// <param name="proof_request_json">Proof request json.</param>
        /// <param name="extra_query_json">Extra query json.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_search_credentials_for_proof_req(int command_handle, int wallet_handle, string proof_request_json, string extra_query_json, ProverSearchCredentialsForProofReqCompletedDelegate cb);

        internal delegate void ProverSearchCredentialsForProofReqCompletedDelegate(int xcommand_handle, int err, int search_handle);


        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_fetch_credentials_for_proof_req(int command_handle, int search_handle, string item_referent, int count, ProverFetchCredentialsForProofReqCompletedDelegate cb);

        internal delegate void ProverFetchCredentialsForProofReqCompletedDelegate(int xcommand_handle, int err, string credentials_json);

        /// <summary>
        /// Close credentials search (make search handle invalid)
        /// </summary>
        /// <returns>The prover close credentials search.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="search_handle">Search handle.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_close_credentials_search_for_proof_req(int command_handle, int search_handle, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Indies the prover create proof.
        /// </summary>
        /// <returns>The prover create proof.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="wallet_handle">Wallet handle.</param>
        /// <param name="proof_req_json">Proof req json.</param>
        /// <param name="requested_credentials_json">Requested credentials json.</param>
        /// <param name="master_secret_id">Master secret identifier.</param>
        /// <param name="schemas_json">Schemas json.</param>
        /// <param name="credential_defs_json">Credential defs json.</param>
        /// <param name="rev_states_json">Rev states json.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_proof(int command_handle, int wallet_handle, string proof_req_json, string requested_credentials_json, string master_secret_id, string schemas_json, string credential_defs_json, string rev_states_json, ProverCreateProofCompletedDelegate cb);

        internal delegate void ProverCreateProofCompletedDelegate(int xcommand_handle, int err, string proof_json);

        /// <summary>
        /// Indies the verifier verify proof.
        /// </summary>
        /// <returns>The verifier verify proof.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="proof_request_json">Proof request json.</param>
        /// <param name="proof_json">Proof json.</param>
        /// <param name="schemas_json">Schemas json.</param>
        /// <param name="credential_defs_json">Credential defs json.</param>
        /// <param name="rev_reg_defs_json">Rev reg defs json.</param>
        /// <param name="rev_regs_json">Rev regs json.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_verifier_verify_proof(int command_handle, string proof_request_json, string proof_json, string schemas_json, string credential_defs_json, string rev_reg_defs_json, string rev_regs_json, VerifierVerifyProofCompletedDelegate cb);

        internal delegate void VerifierVerifyProofCompletedDelegate(int xcommand_handle, int err, bool valid);

        /// <summary>
        /// Indies the state of the create revocation.
        /// </summary>
        /// <returns>The create revocation state.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="blob_storage_reader_handle">BLOB storage reader handle.</param>
        /// <param name="rev_reg_def_json">Rev reg def json.</param>
        /// <param name="rev_reg_delta_json">Rev reg delta json.</param>
        /// <param name="timestamp">Timestamp.</param>
        /// <param name="cred_rev_id">Cred rev identifier.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_revocation_state(int command_handle, int blob_storage_reader_handle, string rev_reg_def_json, string rev_reg_delta_json, long timestamp, string cred_rev_id, CreateRevocationStateCompletedDelegate cb);

        internal delegate void CreateRevocationStateCompletedDelegate(int xcommand_handle, int err, string rev_state_json);

        /// <summary>
        /// Indies the state of the update revocation.
        /// </summary>
        /// <returns>The update revocation state.</returns>
        /// <param name="command_handle">Command handle.</param>
        /// <param name="blob_storage_reader_handle">BLOB storage reader handle.</param>
        /// <param name="rev_state_json">Rev state json.</param>
        /// <param name="rev_reg_def_json">Rev reg def json.</param>
        /// <param name="rev_reg_delta_json">Rev reg delta json.</param>
        /// <param name="timestamp">Timestamp.</param>
        /// <param name="cred_rev_id">Cred rev identifier.</param>
        /// <param name="cb">Cb.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_update_revocation_state(int command_handle, int blob_storage_reader_handle, string rev_state_json, string rev_reg_def_json, string rev_reg_delta_json, long timestamp, string cred_rev_id, UpdateRevocationStateCompletedDelegate cb);

        internal delegate void UpdateRevocationStateCompletedDelegate(int xcommand_handle, int err, string updated_rev_state_json);
    }
}
