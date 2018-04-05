using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.AnonCredsApi
{
    internal static class NativeMethods
    {

        /// <summary>
        /// Create keys (both primary and revocation) for the given schema and signature type (currently only CL signature type is supported).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="issuer_did">a DID of the issuer signing claim_def transaction to the Ledger</param>
        /// <param name="schema_json">schema as a json</param>
        /// <param name="signature_type">signature type (optional). Currently only 'CL' is supported.</param>
        /// <param name="create_non_revoc">whether to request non-revocation claim.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_and_store_claim_def(int command_handle, IntPtr wallet_handle, string issuer_did, string schema_json, string signature_type, bool create_non_revoc, IssuerCreateAndStoreClaimDefCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_issuer_create_and_store_claim_def function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claim_def_json">claim definition json containing information about signature type, schema and issuer's public key.</param>
        internal delegate void IssuerCreateAndStoreClaimDefCompletedDelegate(int xcommand_handle, int err, string claim_def_json);

        /// <summary>
        /// Create a new revocation registry for the given claim definition.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="issuer_did">a DID of the issuer signing revoc_reg transaction to the Ledger</param>
        /// <param name="schema_seq_no">seq no of a schema transaction in Ledger</param>
        /// <param name="max_claim_num">maximum number of claims the new registry can process.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_and_store_revoc_reg(int command_handle, IntPtr wallet_handle, string issuer_did, int schema_seq_no, int max_claim_num, IssuerCreateAndStoreClaimRevocRegCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_issuer_create_and_store_revoc_reg function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="revoc_reg_json">Revoc registry json</param>
        internal delegate void IssuerCreateAndStoreClaimRevocRegCompletedDelegate(int xcommand_handle, int err, string revoc_reg_json);

        /// <summary>
        /// Create claim offer in Wallet
        /// </summary>
        /// <returns>The issuer create claim offer.</returns>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handler (created by open_wallet).</param>
        /// <param name="schema_json">Schema as json.</param>
        /// <param name="issuer_did">a DID of the issuer created Claim definition.</param>
        /// <param name="prover_did">a DID of the target user.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_claim_offer(int command_handle, IntPtr wallet_handle, string schema_json, string issuer_did, string prover_did, IssuerCreateClaimOfferCompletedDelegate cb);

        /// <summary>
        ///  Delegate for the function called back to by the indy_issuer_create_claim_offer function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claim_offer_json">Claimn offer json</param>
        internal delegate void IssuerCreateClaimOfferCompletedDelegate(int xcommand_handle, int err, string claim_offer_json);

        /// <summary>
        /// Signs a given claim for the given user by a given key (claim def).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="claim_req_json">a claim request with a blinded secret</param>
        /// <param name="claim_json">a claim containing attribute values for each of requested attribute names.</param>
        /// <param name="user_revoc_index">index of a new user in the revocation registry (optional, pass -1 if user_revoc_index is absentee; default one is used if not provided)</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_create_claim(int command_handle, IntPtr wallet_handle, string claim_req_json, string claim_json, int user_revoc_index, IssuerCreateClaimCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_issuer_create_and_store_revoc_reg function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="revoc_reg_update_json">Revocation registry update json with a newly issued claim</param>
        /// <param name="claim_json">Claim json containing issued claim, issuer_did, schema_seq_no, and revoc_reg_seq_no
        /// used for issuance</param>
        internal delegate void IssuerCreateClaimCompletedDelegate(int xcommand_handle, int err, string revoc_reg_update_json, string claim_json);

        /// <summary>
        /// Revokes a user identified by a revoc_id in a given revoc-registry.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="issuer_did">The DID of the issuer.</param>
        /// <param name="schema_json">The schema as a json</param>
        /// <param name="user_revoc_index">index of the user in the revocation registry</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_issuer_revoke_claim(int command_handle, IntPtr wallet_handle, string issuer_did, int schema_json, int user_revoc_index, IssuerRevokeClaimCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_issuer_revoke_claim function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="revoc_reg_update_json">Revocation registry update json with a revoked claim</param>
        internal delegate void IssuerRevokeClaimCompletedDelegate(int xcommand_handle, int err, string revoc_reg_update_json);

        /// <summary>
        /// Stores a claim offer from the given issuer in a secure storage.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="claim_offer_json">claim offer as a json containing information about the issuer and a claim</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_store_claim_offer(int command_handle, IntPtr wallet_handle, string claim_offer_json, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Gets all stored claim offers (see prover_store_claim_offer).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="filter_json">optional filter to get claim offers for specific Issuer, claim_def or schema only only</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_claim_offers(int command_handle, IntPtr wallet_handle, string filter_json, ProverGetClaimOffersCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_prover_get_claim_offers function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claim_offers_json">A json with a list of claim offers for the filter.</param>
        internal delegate void ProverGetClaimOffersCompletedDelegate(int xcommand_handle, int err, string claim_offers_json);

        /// <summary>
        /// Creates a master secret with a given name and stores it in the wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="master_secret_name">a new master secret name</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_master_secret(int command_handle, IntPtr wallet_handle, string master_secret_name, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Creates a clam request json for the given claim offer and stores it in a secure wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="prover_did">a DID of the prover</param>
        /// <param name="claim_offer_json">claim offer as a json containing information about the issuer and a claim</param>
        /// <param name="claim_def_json">claim definition json associated with issuer_did and schema_seq_no in the claim_offer</param>
        /// <param name="master_secret_name">the name of the master secret stored in the wallet</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_and_store_claim_req(int command_handle, IntPtr wallet_handle, string prover_did, string claim_offer_json, string claim_def_json, string master_secret_name, ProverCreateAndStoreClaimReqCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_prover_create_and_store_claim_req function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claim_req_json">Claim request json.</param>
        internal delegate void ProverCreateAndStoreClaimReqCompletedDelegate(int xcommand_handle, int err, string claim_req_json);

        /// <summary>
        /// Updates the claim by a master secret and stores in a secure wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="claims_json">claim json</param>
        /// <param name="rev_reg_json">revocation registry json</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_store_claim(int command_handle, IntPtr wallet_handle, string claims_json, string rev_reg_json, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Gets human readable claims according to the filter.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="filter_json">filter for claims</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_claims(int command_handle, IntPtr wallet_handle, string filter_json, ProverGetClaimsCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_prover_get_claims function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claims_json">claims json</param>
        internal delegate void ProverGetClaimsCompletedDelegate(int xcommand_handle, int err, string claims_json);

        /// <summary>
        /// Gets human readable claims matching the given proof request.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="proof_request_json">proof request json</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_get_claims_for_proof_req(int command_handle, IntPtr wallet_handle, string proof_request_json, ProverGetClaimsForProofCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_prover_get_claims_for_proof_req function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="claims_json">json with claims for the given pool request.</param>
        internal delegate void ProverGetClaimsForProofCompletedDelegate(int xcommand_handle, int err, string claims_json);

        /// <summary>
        /// Creates a proof according to the given proof request
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="proof_req_json">proof request json as come from the verifier</param>
        /// <param name="requested_claims_json">either a claim or self-attested attribute for each requested attribute</param>
        /// <param name="schemas_json">all schema jsons participating in the proof request</param>
        /// <param name="master_secret_name">master secret name</param>
        /// <param name="claim_defs_json">all claim definition jsons participating in the proof request</param>
        /// <param name="revoc_regs_json">all revocation registry jsons participating in the proof request</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prover_create_proof(int command_handle, IntPtr wallet_handle, string proof_req_json, string requested_claims_json, string schemas_json, string master_secret_name, string claim_defs_json, string revoc_regs_json, ProverCreateProofCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_prover_create_proof function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="proof_json">Proof json.</param>
        internal delegate void ProverCreateProofCompletedDelegate(int xcommand_handle, int err, string proof_json);

        /// <summary>
        /// Verifies a proof (of multiple claim).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="proof_request_json">initial proof request as sent by the verifier</param>
        /// <param name="proof_json">proof json</param>
        /// <param name="schemas_json">all schema jsons participating in the proof</param>
        /// <param name="claim_defs_jsons">all claim definition jsons participating in the proof</param>
        /// <param name="revoc_regs_json">all revocation registry jsons participating in the proof</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_verifier_verify_proof(int command_handle, string proof_request_json, string proof_json, string schemas_json, string claim_defs_jsons, string revoc_regs_json, VerifierVerifyProofCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to by the indy_verifier_verify_proof function.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="valid">true if the proof is valid, otherwise false</param>
        internal delegate void VerifierVerifyProofCompletedDelegate(int xcommand_handle, int err, bool valid);
    }
}
