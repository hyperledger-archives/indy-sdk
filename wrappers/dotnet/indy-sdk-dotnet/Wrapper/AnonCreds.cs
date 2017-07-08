using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.LibSovrin;

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
        private static IssuerCreateAndStoreClaimDefResultDelegate IssuerCreateAndStoreClaimDefResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the IssuerCreateAndStoreClaimRevocRegAsync command completes.
        /// </summary>
        private static IssuerCreateAndStoreClaimRevocRegResultDelegate IssuerCreateAndStoreClaimRevocRegResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the IssuerCreateClaimAsync command completes.
        /// </summary>
        private static IssuerCreateClaimResultDelegate IssuerCreateClaimResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the IssuerRevokeClaimAsync command completes.
        /// </summary>
        private static IssuerRevokeClaimResultDelegate IssuerRevokeClaimResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimOffersAsync command completes.
        /// </summary>
        private static ProverGetClaimOffersResultDelegate ProverGetClaimOffersResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the roverCreateAndStoreClaimReqAsync command completes.
        /// </summary>
        private static ProverCreateAndStoreClaimReqResultDelegate ProverCreateAndStoreClaimReqResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimsAsync command completes.
        /// </summary>
        private static ProverGetClaimsResultDelegate ProverGetClaimsResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimsForProofAsync command completes.
        /// </summary>
        private static ProverGetClaimsForProofResultDelegate ProverGetClaimsForProofResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the ProverCreateProofAsync command completes.
        /// </summary>
        private static ProverCreateProofResultDelegate ProverCreateProofResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the VerifierVerifyProofAsync command completes.
        /// </summary>
        private static VerifierVerifyProofResultDelegate VerifierVerifyProofResultCallback { get; }

        /// <summary>
        /// Static initializer.
        /// </summary>
        static AnonCreds()
        {
            IssuerCreateAndStoreClaimDefResultCallback = (xCommandHandle, err, claimDefJson, claimDefUuid) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<IssuerCreateAndStoreClaimDefResult>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                var callbackResult = new IssuerCreateAndStoreClaimDefResult(claimDefJson, claimDefUuid);

                taskCompletionSource.SetResult(callbackResult);
            };

            IssuerCreateAndStoreClaimRevocRegResultCallback = (xCommandHandle, err, claimDefJson, claimDefUuid) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<IssuerCreateAndStoreRevocRegResult>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                var callbackResult = new IssuerCreateAndStoreRevocRegResult(claimDefJson, claimDefUuid);

                taskCompletionSource.SetResult(callbackResult);
            };

            IssuerCreateClaimResultCallback = (xCommandHandle, err, revocRegUpdateJson, xClaimJson) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<IssuerCreateClaimResult>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                var callbackResult = new IssuerCreateClaimResult(revocRegUpdateJson, xClaimJson);

                taskCompletionSource.SetResult(callbackResult);
            };

            IssuerRevokeClaimResultCallback = (xCommandHandle, err, revocRegUpdateJson) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<string>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(revocRegUpdateJson);
            };

            ProverGetClaimOffersResultCallback = (xCommandHandle, err, claimOffersJson) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<string>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(claimOffersJson);
            };

            ProverCreateAndStoreClaimReqResultCallback = (xCommandHandle, err, claimReqJson) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<string>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(claimReqJson);
            };

            ProverGetClaimsResultCallback = (xCommandHandle, err, claimsJson) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<string>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(claimsJson);
            };

            ProverGetClaimsForProofResultCallback = (xCommandHandle, err, claimsJson) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<string>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(claimsJson);
            };

            ProverCreateProofResultCallback = (xCommandHandle, err, proofJson) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<string>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(proofJson);
            };

            VerifierVerifyProofResultCallback = (xCommandHandle, err, valid) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<bool>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(valid);
            };

        }

        /// <summary>
        /// Create keys (both primary and revocation) for the given schema and signature type.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="issuerDid">The issuer DID.</param>
        /// <param name="schemaJson">The schema of the claim definition.</param>
        /// <param name="signatureType">The type of signature to use.</param>
        /// <param name="createNonRevoc">Whether to request non-revocation claim.</param>
        /// <returns>An asynchronous task that returns a IssuerCreateAndStoreClaimDefResult result.</returns>
        public static Task<IssuerCreateAndStoreClaimDefResult> IssuerCreateAndStoreClaimDefAsync(Wallet wallet, string issuerDid, string schemaJson, string signatureType, bool createNonRevoc)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<IssuerCreateAndStoreClaimDefResult>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_create_and_store_claim_def(
                commandHandle,
                wallet.Handle,
                issuerDid,
                schemaJson,
                signatureType,
                createNonRevoc,
                IssuerCreateAndStoreClaimDefResultCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<IssuerCreateAndStoreRevocRegResult>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_create_and_store_revoc_reg(
                commandHandle,
                wallet.Handle,
                issuerDid,
                schemaSeqNo,
                maxClaimNum,
                IssuerCreateAndStoreClaimRevocRegResultCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<IssuerCreateClaimResult>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_create_claim(
                commandHandle,
                wallet.Handle,
                claimReqJson,
                claimJson,
                revocRegSeqNo,
                userRevocIndex,
                IssuerCreateClaimResultCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_revoke_claim(
                commandHandle,
                wallet.Handle,
                revocRegSeqNo,
                userRevocIndex,
                IssuerRevokeClaimResultCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_store_claim_offer(
                commandHandle,
                wallet.Handle,
                claimOfferJson,
                ResultOnlyCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_get_claim_offers(
                commandHandle,
                wallet.Handle,
                filterJson,
                ProverGetClaimOffersResultCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_create_master_secret(
                commandHandle,
                wallet.Handle,
                masterSecretName,
                ResultOnlyCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_create_and_store_claim_req(
                commandHandle,
                wallet.Handle,
                proverDid,
                claimOfferJson,
                claimDefJson,
                masterSecretName,
                ProverCreateAndStoreClaimReqResultCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_store_claim(
                commandHandle,
                wallet.Handle,
                claimsJson,
                ResultOnlyCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_get_claims(
                commandHandle,
                wallet.Handle,
                filterJson,
                ProverGetClaimsResultCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_get_claims_for_proof_req(
                commandHandle,
                wallet.Handle,
                proofRequestJson,
                ProverGetClaimsForProofResultCallback
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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_create_proof(
                commandHandle,
                wallet.Handle,
                proofReqJson,
                requestedClaimsJson,
                schemasJson,
                masterSecretName,
                claimDefsJson,
                revocRegsJson,
                ProverCreateProofResultCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Verify a proof for a verifier.
        /// </summary>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="proofRequestJson">The proof request JSON.</param>
        /// <param name="proofJson">The proof JSON.</param>
        /// <param name="schemasJson">The schemas JSON.</param>
        /// <param name="claimDefsJson">The claim definitions JSON.</param>
        /// <param name="revocRegsJson">The revocation registries JSON.</param>
        /// <returns>An asynchronous task that returns true if the signature is valide, otherwise false.</returns>
        public static Task<bool> VerifierVerifyProofAsync(Wallet wallet, string proofRequestJson, string proofJson, string schemasJson, string claimDefsJson, string revocRegsJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var commandResult = LibSovrin.sovrin_verifier_verify_proof(
                commandHandle,
                wallet.Handle,
                proofRequestJson,
                proofJson,
                schemasJson,
                claimDefsJson,
                revocRegsJson,
                VerifierVerifyProofResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}
