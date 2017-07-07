using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for anoncreds functions.
    /// </summary>
    public sealed class AnonCreds : AsyncWrapperBase
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


        public static Task<IssuerCreateAndStoreClaimDefResult> IssuerCreateAndStoreClaimDefAsync(Wallet wallet, string schemaJson, string signatureType, bool createNonRevoc)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<IssuerCreateAndStoreClaimDefResult>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_create_and_store_claim_def(
                commandHandle,
                wallet.Handle,
                schemaJson,
                signatureType,
                createNonRevoc,
                IssuerCreateAndStoreClaimDefResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<IssuerCreateAndStoreRevocRegResult> IssuerCreateAndStoreRevocRegAsync(Wallet wallet, int claimDefSeqNo, int maxClaimNum)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<IssuerCreateAndStoreRevocRegResult>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_create_and_store_revoc_reg(
                commandHandle,
                wallet.Handle,
                claimDefSeqNo,
                maxClaimNum,
                IssuerCreateAndStoreClaimRevocRegResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

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

        public static Task<string> IssuerRevokeClaimAsync(Wallet wallet, int claimDefSeqNo, int revocRegSeqNo, int userRevocIndex)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);

            var commandResult = LibSovrin.sovrin_issuer_revoke_claim(
                commandHandle,
                wallet.Handle,
                claimDefSeqNo,
                revocRegSeqNo,
                userRevocIndex,
                IssuerRevokeClaimResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

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
