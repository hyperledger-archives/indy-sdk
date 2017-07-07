using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Async wrapper for anoncreds functions.
    /// </summary>
    public sealed class AnonCredsWrapper : AsyncWrapperBase
    {
        private static IssuerCreateAndStoreClaimDefResultDelegate IssuerCreateAndStoreClaimDefResultCallback { get; }
        private static IssuerCreateAndStoreClaimRevocRegResultDelegate IssuerCreateAndStoreClaimRevocRegResultCallback { get; }
        private static IssuerCreateClaimResultDelegate IssuerCreateClaimResultCallback { get; }
        private static IssuerRevokeClaimResultDelegate IssuerRevokeClaimResultCallback { get; }
        private static ProverGetClaimOffersResultDelegate ProverGetClaimOffersResultCallback { get; }
        private static ProverCreateAndStoreClaimReqResultDelegate ProverCreateAndStoreClaimReqResultCallback { get; }
        private static ProverGetClaimsResultDelegate ProverGetClaimsResultCallback { get; }
        private static ProverGetClaimsForProofResultDelegate ProverGetClaimsForProofResultCallback { get; }
        private static ProverCreateProofResultDelegate ProverCreateProofResultCallback { get; }
        private static VerifierVerifyProofResultDelegate VerifierVerifyProofResultCallback { get; }

        static AnonCredsWrapper()
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


        public static Task<IssuerCreateAndStoreClaimDefResult> IssuerCreateAndStoreClaimDefAsync(IntPtr walletHandle, string schemaJson, string signatureType, bool createNonRevoc)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<IssuerCreateAndStoreClaimDefResult>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_create_and_store_claim_def(
                commandHandle,
                walletHandle,
                schemaJson,
                signatureType,
                createNonRevoc,
                IssuerCreateAndStoreClaimDefResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<IssuerCreateAndStoreRevocRegResult> IssuerCreateAndStoreRevocRegAsync(IntPtr walletHandle, int claimDefSeqNo, int maxClaimNum)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<IssuerCreateAndStoreRevocRegResult>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_create_and_store_revoc_reg(
                commandHandle,
                walletHandle,
                claimDefSeqNo,
                maxClaimNum,
                IssuerCreateAndStoreClaimRevocRegResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<IssuerCreateClaimResult> IssuerCreateClaimAsync(IntPtr walletHandle, string claimReqJson, string claimJson, int revocRegSeqNo, int userRevocIndex)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<IssuerCreateClaimResult>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_create_claim(
                commandHandle,
                walletHandle,
                claimReqJson,
                claimJson,
                revocRegSeqNo,
                userRevocIndex,
                IssuerCreateClaimResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<string> IssuerRevokeClaimAsync(IntPtr walletHandle, int claimDefSeqNo, int revocRegSeqNo, int userRevocIndex)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_revoke_claim(
                commandHandle,
                walletHandle,
                claimDefSeqNo,
                revocRegSeqNo,
                userRevocIndex,
                IssuerRevokeClaimResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task ProverStoreClaimOfferAsync(IntPtr walletHandle, string claimOfferJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_store_claim_offer(
                commandHandle,
                walletHandle,
                claimOfferJson,
                ResultOnlyCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<string> ProverGetClaimOffersAsync(IntPtr walletHandle, string filterJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_get_claim_offers(
                commandHandle,
                walletHandle,
                filterJson,
                ProverGetClaimOffersResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task ProverCreateMasterSecretAsync(IntPtr walletHandle, string masterSecretName)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_create_master_secret(
                commandHandle,
                walletHandle,
                masterSecretName,
                ResultOnlyCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<string> ProverCreateAndStoreClaimReqAsync(IntPtr walletHandle, string proverDid, string claimOfferJson, string claimDefJson, string masterSecretName)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_create_and_store_claim_req(
                commandHandle,
                walletHandle,
                proverDid,
                claimOfferJson,
                claimDefJson,
                masterSecretName,
                ProverCreateAndStoreClaimReqResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task ProverStoreClaimAsync(IntPtr walletHandle, string claimsJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_store_claim(
                commandHandle,
                walletHandle,
                claimsJson,
                ResultOnlyCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<string> ProverGetClaimsAsync(IntPtr walletHandle, string filterJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_get_claims(
                commandHandle,
                walletHandle,
                filterJson,
                ProverGetClaimsResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<string> ProverGetClaimsForProofReqAsync(IntPtr walletHandle, string proofRequestJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_get_claims_for_proof_req(
                commandHandle,
                walletHandle,
                proofRequestJson,
                ProverGetClaimsForProofResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<string> ProverCreateProofAsync(IntPtr walletHandle, string proofReqJson, string requestedClaimsJson, string schemasJson, string masterSecretName, string claimDefsJson, string revocRegsJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_prover_create_proof(
                commandHandle,
                walletHandle,
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

        public static Task<bool> VerifierVerifyProofAsync(IntPtr walletHandle, string proofRequestJson, string proofJson, string schemasJson, string claimDefsJson, string revocRegsJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var commandResult = LibSovrin.sovrin_verifier_verify_proof(
                commandHandle,
                walletHandle,
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
