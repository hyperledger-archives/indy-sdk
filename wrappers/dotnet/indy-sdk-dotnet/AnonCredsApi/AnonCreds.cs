using Hyperledger.Indy.BlobStorageApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Threading.Tasks;
using static Hyperledger.Indy.AnonCredsApi.NativeMethods;

namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Provides methods for managing anonymous credentials.
    /// </summary>
    public static class AnonCreds
    {
        /// <summary>
        /// Gets the callback to use when the IssuerCreateAndStoreClaimDefAsync command completes.
        /// </summary>
        private static IssuerCreateSchemaCompletedDelegate _issuerCreateSchemaCallback = (xcommand_handle, err, schema_id, schema_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<IssuerCreateSchemaResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new IssuerCreateSchemaResult(schema_id, schema_json));
        };

        /// <summary>
        /// Gets the callback to use when the IssuerCreateAndStoreClaimDefAsync command completes.
        /// </summary>
        private static IssuerCreateAndStoreCredentialDefCompletedDelegate _issuerCreateAndStoreClaimDefCallback = (xcommand_handle, err, claim_def_id, claim_def_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<IssuerCreateAndStoreCredentialDefResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new IssuerCreateAndStoreCredentialDefResult(claim_def_id, claim_def_json));
        };

        /// <summary>
        /// Gets the callback to use when the IssuerCreateAndStoreClaimRevocRegAsync command completes.
        /// </summary>
        private static IssuerCreateAndStoreRevocRegCompletedDelegate _issuerCreateAndStoreClaimRevocRegCallback = (xcommand_handle, err, revoc_reg_id, revoc_reg_def_json, revoc_reg_entry_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<IssuerCreateAndStoreRevocRegResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new IssuerCreateAndStoreRevocRegResult(revoc_reg_id, revoc_reg_def_json, revoc_reg_entry_json));
        };

        /// <summary>
        /// Gets the callback to use when the IssuerCreateClaimAsync command completes.
        /// </summary>
        private static IssuerCreateCredentialOfferCompletedDelegate _issuerCreateCredentialOfferCallback = (xcommand_handle, err, cred_offer_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(cred_offer_json);
        };

        /// <summary>
        /// Gets the callback to use when the IssuerCreateClaimAsync command completes.
        /// </summary>
        private static IssuerCreateCredentialCompletedDelegate _issuerCreateCredentialCallback = (xcommand_handle, err, cred_json, cred_revoc_id, revoc_reg_delta_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<IssuerCreateCredentialResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var callbackResult = new IssuerCreateCredentialResult(cred_json, cred_revoc_id, revoc_reg_delta_json);

            taskCompletionSource.SetResult(callbackResult);
        };


        /// <summary>
        /// Gets the callback to use when the IssuerRevokeCredentialAsync command completes.
        /// </summary>
        private static IssuerRevokeCredentialCompletedDelegate _issuerRevokeCredentialCallback = (xcommand_handle, err, revoc_reg_delta_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(revoc_reg_delta_json);
        };

        /// <summary>
        /// The issuer merge revocation registry deltas callback.
        /// </summary>
        private static IssuerMergeRevocationRegistryDeltasCompletedDelegate _issuerMergeRevocationRegistryDeltasCallback = (xcommand_handle, err, merged_rev_reg_delta) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(merged_rev_reg_delta);
        };

        /// <summary>
        /// The prover create master secret callback.
        /// </summary>
        private static ProverCreateMasterSecretCompletedDelegate _proverCreateMasterSecretCallback = (xcommand_handle, err, out_master_secret_id) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(out_master_secret_id);
        };

        /// <summary>
        /// Gets the callback to use when the roverCreateAndStoreClaimReqAsync command completes.
        /// </summary>
        private static ProverCreateCredentialReqCompletedDelegate _proverCreateCredentialReqCallback = (xcommand_handle, err, cred_req_json, cred_req_metadata_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<ProverCreateCredentialRequestResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new ProverCreateCredentialRequestResult(cred_req_json, cred_req_metadata_json));
        };

        /// <summary>
        /// The prover store credential callback.
        /// </summary>
        private static ProverStoreCredentialCompletedDelegate _proverStoreCredentialCallback = (xcommand_handle, err, out_cred_id) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(out_cred_id);
        };

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimsAsync command completes.
        /// </summary>
        private static ProverGetCredentialsCompletedDelegate _proverGetCredentialsCallback = (xcommand_handle, err, matched_credentials_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(matched_credentials_json);
        };

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimsForProofAsync command completes.
        /// </summary>
        private static ProverGetCredentialsForProofCompletedDelegate _proverGetClaimsForProofCallback = (xcommand_handle, err, claims_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claims_json);
        };

        /// <summary>
        /// Gets the callback to use when the ProverCreateProofAsync command completes.
        /// </summary>
        private static ProverCreateProofCompletedDelegate _proverCreateProofCallback = (xcommand_handle, err, proof_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(proof_json);
        };

        /// <summary>
        /// Gets the callback to use when the VerifierVerifyProofAsync command completes.
        /// </summary>
        private static VerifierVerifyProofCompletedDelegate _verifierVerifyProofCallback = (xcommand_handle, err, valid) =>
        {
            var taskCompletionSource = PendingCommands.Remove<bool>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(valid);
        };

        /// <summary>
        /// The create revocation state callback.
        /// </summary>
        private static CreateRevocationStateCompletedDelegate _createRevocationStateCallback = (xcommand_handle, err, rev_state_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(rev_state_json);
        };

        /// <summary>
        /// The update revocation state callback.
        /// </summary>
        private static UpdateRevocationStateCompletedDelegate _updateRevocationStateCallback = (xcommand_handle, err, updated_rev_state_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(updated_rev_state_json);
        };

        /// <summary>
        /// Create credential schema entity that describes credential attributes list and allows credentials
        /// interoperability.
        ///
        /// Schema is public and intended to be shared with all anoncreds workflow actors usually by publishing SCHEMA transaction
        /// to Indy distributed ledger.
        ///
        /// It is IMPORTANT for current version POST Schema in Ledger and after that GET it from Ledger
        /// with correct seq_no to save compatibility with Ledger.
        /// After that can call indy_issuer_create_and_store_credential_def to build corresponding Credential Definition.
        ///
        /// </summary>
        /// <returns>
        /// schemaId: identifier of created schema
        /// schemaJson: schema as json
        /// </returns>
        /// <param name="issuerDid">DID of schema issuer</param>
        /// <param name="name">Name of the schema</param>
        /// <param name="version">Version of the schema</param>
        /// <param name="attrs">A list of schema attributes descriptions</param>
        public static Task<IssuerCreateSchemaResult> IssuerCreateSchemaAsync(string issuerDid, string name, string version, string attrs)
        {
            ParamGuard.NotNullOrWhiteSpace(issuerDid, nameof(issuerDid));
            ParamGuard.NotNullOrWhiteSpace(name, nameof(name));
            ParamGuard.NotNullOrWhiteSpace(version, nameof(version));
            ParamGuard.NotNullOrWhiteSpace(attrs, nameof(attrs));

            var taskCompletionSource = new TaskCompletionSource<IssuerCreateSchemaResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_create_schema(
                commandHandle,
                issuerDid,
                name,
                version,
                attrs,
                _issuerCreateSchemaCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates keys for the given schema and signature type.
        /// </summary>
        /// <remarks>
        /// <para>This method creates both primary and revocation keys for the given
        /// signature type and schema and stores them in the provided <paramref name="wallet"/>.
        /// The generated claim definition is returned as a JSON string containing information about the 
        /// signature type, schema, the issuer's public key and the unique identifier of the public key 
        /// in the wallet.
        /// </para>
        /// <note type="note">Currently the only signature type that is supported is 'CL'.</note>
        /// </remarks>
        /// <param name="wallet">The wallet into which the claim definition will be stored.</param>
        /// <param name="issuerDid">The DID of the issuer of the claim definition.</param>
        /// <param name="schemaJson">The JSON schema of the claim definition.</param>
        /// <param name="tag">Allows to distinct between credential definitions for the same issuer and schema</param>
        /// <param name="type">The type of signature to use.</param>
        /// <param name="configJson">Whether to request non-revocation claim.</param>
        /// <returns>
        /// credDefId: identifier of created credential definition
        /// credDefJson: public part of created credential definition
        /// </returns>
        public static Task<IssuerCreateAndStoreCredentialDefResult> IssuerCreateAndStoreCredentialDefAsync(Wallet wallet, string issuerDid, string schemaJson, string tag, string type, string configJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(issuerDid, "issuerDid");
            ParamGuard.NotNullOrWhiteSpace(schemaJson, "schemaJson");

            var taskCompletionSource = new TaskCompletionSource<IssuerCreateAndStoreCredentialDefResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_create_and_store_credential_def(
                commandHandle,
                wallet.Handle,
                issuerDid,
                schemaJson,
                tag,
                type,
                configJson,
                _issuerCreateAndStoreClaimDefCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Create a new revocation registry for the given credential definition as tuple of entities:
        /// - Revocation registry definition that encapsulates credentials definition reference, revocation type specific configuration and
        ///   secrets used for credentials revocation
        /// - Revocation registry state that stores the information about revoked entities in a non-disclosing way. The state can be
        ///   represented as ordered list of revocation registry entries were each entry represents the list of revocation or issuance operations.
        ///
        /// Revocation registry definition entity contains private and public parts. Private part will be stored in the wallet. Public part
        /// will be returned as json intended to be shared with all anoncreds workflow actors usually by publishing REVOC_REG_DEF transaction
        /// to Indy distributed ledger.
        ///
        /// Revocation registry state is stored on the wallet and also intended to be shared as the ordered list of REVOC_REG_ENTRY transactions.
        /// This call initializes the state in the wallet and returns the initial entry.
        ///
        /// Some revocation registry types (for example, 'CL_ACCUM') can require generation of binary blob called tails used to hide information about revoked credentials in public
        /// revocation registry and intended to be distributed out of leger (REVOC_REG_DEF transaction will still contain uri and hash of tails).
        /// This call requires access to pre-configured blob storage writer instance handle that will allow to write generated tails.
        ///
        /// </summary>
        /// <returns>
        /// revoc_reg_id: identifier of created revocation registry definition
        /// revoc_reg_def_json: public part of revocation registry definition
        /// revoc_reg_entry_json: revocation registry entry that defines initial state of revocation registry</returns>
        /// <param name="wallet">wallet handler (created by open_wallet)..</param>
        /// <param name="issuerDid">a DID of the issuer signing transaction to the Ledger.</param>
        /// <param name="type">revocation registry type (optional, default value depends on credential definition type). Supported types are:
        /// - 'CL_ACCUM': Type-3 pairing based accumulator. Default for 'CL' credential definition type.</param>
        /// <param name="tag">allows to distinct between revocation registries for the same issuer and credential definition.</param>
        /// <param name="credDefId">id of stored in ledger credential definition.</param>
        /// <param name="configJson">type-specific configuration of revocation registry as json:
        /// - 'CL_ACCUM': {
        ///     "issuance_type": (optional) type of issuance. Currently supported:
        ///         1) ISSUANCE_BY_DEFAULT: all indices are assumed to be issued and initial accumulator is calculated over all indices;
        ///            Revocation Registry is updated only during revocation.
        ///         2) ISSUANCE_ON_DEMAND: nothing is issued initially accumulator is 1 (used by default);
        ///     "max_cred_num": maximum number of credentials the new registry can process (optional, default 100000)
        /// }.</param>
        /// <param name="tailsWriter">handle of blob storage to store tails</param>
        public static Task<IssuerCreateAndStoreRevocRegResult> IssuerCreateAndStoreRevocRegAsync(Wallet wallet, string issuerDid, string type, string tag, string credDefId, string configJson, BlobStorageWriter tailsWriter)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(issuerDid, "issuerDid");
            ParamGuard.NotNullOrWhiteSpace(tag, "tag");
            ParamGuard.NotNullOrWhiteSpace(credDefId, "credDefId");
            ParamGuard.NotNullOrWhiteSpace(configJson, "configJson");

            var taskCompletionSource = new TaskCompletionSource<IssuerCreateAndStoreRevocRegResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_create_and_store_revoc_reg(
                commandHandle,
                wallet.Handle,
                issuerDid,
                type,
                tag,
                credDefId,
                configJson,
                tailsWriter.Handle,
                _issuerCreateAndStoreClaimRevocRegCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Create credential offer that will be used by Prover for
        /// credential request creation. Offer includes nonce and key correctness proof
        /// for authentication between protocol steps and integrity checking.
        /// </summary>
        /// <returns>
        /// credential offer json:
        ///     {
        ///         "schema_id": string,
        ///         "cred_def_id": string,
        ///         // Fields below can depend on Cred Def type
        ///         "nonce": string,
        ///         "key_correctness_proof" : [key_correctness_proof]
        ///     }
        /// </returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="credDefId"> id of credential definition stored in the wallet</param>
        public static Task<string> IssuerCreateCredentialOfferAsync(Wallet wallet, string credDefId)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(credDefId, "credDefId");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_create_credential_offer(
                commandHandle,
                wallet.Handle,
                credDefId,
                _issuerCreateCredentialOfferCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Check Cred Request for the given Cred Offer and issue Credential for the given Cred Request.
        ///
        /// Cred Request must match Cred Offer. The credential definition and revocation registry definition
        /// referenced in Cred Offer and Cred Request must be already created and stored into the wallet.
        ///
        /// Information for this credential revocation will be store in the wallet as part of revocation registry under
        /// generated cred_revoc_id local for this wallet.
        ///
        /// This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
        /// Note that it is possible to accumulate deltas to reduce ledger load.
        /// </summary>
        /// <returns>
        /// cred_json: Credential json containing signed credential values
        ///     {
        ///         "schema_id": string,
        ///         "cred_def_id": string,
        ///         "rev_reg_def_id", Optional&lt;string&gt;,
        ///         "values": [see cred_values_json above],
        ///         // Fields below can depend on Cred Def type
        ///         "signature": [signature],
        ///         "signature_correctness_proof": [signature_correctness_proof]
        ///     }
        /// cred_revoc_id: local id for revocation info (Can be used for revocation of this cred)
        /// revoc_reg_delta_json: Revocation registry delta json with a newly issued credential
        /// </returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="credOfferJson">a cred offer created by indy_issuer_create_credential_offer</param>
        /// <param name="credReqJson">a credential request created by indy_prover_create_credential_req.</param>
        /// <param name="credValuesJson">a credential containing attribute values for each of requested attribute names.
        ///     Example:
        ///     {
        ///      "attr1" : {"raw": "value1", "encoded": "value1_as_int" },
        ///      "attr2" : {"raw": "value1", "encoded": "value1_as_int" }
        ///     }</param>
        /// <param name="revRegId">id of revocation registry stored in the wallet.</param>
        /// <param name="blobStorageReader">configuration of blob storage reader handle that will allow to read revocation tails</param>
        public static Task<IssuerCreateCredentialResult> IssuerCreateCredentialAsync(Wallet wallet, string credOfferJson, string credReqJson, string credValuesJson, string revRegId, BlobStorageReader blobStorageReader)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(credOfferJson, "credOfferJson");
            ParamGuard.NotNullOrWhiteSpace(credReqJson, "credReqJson");
            ParamGuard.NotNullOrWhiteSpace(credValuesJson, "credValuesJson");


            var taskCompletionSource = new TaskCompletionSource<IssuerCreateCredentialResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_create_credential(
                commandHandle,
                wallet.Handle,
                credOfferJson,
                credReqJson,
                credValuesJson,
                revRegId,
                blobStorageReader?.Handle ?? -1,
                _issuerCreateCredentialCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// 
        /// Revoke a credential identified by a cred_revoc_id (returned by indy_issuer_create_credential).
        ///
        /// The corresponding credential definition and revocation registry must be already
        /// created an stored into the wallet.
        ///
        /// This call returns revoc registry delta as json file intended to be shared as REVOC_REG_ENTRY transaction.
        /// Note that it is possible to accumulate deltas to reduce ledger load.
        /// </summary>
        /// <returns>revoc_reg_delta_json: Revocation registry delta json with a revoked credential.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="blobStorageReader">configuration of blob storage reader handle that will allow to read revocation tails</param>
        /// <param name="revRegId">id of revocation registry stored in wallet</param>
        /// <param name="credRevocId">local id for revocation info.</param>
        public static Task<string> IssuerRevokeCredentialAsync(Wallet wallet, BlobStorageReader blobStorageReader, string revRegId, string credRevocId)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(revRegId, "revRegId");
            ParamGuard.NotNullOrWhiteSpace(credRevocId, "credRevocId");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_revoke_credential(
                commandHandle,
                wallet.Handle,
                blobStorageReader.Handle,
                revRegId,
                credRevocId,
                _issuerRevokeCredentialCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Merge two revocation registry deltas (returned by indy_issuer_create_credential or indy_issuer_revoke_credential) to accumulate common delta.
        /// Send common delta to ledger to reduce the load.
        /// </summary>
        /// <returns>merged_rev_reg_delta: Merged revocation registry delta.</returns>
        /// <param name="revRegDelta">revocation registry delta.</param>
        /// <param name="otherRevRegDelta">revocation registry delta for which PrevAccum value  is equal to current accum value of rev_reg_delta_json..</param>
        public static Task<string> IssuerMergeRevocationRegistryDeltasAsync(string revRegDelta, string otherRevRegDelta)
        {
            ParamGuard.NotNullOrWhiteSpace(revRegDelta, "revRegDelta");
            ParamGuard.NotNullOrWhiteSpace(otherRevRegDelta, "otherRevRegDelta");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_merge_revocation_registry_deltas(
                commandHandle,
                revRegDelta,
                otherRevRegDelta,
                _issuerMergeRevocationRegistryDeltasCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a master secret with the specified name and stores it in the provided wallet.
        /// </summary>
        /// <remarks>
        /// The name of the master secret must be unique within the wallet.
        /// </remarks>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="masterSecretId">The name of the master secret.</param>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation has completed.</returns>
        public static Task<string> ProverCreateMasterSecretAsync(Wallet wallet, string masterSecretId)
        {
            ParamGuard.NotNull(wallet, "wallet");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_create_master_secret(
                commandHandle,
                wallet.Handle,
                masterSecretId,
                _proverCreateMasterSecretCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a credential request for the given credential offer.
        ///
        /// The method creates a blinded master secret for a master secret identified by a provided name.
        /// The master secret identified by the name must be already stored in the secure wallet (see prover_create_master_secret)
        /// The blinded master secret is a part of the credential request.
        /// </summary>
        /// <returns>
        /// cred_req_json: Credential request json for creation of credential by Issuer
        ///     {
        ///      "prover_did" : string,
        ///      "cred_def_id" : string,
        ///         // Fields below can depend on Cred Def type
        ///      "blinded_ms" : [blinded_master_secret],
        ///      "blinded_ms_correctness_proof" : [blinded_ms_correctness_proof],
        ///      "nonce": string
        ///    }
        /// cred_req_metadata_json: Credential request metadata json for processing of received form Issuer credential.
        ///</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="proverDid">a DID of the prover.</param>
        /// <param name="credOfferJson">credential offer as a json containing information about the issuer and a credential.</param>
        /// <param name="credDefJson">credential definition json.</param>
        /// <param name="masterSecretId">the id of the master secret stored in the wallet.</param>
        public static Task<ProverCreateCredentialRequestResult> ProverCreateCredentialReqAsync(Wallet wallet, string proverDid, string credOfferJson, string credDefJson, string masterSecretId)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(proverDid, "proverDid");
            ParamGuard.NotNullOrWhiteSpace(credOfferJson, "credOfferJson");
            ParamGuard.NotNullOrWhiteSpace(credDefJson, "credDefJson");
            ParamGuard.NotNullOrWhiteSpace(masterSecretId, "masterSecretId");

            var taskCompletionSource = new TaskCompletionSource<ProverCreateCredentialRequestResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_create_credential_req(
                commandHandle,
                wallet.Handle,
                proverDid,
                credOfferJson,
                credDefJson,
                masterSecretId,
                _proverCreateCredentialReqCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Check credential provided by Issuer for the given credential request,
        /// updates the credential by a master secret and stores in a secure wallet.
        /// </summary>
        /// <returns>out_cred_id: identifier by which credential is stored in the wallet.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="credId">(optional, default is a random one) identifier by which credential will be stored in the wallet</param>
        /// <param name="credReqMetadataJson">a credential request metadata created by indy_prover_create_credential_req</param>
        /// <param name="credJson">credential json received from issuer.</param>
        /// <param name="credDefJson">redential definition json.</param>
        /// <param name="revRegDefJson">revocation registry definition json.</param>
        public static Task<string> ProverStoreCredentialAsync(Wallet wallet, string credId, string credReqMetadataJson, string credJson, string credDefJson, string revRegDefJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(credReqMetadataJson, "credReqMetadataJson");
            ParamGuard.NotNullOrWhiteSpace(credJson, "credJson");
            ParamGuard.NotNullOrWhiteSpace(credDefJson, "credDefJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_store_credential(
                commandHandle,
                wallet.Handle,
                credId,
                credReqMetadataJson,
                credJson,
                credDefJson,
                revRegDefJson,
                _proverStoreCredentialCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets human readable credentials according to the filter.
        /// If filter is NULL, then all credentials are returned.
        /// Credentials can be filtered by Issuer, credential_def and/or Schema.
        /// </summary>
        /// <returns>/// credentials json
        ///     [{
        ///         "referent": string, // cred_id in the wallet
        ///         "values": [see cred_values_json above],
        ///         "schema_id": string,
        ///         "cred_def_id": string,
        ///         "rev_reg_id": Optional string,
        ///         "cred_rev_id": Optional string
        ///     }].</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="filterJson">filter_json: filter for credentials
        ///        {
        ///            "schema_id": string, (Optional)
        ///            "schema_issuer_did": string, (Optional)
        ///            "schema_name": string, (Optional)
        ///            "schema_version": string, (Optional)
        ///            "issuer_did": string, (Optional)
        ///            "cred_def_id": string, (Optional)
        ///        }</param>
        public static Task<string> ProverGetCredentialsAsync(Wallet wallet, string filterJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(filterJson, "filterJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_get_credentials(
                commandHandle,
                wallet.Handle,
                filterJson,
                _proverGetCredentialsCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets human readable credentials matching the given proof request.
        /// </summary>
        /// <returns>
        /// credentials_json: json with credentials for the given pool request.
        ///     {
        ///         "requested_attrs": {
        ///             "[attr_referent]": [{ cred_info: (credential_info), interval: Optional(non_revoc_interval) }],
        ///             ...,
        ///         },
        ///         "requested_predicates": {
        ///             "requested_predicates": [{ cred_info: (credential_info), timestamp: Optional integer }, { cred_info: (credential_2_info), timestamp: Optional integer }],
        ///             "requested_predicate_2_referent": [{ cred_info: (credential_2_info), timestamp: Optional integer }]
        ///         }
        ///     }, where credential is
        ///     {
        ///         "referent": string,
        ///         "attrs": [{"attr_name" : "attr_raw_value"}],
        ///         "schema_id": string,
        ///         "cred_def_id": string,
        ///         "rev_reg_id": Optional int,
        ///         "cred_rev_id": Optional int,
        ///     }
        /// </returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="proofRequestJson">/// proof_request_json: proof request json
        ///     {
        ///         "name": string,
        ///         "version": string,
        ///         "nonce": string,
        ///         "requested_attributes": { // set of requested attributes
        ///              "[attr_referent]": [attr_info], // see below
        ///              ...,
        ///         },
        ///         "requested_predicates": { // set of requested predicates
        ///              "[predicate_referent]": [predicate_info], // see below
        ///              ...,
        ///          },
        ///         "non_revoked": Optional [non_revoc_interval], // see below,
        ///                        // If specified prover must proof non-revocation
        ///                        // for date in this interval for each attribute
        ///                        // (can be overridden on attribute level)
        ///     }
        /// 
        /// where
        /// attr_referent: Proof-request local identifier of requested attribute
        /// attr_info: Describes requested attribute
        ///     {
        ///         "name": string, // attribute name, (case insensitive and ignore spaces)
        ///         "restrictions": Optional ['attr_filter'] // see below,
        ///                         // if specified, credential must satisfy to one of the given restriction.
        ///         "non_revoked": Optional [non_revoc_interval], // see below,
        ///                        // If specified prover must proof non-revocation
        ///                        // for date in this interval this attribute
        ///                        // (overrides proof level interval)
        ///     }
        /// predicate_referent: Proof-request local identifier of requested attribute predicate
        /// predicate_info: Describes requested attribute predicate
        ///     {
        ///         "name": attribute name, (case insensitive and ignore spaces)
        ///         "p_type": predicate type (Currently >= only)
        ///         "p_value": predicate value
        ///         "restrictions": Optional ['attr_filter'] // see below,
        ///                         // if specified, credential must satisfy to one of the given restriction.
        ///         "non_revoked": Optional [non_revoc_interval], // see below,
        ///                        // If specified prover must proof non-revocation
        ///                        // for date in this interval this attribute
        ///                        // (overrides proof level interval)
        ///     }
        /// non_revoc_interval: Defines non-revocation interval
        ///     {
        ///         "from": Optional int, // timestamp of interval beginning
        ///         "to": Optional int, // timestamp of interval ending
        ///     }
        /// filter: see filter_json above
        /// </param>
        public static Task<string> ProverGetCredentialsForProofReqAsync(Wallet wallet, string proofRequestJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(proofRequestJson, "proofRequestJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_get_credentials_for_proof_req(
                commandHandle,
                wallet.Handle,
                proofRequestJson,
                _proverGetClaimsForProofCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a proof according to the given proof request
        /// Either a corresponding credential with optionally revealed attributes or self-attested attribute must be provided
        /// for each requested attribute (see indy_prover_get_credentials_for_pool_req).
        /// A proof request may request multiple credentials from different schemas and different issuers.
        /// All required schemas, public keys and revocation registries must be provided.
        /// The proof request also contains nonce.
        /// The proof contains either proof or self-attested attribute value for each requested attribute.
        ///
        /// </summary>
        /// <remarks>
        /// where
        /// attr_referent: Proof-request local identifier of requested attribute
        /// attr_info: Describes requested attribute
        ///     {
        ///         "name": string, // attribute name, (case insensitive and ignore spaces)
        ///         "restrictions": Optional ['attr_filter'] // see above,
        ///                         // if specified, credential must satisfy to one of the given restriction.
        ///         "non_revoked": Optional [non_revoc_interval], // see below,
        ///                        // If specified prover must proof non-revocation
        ///                        // for date in this interval this attribute
        ///                        // (overrides proof level interval)
        ///     }
        /// predicate_referent: Proof-request local identifier of requested attribute predicate
        /// predicate_info: Describes requested attribute predicate
        ///     {
        ///         "name": attribute name, (case insensitive and ignore spaces)
        ///         "p_type": predicate type (Currently >= only)
        ///         "p_value": predicate value
        ///         "restrictions": Optional ['attr_filter'] // see above,
        ///                         // if specified, credential must satisfy to one of the given restriction.
        ///         "non_revoked": Optional [non_revoc_interval], // see below,
        ///                        // If specified prover must proof non-revocation
        ///                        // for date in this interval this attribute
        ///                        // (overrides proof level interval)
        ///     }
        /// non_revoc_interval: Defines non-revocation interval
        ///     {
        ///         "from": Optional int, // timestamp of interval beginning
        ///         "to": Optional int, // timestamp of interval ending
        ///     }
        ///
        /// </remarks>
        /// <returns>
        /// Proof json
        /// For each requested attribute either a proof (with optionally revealed attribute value) or
        /// self-attested attribute value is provided.
        /// Each proof is associated with a credential and corresponding schema_id, cred_def_id, rev_reg_id and timestamp.
        /// There is also aggregated proof part common for all credential proofs.
        ///     {
        ///         "requested": {
        ///             "revealed_attrs": {
        ///                 "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
        ///                 "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
        ///             },
        ///             "unrevealed_attrs": {
        ///                 "requested_attr3_id": {sub_proof_index: number}
        ///             },
        ///             "self_attested_attrs": {
        ///                 "requested_attr2_id": self_attested_value,
        ///             },
        ///             "requested_predicates": {
        ///                 "requested_predicate_1_referent": {sub_proof_index: int},
        ///                 "requested_predicate_2_referent": {sub_proof_index: int},
        ///             }
        ///         }
        ///         "proof": {
        ///             "proofs": [ (credential_proof), (credential_proof), (credential_proof) ],
        ///             "aggregated_proof": (aggregated_proof)
        ///         }
        ///         "identifiers": [{schema_id, cred_def_id, Optional rev_reg_id , Optional timestamp}]
        ///     }
        ///
        /// </returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="proofRequest">proof_request_json: proof request json
        ///     {
        ///         "name": string,
        ///         "version": string,
        ///         "nonce": string,
        ///         "requested_attributes": { // set of requested attributes
        ///              "(attr_referent)": (attr_info), // see below
        ///              ...,
        ///         },
        ///         "requested_predicates": { // set of requested predicates
        ///              "(predicate_referent)": (predicate_info), // see below
        ///              ...,
        ///          },
        ///         "non_revoked": Optional [non_revoc_interval], // see below,
        ///                        // If specified prover must proof non-revocation
        ///                        // for date in this interval for each attribute
        ///                        // (can be overridden on attribute level)
        ///     }</param>
        /// <param name="requestedCredentials">
        /// requested_credentials_json: either a credential or self-attested attribute for each requested attribute
        ///     {
        ///         "self_attested_attributes": {
        ///             "self_attested_attribute_referent": string
        ///         },
        ///         "requested_attributes": {
        ///             "requested_attribute_referent_1": {"cred_id": string, "timestamp": Optional number, revealed: bool }},
        ///             "requested_attribute_referent_2": {"cred_id": string, "timestamp": Optional number, revealed: bool }}
        ///         },
        ///         "requested_predicates": {
        ///             "requested_predicates_referent_1": {"cred_id": string, "timestamp": Optional number }},
        ///         }
        ///     }.</param>
        /// <param name="masterSecret">the id of the master secret stored in the wallet</param>
        /// <param name="schemas">
        /// schemas_json: all schemas json participating in the proof request
        ///     {
        ///         [schema1_id]: [schema1_json],
        ///         [schema2_id]: [schema2_json],
        ///         [schema3_id]: [schema3_json],
        ///     }.</param>
        /// <param name="credentialDefs">
        /// credential_defs_json: all credential definitions json participating in the proof request
        ///     {
        ///         "cred_def1_id": (credential_def1_json),
        ///         "cred_def2_id": (credential_def2_json),
        ///         "cred_def3_id": (credential_def3_json),
        ///     }.</param>
        /// <param name="revStates">
        /// rev_states_json: all revocation states json participating in the proof request
        ///     {
        ///         "rev_reg_def1_id": {
        ///             "timestamp1": (rev_state1),
        ///             "timestamp2": (rev_state2),
        ///         },
        ///         "rev_reg_def2_id": {
        ///             "timestamp3": (rev_state3)
        ///         },
        ///         "rev_reg_def3_id": {
        ///             "timestamp4": (rev_state4)
        ///         },
        ///     }.</param>
        public static Task<string> ProverCreateProofAsync(Wallet wallet, string proofRequest, string requestedCredentials, string masterSecret, string schemas, string credentialDefs, string revStates)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(proofRequest, "proofRequest");
            ParamGuard.NotNullOrWhiteSpace(requestedCredentials, "requestedCredentials");
            ParamGuard.NotNullOrWhiteSpace(schemas, "schemas");
            ParamGuard.NotNullOrWhiteSpace(masterSecret, "masterSecret");
            ParamGuard.NotNullOrWhiteSpace(credentialDefs, "credentialDefs");
            ParamGuard.NotNullOrWhiteSpace(revStates, "revStates");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_create_proof(
                commandHandle,
                wallet.Handle,
                proofRequest,
                requestedCredentials,
                masterSecret,
                schemas,
                credentialDefs,
                revStates,
                _proverCreateProofCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Verifies a proof (of multiple credential).
        /// All required schemas, public keys and revocation registries must be provided.
        /// </summary>
        /// <returns>The verify proof async.</returns>
        /// <param name="proofRequest">
        /// proof_request_json: proof request json
        ///     {
        ///         "name": string,
        ///         "version": string,
        ///         "nonce": string,
        ///         "requested_attributes": { // set of requested attributes
        ///              "(attr_referent)": (attr_info), // see below
        ///              ...,
        ///         },
        ///         "requested_predicates": { // set of requested predicates
        ///              "[predicate_referent]": (predicate_info), // see below
        ///              ...,
        ///          },
        ///         "non_revoked": Optional [non_revoc_interval], // see below,
        ///                        // If specified prover must proof non-revocation
        ///                        // for date in this interval for each attribute
        ///                        // (can be overridden on attribute level)
        ///     }</param>
        /// <param name="proof">
        /// proof_json: created for request proof json
        ///     {
        ///         "requested": {
        ///             "revealed_attrs": {
        ///                 "requested_attr1_id": {sub_proof_index: number, raw: string, encoded: string},
        ///                 "requested_attr4_id": {sub_proof_index: number: string, encoded: string},
        ///             },
        ///             "unrevealed_attrs": {
        ///                 "requested_attr3_id": {sub_proof_index: number}
        ///             },
        ///             "self_attested_attrs": {
        ///                 "requested_attr2_id": self_attested_value,
        ///             },
        ///             "requested_predicates": {
        ///                 "requested_predicate_1_referent": {sub_proof_index: int},
        ///                 "requested_predicate_2_referent": {sub_proof_index: int},
        ///             }
        ///         }
        ///         "proof": {
        ///             "proofs": [ &lt;credential_proof>, &lt;credential_proof>, &lt;credential_proof> ],
        ///             "aggregated_proof": &lt;aggregated_proof>
        ///         }
        ///         "identifiers": [{schema_id, cred_def_id, Optional&lt;rev_reg_id>, Optional timestamp }]
        ///     }.</param>
        /// <param name="schemas">
        /// schemas_json: all schema jsons participating in the proof
        ///     {
        ///         &lt;schema1_id>: &lt;schema1_json>,
        ///         &lt;schema2_id>: &lt;schema2_json>,
        ///         &lt;schema3_id>: &lt;schema3_json>,
        ///     }.</param>
        /// <param name="credentialDefs">
        /// credential_defs_json: all credential definitions json participating in the proof
        ///     {
        ///         "cred_def1_id": &lt;credential_def1_json>,
        ///         "cred_def2_id": &lt;credential_def2_json>,
        ///         "cred_def3_id": &lt;credential_def3_json>,
        ///     }</param>
        /// <param name="revocRegDefs">
        /// rev_reg_defs_json: all revocation registry definitions json participating in the proof
        ///     {
        ///         "rev_reg_def1_id": &lt;rev_reg_def1_json>,
        ///         "rev_reg_def2_id": &lt;rev_reg_def2_json>,
        ///         "rev_reg_def3_id": &lt;rev_reg_def3_json>,
        ///     }.</param>
        /// <param name="revocRegs">
        /// rev_regs_json: all revocation registries json participating in the proof
        ///     {
        ///         "rev_reg_def1_id": {
        ///             "timestamp1": &lt;rev_reg1>,
        ///             "timestamp2": &lt;rev_reg2>,
        ///         },
        ///         "rev_reg_def2_id": {
        ///             "timestamp3": &lt;rev_reg3>
        ///         },
        ///         "rev_reg_def3_id": {
        ///             "timestamp4": &lt;rev_reg4>
        ///         },
        ///     }</param>
        public static Task<bool> VerifierVerifyProofAsync(string proofRequest, string proof, string schemas, string credentialDefs, string revocRegDefs, string revocRegs)
        {
            ParamGuard.NotNullOrWhiteSpace(proofRequest, "proofRequest");
            ParamGuard.NotNullOrWhiteSpace(proof, "proof");
            ParamGuard.NotNullOrWhiteSpace(schemas, "schemas");
            ParamGuard.NotNullOrWhiteSpace(credentialDefs, "credentialDefs");
            ParamGuard.NotNullOrWhiteSpace(revocRegDefs, "revocRegDefs");
            ParamGuard.NotNullOrWhiteSpace(revocRegs, "revocRegs");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_verifier_verify_proof(
                commandHandle,
                proofRequest,
                proof,
                schemas,
                credentialDefs,
                revocRegDefs,
                revocRegs,
                _verifierVerifyProofCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Create revocation state for credential in the particular time moment.
        /// </summary>
        /// <returns>
        /// revocation state json:
        ///     {
        ///         "rev_reg": &lt;revocation registry>,
        ///         "witness": &lt;witness>,
        ///         "timestamp" : integer
        ///     }
        /// .</returns>
        /// <param name="blobStorageReader">Configuration of blob storage reader handle that will allow to read revocation tails.</param>
        /// <param name="revRegDef">Revocation registry definition json.</param>
        /// <param name="revRegDelta">Revocation registry definition delta json.</param>
        /// <param name="timestamp">Time represented as a total number of seconds from Unix Epoch.</param>
        /// <param name="credRevId">user credential revocation id in revocation registry.</param>
        public static Task<string> CreateRevocationStateAsync(BlobStorageReader blobStorageReader, string revRegDef, string revRegDelta, long timestamp, string credRevId)
        {
            ParamGuard.NotNullOrWhiteSpace(revRegDef, "revRegDef");
            ParamGuard.NotNullOrWhiteSpace(revRegDelta, "revRegDelta");
            ParamGuard.NotNullOrWhiteSpace(credRevId, "credRevId");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_create_revocation_state(
                commandHandle,
                blobStorageReader.Handle,
                revRegDef,
                revRegDelta,
                timestamp,
                credRevId,
                _createRevocationStateCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Create new revocation state for a credential based on existed state
        /// at the particular time moment (to reduce calculation time).
        /// </summary>
        /// <returns>
        /// revocation state json:
        ///     {
        ///         "rev_reg": &lt;revocation registry>,
        ///         "witness": &lt;witness>,
        ///         "timestamp" : integer
        ///     }
        /// .</returns>
        /// <param name="blobStorageReader">configuration of blob storage reader handle that will allow to read revocation tails.</param>
        /// <param name="revState">revocation registry state json</param>
        /// <param name="revRegDef">revocation registry definition json</param>
        /// <param name="revRegDelta">revocation registry definition delta json.</param>
        /// <param name="timestamp">time represented as a total number of seconds from Unix Epoch.</param>
        /// <param name="credRevId">user credential revocation id in revocation registry.</param>
        public static Task<string> UpdateRevocationStateAsync(BlobStorageReader blobStorageReader, string revState, string revRegDef, string revRegDelta, long timestamp, string credRevId)
        {
            ParamGuard.NotNullOrWhiteSpace(revState, "revState");
            ParamGuard.NotNullOrWhiteSpace(revRegDef, "revRegDef");
            ParamGuard.NotNullOrWhiteSpace(revRegDelta, "revRegDelta");
            ParamGuard.NotNullOrWhiteSpace(credRevId, "credRevId");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_update_revocation_state(
                commandHandle,
                blobStorageReader.Handle,
                revState,
                revRegDef,
                revRegDelta,
                timestamp,
                credRevId,
                _updateRevocationStateCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}