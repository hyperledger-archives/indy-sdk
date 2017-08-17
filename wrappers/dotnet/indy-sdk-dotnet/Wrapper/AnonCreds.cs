using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.IndyNativeMethods;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for anoncreds functions.
    /// </summary>
    public sealed class AnonCreds : AsyncWrapperBase
    {
        /// <summary>
        /// Gets the callback to use when the IssuerCreateAndStoreClaimDefAsync command completes.
        /// </summary>
        private static IssuerCreateAndStoreClaimDefResultDelegate _issuerCreateAndStoreClaimDefCallback = (xcommand_handle, err, claim_def_json) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<string>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claim_def_json);
        };

        /// <summary>
        /// Gets the callback to use when the IssuerCreateAndStoreClaimRevocRegAsync command completes.
        /// </summary>
        private static IssuerCreateAndStoreClaimRevocRegResultDelegate _issuerCreateAndStoreClaimRevocRegCallback = (xcommand_handle, err, revoc_reg_json, revoc_reg_uuid) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<IssuerCreateAndStoreRevocRegResult>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            var callbackResult = new IssuerCreateAndStoreRevocRegResult(revoc_reg_json, revoc_reg_uuid);

            taskCompletionSource.SetResult(callbackResult);
        };

        /// <summary>
        /// Gets the callback to use when the IssuerCreateClaimAsync command completes.
        /// </summary>
        private static IssuerCreateClaimResultDelegate _issuerCreateClaimCallback = (xcommand_handle, err, revoc_reg_update_json, xclaim_json) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<IssuerCreateClaimResult>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            var callbackResult = new IssuerCreateClaimResult(revoc_reg_update_json, xclaim_json);

            taskCompletionSource.SetResult(callbackResult);
        };


        /// <summary>
        /// Gets the callback to use when the IssuerRevokeClaimAsync command completes.
        /// </summary>
        private static IssuerRevokeClaimResultDelegate IssuerRevokeClaimCallback = (xcommand_handle, err, revoc_reg_update_json) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<string>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(revoc_reg_update_json);
        };

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimOffersAsync command completes.
        /// </summary>
        private static ProverGetClaimOffersResultDelegate _proverGetClaimOffersCallback = (xcommand_handle, err, claim_offer_json) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<string>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claim_offer_json);
        };

        /// <summary>
        /// Gets the callback to use when the roverCreateAndStoreClaimReqAsync command completes.
        /// </summary>
        private static ProverCreateAndStoreClaimReqResultDelegate _proverCreateAndStoreClaimReqCallback = (xcommand_handle, err, claim_req_json) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<string>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claim_req_json);
        };

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimsAsync command completes.
        /// </summary>
        private static ProverGetClaimsResultDelegate _proverGetClaimsCallback = (xcommand_handle, err, claims_json) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<string>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claims_json);
        };

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimsForProofAsync command completes.
        /// </summary>
        private static ProverGetClaimsForProofResultDelegate _proverGetClaimsForProofCallback = (xcommand_handle, err, claims_json) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<string>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claims_json);
        };

        /// <summary>
        /// Gets the callback to use when the ProverCreateProofAsync command completes.
        /// </summary>
        private static ProverCreateProofResultDelegate _proverCreateProofCallback = (xcommand_handle, err, proof_json) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<string>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(proof_json);
        };

        /// <summary>
        /// Gets the callback to use when the VerifierVerifyProofAsync command completes.
        /// </summary>
        private static VerifierVerifyProofResultDelegate _verifierVerifyProofCallback = (xcommand_handle, err, valid) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<bool>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(valid);
        };

        /// <summary>
        /// Create keys (both primary and revocation) for the given schema and signature type.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="issuerDid">The issuer DID.</param>
        /// <param name="schemaJson">The schema of the claim definition.</param>
        /// <param name="signatureType">The type of signature to use.</param>
        /// <param name="createNonRevoc">Whether to request non-revocation claim.</param>
        /// <returns>An asynchronous task that returns a IssuerCreateAndStoreClaimDefResult result.</returns>
        public static Task<string> IssuerCreateAndStoreClaimDefAsync(Wallet wallet, string issuerDid, string schemaJson, string signatureType, bool createNonRevoc)
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_issuer_create_and_store_claim_def(
                commandHandle,
                wallet.Handle,
                issuerDid,
                schemaJson,
                signatureType,
                createNonRevoc,
                _issuerCreateAndStoreClaimDefCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Create a new revocation registry for the given claim definition.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="issuerDid">The DID of the issuer.</param>
        /// <param name="schemaSeqNo">The sequence number of a schema transaction in the ledger.</param>
        /// <param name="maxClaimNum">The maximum number of claims the new registry can process.</param>
        /// <returns>An asynchronous task that returns a IssuerCreateAndStoreRevocRegResult result.</returns>
        public static Task<IssuerCreateAndStoreRevocRegResult> IssuerCreateAndStoreRevocRegAsync(Wallet wallet, string issuerDid, int schemaSeqNo, int maxClaimNum)
        {
            var taskCompletionSource = new TaskCompletionSource<IssuerCreateAndStoreRevocRegResult>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_issuer_create_and_store_revoc_reg(
                commandHandle,
                wallet.Handle,
                issuerDid,
                schemaSeqNo,
                maxClaimNum,
                _issuerCreateAndStoreClaimRevocRegCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Signs a given claim for the given user by a given key (claim def).
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="claimReqJson">a claim request with a blinded secret</param>
        /// <param name="claimJson">a claim containing attribute values for each of requested attribute names.</param>
        /// <param name="revocRegSeqNo">seq no of a revocation registry transaction in Ledger or -1 if revoc_reg_seq_no is absentee.</param>
        /// <param name="userRevocIndex">index of a new user in the revocation registry or -1 if user_revoc_index is absentee.</param>
        /// <returns>An asynchronous task that returns a IssuerCreateClaimResult result.</returns>
        public static Task<IssuerCreateClaimResult> IssuerCreateClaimAsync(Wallet wallet, string claimReqJson, string claimJson, int revocRegSeqNo, int userRevocIndex)
        {
            var taskCompletionSource = new TaskCompletionSource<IssuerCreateClaimResult>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_issuer_create_claim(
                commandHandle,
                wallet.Handle,
                claimReqJson,
                claimJson,
                revocRegSeqNo,
                userRevocIndex,
                _issuerCreateClaimCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Revokes a user identified by a revoc_id in a given revoc-registry.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="revocRegSeqNo">seq no of a revocation registry transaction in Ledger</param>
        /// <param name="userRevocIndex">index of the user in the revocation registry</param>
        /// <returns>An asynchronous task that returns a revocation registry update JSON with a revoked claim.</returns>
        public static Task<string> IssuerRevokeClaimAsync(Wallet wallet, int revocRegSeqNo, int userRevocIndex)
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_issuer_revoke_claim(
                commandHandle,
                wallet.Handle,
                revocRegSeqNo,
                userRevocIndex,
                IssuerRevokeClaimCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Stores a claim offer for a prover.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="claimOfferJson">The claim offer JSON</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        public static Task ProverStoreClaimOfferAsync(Wallet wallet, string claimOfferJson)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_prover_store_claim_offer(
                commandHandle,
                wallet.Handle,
                claimOfferJson,
                _noValueCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets claim offers for a prover.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="filterJson">The filter JSON.</param>
        /// <returns>An asynchronous task that returns a JSON string with a list of claim offers for the filter.</returns>
        public static Task<string> ProverGetClaimOffersAsync(Wallet wallet, string filterJson)
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_prover_get_claim_offers(
                commandHandle,
                wallet.Handle,
                filterJson,
                _proverGetClaimOffersCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a master secret for a prover.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="masterSecretName">The name of the master secret.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        public static Task ProverCreateMasterSecretAsync(Wallet wallet, string masterSecretName)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_prover_create_master_secret(
                commandHandle,
                wallet.Handle,
                masterSecretName,
                _noValueCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates and stores a claim request for a prover.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="proverDid">The DID of the prover.</param>
        /// <param name="claimOfferJson">The claim offer JSON.</param>
        /// <param name="claimDefJson">The claim definition JSON.</param>
        /// <param name="masterSecretName">The master secret name.</param>
        /// <returns>An asynchronous task that returns a claim request JSON.</returns>
        public static Task<string> ProverCreateAndStoreClaimReqAsync(Wallet wallet, string proverDid, string claimOfferJson, string claimDefJson, string masterSecretName)
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_prover_create_and_store_claim_req(
                commandHandle,
                wallet.Handle,
                proverDid,
                claimOfferJson,
                claimDefJson,
                masterSecretName,
                _proverCreateAndStoreClaimReqCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Store a claim for a prover.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="claimsJson">The claims JSON.</param>
        /// <returns>An asynchronous task that returns no value.</returns>
        public static Task ProverStoreClaimAsync(Wallet wallet, string claimsJson)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_prover_store_claim(
                commandHandle,
                wallet.Handle,
                claimsJson,
                _noValueCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Get claims for a prover.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="filterJson">The filter JSON.</param>
        /// <returns>An asynchronous task that returns claim JSON.</returns>
        public static Task<string> ProverGetClaimsAsync(Wallet wallet, string filterJson)
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_prover_get_claims(
                commandHandle,
                wallet.Handle,
                filterJson,
                _proverGetClaimsCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Get prover claims for a proof request.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="proofRequestJson">The proof request JSON.</param>
        /// <returns>An asynchronous task that returns JSON with claims for the given proof request.</returns>
        public static Task<string> ProverGetClaimsForProofReqAsync(Wallet wallet, string proofRequestJson)
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_prover_get_claims_for_proof_req(
                commandHandle,
                wallet.Handle,
                proofRequestJson,
                _proverGetClaimsForProofCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Create a proof for a prover.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="proofReqJson">The proof request JSON.</param>
        /// <param name="requestedClaimsJson">The requested claims JSON.</param>
        /// <param name="schemasJson">The schemas JSON.</param>
        /// <param name="masterSecretName">The master secret name.</param>
        /// <param name="claimDefsJson">The claim definitions JSON.</param>
        /// <param name="revocRegsJson">The recovation registries JSON.</param>
        /// <returns>An asynchronous task that returns proof JSON.</returns>
        public static Task<string> ProverCreateProofAsync(Wallet wallet, string proofReqJson, string requestedClaimsJson, string schemasJson, string masterSecretName, string claimDefsJson, string revocRegsJson)
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_prover_create_proof(
                commandHandle,
                wallet.Handle,
                proofReqJson,
                requestedClaimsJson,
                schemasJson,
                masterSecretName,
                claimDefsJson,
                revocRegsJson,
                _proverCreateProofCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Verify a proof for a verifier.
        /// </summary>
        /// <param name="proofRequestJson">The proof request JSON.</param>
        /// <param name="proofJson">The proof JSON.</param>
        /// <param name="schemasJson">The schemas JSON.</param>
        /// <param name="claimDefsJson">The claim definitions JSON.</param>
        /// <param name="revocRegsJson">The revocation registries JSON.</param>
        /// <returns>An asynchronous task that returns true if the signature is valide, otherwise false.</returns>
        public static Task<bool> VerifierVerifyProofAsync(string proofRequestJson, string proofJson, string schemasJson, string claimDefsJson, string revocRegsJson)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_verifier_verify_proof(
                commandHandle,
                proofRequestJson,
                proofJson,
                schemasJson,
                claimDefsJson,
                revocRegsJson,
                _verifierVerifyProofCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}
