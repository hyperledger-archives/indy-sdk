using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
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
        private static IssuerCreateAndStoreClaimDefCompletedDelegate _issuerCreateAndStoreClaimDefCallback = (xcommand_handle, err, claim_def_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claim_def_json);
        };

        /// <summary>
        /// Gets the callback to use when the IssuerCreateAndStoreClaimRevocRegAsync command completes.
        /// </summary>
        private static IssuerCreateAndStoreClaimRevocRegCompletedDelegate _issuerCreateAndStoreClaimRevocRegCallback = (xcommand_handle, err, revoc_reg_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(revoc_reg_json);
        };

        /// <summary>
        /// Gets the callback to use when the IssuerCreateClaimAsync command completes.
        /// </summary>
        private static IssuerCreateClaimOfferCompletedDelegate _issuerCreateClaimOfferCallback = (xcommand_handle, err, claim_offer_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claim_offer_json);
        };

        /// <summary>
        /// Gets the callback to use when the IssuerCreateClaimAsync command completes.
        /// </summary>
        private static IssuerCreateClaimCompletedDelegate _issuerCreateClaimCallback = (xcommand_handle, err, revoc_reg_update_json, claim_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<IssuerCreateClaimResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var callbackResult = new IssuerCreateClaimResult(revoc_reg_update_json, claim_json);

            taskCompletionSource.SetResult(callbackResult);
        };


        /// <summary>
        /// Gets the callback to use when the IssuerRevokeClaimAsync command completes.
        /// </summary>
        private static IssuerRevokeClaimCompletedDelegate IssuerRevokeClaimCallback = (xcommand_handle, err, revoc_reg_update_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(revoc_reg_update_json);
        };

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimOffersAsync command completes.
        /// </summary>
        private static ProverGetClaimOffersCompletedDelegate _proverGetClaimOffersCallback = (xcommand_handle, err, claim_offer_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claim_offer_json);
        };

        /// <summary>
        /// Gets the callback to use when the roverCreateAndStoreClaimReqAsync command completes.
        /// </summary>
        private static ProverCreateAndStoreClaimReqCompletedDelegate _proverCreateAndStoreClaimReqCallback = (xcommand_handle, err, claim_req_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claim_req_json);
        };

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimsAsync command completes.
        /// </summary>
        private static ProverGetClaimsCompletedDelegate _proverGetClaimsCallback = (xcommand_handle, err, claims_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(claims_json);
        };

        /// <summary>
        /// Gets the callback to use when the ProverGetClaimsForProofAsync command completes.
        /// </summary>
        private static ProverGetClaimsForProofCompletedDelegate _proverGetClaimsForProofCallback = (xcommand_handle, err, claims_json) =>
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
        /// <param name="signatureType">The type of signature to use.</param>
        /// <param name="createNonRevoc">Whether to request non-revocation claim.</param>
        /// <returns>
        /// An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves to a 
        /// JSON string containing the claim definition.</returns>
        public static Task<string> IssuerCreateAndStoreClaimDefAsync(Wallet wallet, string issuerDid, string schemaJson, string signatureType, bool createNonRevoc)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(issuerDid, "issuerDid");
            ParamGuard.NotNullOrWhiteSpace(schemaJson, "schemaJson");
            
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_create_and_store_claim_def(
                commandHandle,
                wallet.Handle,
                issuerDid,
                schemaJson,
                signatureType,
                createNonRevoc,
                _issuerCreateAndStoreClaimDefCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a new revocation registry for the provided claim definition.
        /// </summary>
        /// <remarks>
        /// The revocation registry is stored in the provided <paramref name="wallet"/> and is identified by
        /// a unique key which is returned in the revocation registry JSON string returned by the method.
        /// </remarks>
        /// <param name="wallet">The wallet to store the revocation registry in.</param>
        /// <param name="issuerDid">The DID of the issuer that signed the revoc_reg transaction to the ledger.</param>
        /// <param name="schemaSeqNo">The sequence number of a schema transaction in the ledger.</param>
        /// <param name="maxClaimNum">The maximum number of claims the new registry can process.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves 
        /// to a JSON string containing the revocation registry.</returns>
        public static Task<string> IssuerCreateAndStoreRevocRegAsync(Wallet wallet, string issuerDid, int schemaSeqNo, int maxClaimNum)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(issuerDid, "issuerDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_create_and_store_revoc_reg(
                commandHandle,
                wallet.Handle,
                issuerDid,
                schemaSeqNo,
                maxClaimNum,
                _issuerCreateAndStoreClaimRevocRegCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// TODO: Issuers the create claim offer async.
        /// </summary>
        /// <returns>The create claim offer async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="schemaJson">Schema json.</param>
        /// <param name="issuerDid">Issuer did.</param>
        /// <param name="proverDid">Prover did.</param>
        public static Task<string> IssuerCreateClaimOfferAsync(Wallet wallet, string schemaJson, string issuerDid, string proverDid)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(schemaJson, "schemaJson");
            ParamGuard.NotNullOrWhiteSpace(issuerDid, "issuerDid");
            ParamGuard.NotNullOrWhiteSpace(proverDid, "proverDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_create_claim_offer(
                commandHandle,
                wallet.Handle,
                schemaJson,
                issuerDid,
                proverDid,
                _issuerCreateClaimOfferCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Signs the provided claim for using the key provided in the specified claim request.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The <paramref name="claimReqJson"/> parameter must be passed a claim request that was previously
        /// created using the <see cref="ProverCreateAndStoreClaimReqAsync(Wallet, string, string, string, string)"/>
        /// method.  Usually the claim request will be received from another party that has performed this 
        /// action.
        /// </para>
        /// <para>
        /// The claim to be signed is provided in the <paramref name="claimJson"/> parameter 
        /// and the structure of the claim must conform to the schema from claim request provided in 
        /// the <paramref name="claimReqJson"/> parameter.  Claims must be structured as a series of
        /// attributes, each of which has two values; a human readable value and a hex encoded value.  
        /// <code>
        /// {
        ///      "attr1" : ["value1", "value1_as_int"],
        ///      "attr2" : ["value2", "value2_as_int"]
        /// }
        /// </code>
        /// For example:
        /// <code>
        /// {
        ///     'name': ['Alex', '1139481716457488690172217916278103335'],
        ///     'height': ['175', '175']
        /// }
        /// </code>
        /// </para>
        /// <para>
        /// This method results a revocation registry update JSON and a newly issued claim JSON.  The
        /// claim JSON contains the issued claim, the DID of the issuer (<c>issuer_did</c>), 
        /// schema sequence number (<c>schema_seq_no</c>) and revocation registry sequence number (<c>
        /// revoc_reg_seq_no</c>) used for issuance:
        /// <code>
        /// {
        ///     "claim": &lt;see claim_json above&gt;,
        ///     "signature": &lt;signature&gt;,
        ///     "revoc_reg_seq_no", string,
        ///     "issuer_did", string,
        ///     "schema_seq_no", string,
        /// }
        /// </code>
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet containing the keys to use for signing the claim.</param>
        /// <param name="claimReqJson">A claim request with a blinded secret.</param>
        /// <param name="claimJson">A claim containing attribute values for each of requested attribute names.</param>
        /// <param name="userRevocIndex">The index of a new user in the revocation registry or -1 if absentee.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves to 
        /// an <see cref="IssuerCreateClaimResult"/>.</returns>
        public static Task<IssuerCreateClaimResult> IssuerCreateClaimAsync(Wallet wallet, string claimReqJson, string claimJson, int userRevocIndex)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(claimReqJson, "claimReqJson");
            ParamGuard.NotNullOrWhiteSpace(claimJson, "claimJson");

            var taskCompletionSource = new TaskCompletionSource<IssuerCreateClaimResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_create_claim(
                commandHandle,
                wallet.Handle,
                claimReqJson,
                claimJson,
                userRevocIndex,
                _issuerCreateClaimCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Revokes a user identified by a revoc_id in a given revoc-registry.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The corresponding claim definition and revocation registry must be already
        /// have been created and stored in the wallet.
        /// </para>
        /// </remarks>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="issuerDid">The DID of the issuer.</param>
        /// <param name="schemaSequenceNumber">The sequence number of the schema.</param>
        /// <param name="userRevocIndex">index of the user in the revocation registry</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves 
        /// to a revocation registry update JSON with a revoked claim.</returns>
        public static Task<string> IssuerRevokeClaimAsync(Wallet wallet, string issuerDid, int schemaSequenceNumber, int userRevocIndex)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(issuerDid, "issuerDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_issuer_revoke_claim(
                commandHandle,
                wallet.Handle,
                issuerDid,
                schemaSequenceNumber,
                userRevocIndex,
                IssuerRevokeClaimCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Stores a claim offer.
        /// </summary>
        /// <remarks>
        /// Stores a claim offer from the issuer specified in the <paramref name="claimOfferJson"/>
        /// into the provided <paramref name="wallet"/>.  The expected structure of the claim offer
        /// is as follows:
        /// <code>
        /// {
        ///     "issuer_did": string,
        ///     "schema_seq_no": string
        /// }
        /// </code>
        /// </remarks>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="claimOfferJson">The claim offer JSON</param>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation has completed.</returns>
        public static Task ProverStoreClaimOfferAsync(Wallet wallet, string claimOfferJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(claimOfferJson, "claimOfferJson");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_store_claim_offer(
                commandHandle,
                wallet.Handle,
                claimOfferJson,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets all claim offers in the provided wallet matching the specified filter.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Claim offers stored with the <see cref="ProverStoreClaimOfferAsync(Wallet, string)"/> can be
        /// retrieved from the <paramref name="wallet"/> by searching on the DID of the issuer and/or the schema 
        /// sequence number.  To filter the claim offers a <paramref name="filterJson"/> parameter must be provided with 
        /// a JSON string which can include the following members:
        /// <code>
        /// {
        ///     "issuer_did": string,
        ///     "schema_seq_no": string
        /// }
        /// </code>
        /// </para>
        /// <para>
        /// The return value from this method is a JSON string that contains the list of matching claim 
        /// offers in the following format:
        /// <code>
        /// {
        ///     [
        ///         {
        ///             "issuer_did": string,
        ///             "schema_seq_no": string
        ///         },
        ///         ...
        ///     ]
        /// }
        /// </code>
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet containing the claims to get.</param>
        /// <param name="filterJson">The filter JSON.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves 
        /// to a JSON string with a list of claim offers matching the filter.</returns>
        public static Task<string> ProverGetClaimOffersAsync(Wallet wallet, string filterJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(filterJson, "filterJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_get_claim_offers(
                commandHandle,
                wallet.Handle,
                filterJson,
                _proverGetClaimOffersCallback
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
        /// <param name="masterSecretName">The name of the master secret.</param>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation has completed.</returns>
        public static Task ProverCreateMasterSecretAsync(Wallet wallet, string masterSecretName)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(masterSecretName, "masterSecretName");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_create_master_secret(
                commandHandle,
                wallet.Handle,
                masterSecretName,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a claim request for the specified claim offer and stores it in the provided wallet.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The JSON of a claim definition that is associated with the issuer_did and schema_seq_no in the 
        /// claim_offer must be provided in the <paramref name="claimDefJson"/> parameter.  Claim 
        /// definitions can be retrieved from the ledger using the 
        /// <see cref="Ledger.BuildGetClaimDefTxnAsync(string, int, string, string)"/>method.
        /// </para>
        /// <para>
        /// The JSON in the <paramref name="claimOfferJson"/> parameter contains information about the 
        /// issuer of the claim offer:
        /// <code>
        /// {
        ///     "issuer_did": string,
        ///     "schema_seq_no": string
        /// }
        /// </code>
        /// This method gets the public key and schema the <c>issuer_did</c> from the ledger for and 
        /// stores them in the provided wallet. Once this is complete a blinded master secret is for the 
        /// master secret specified by the <paramref name="masterSecretName"/> parameter.  
        /// <note type="note">
        /// The master secret identified by the name must be already stored in the secure wallet using the
        /// <see cref="ProverCreateMasterSecretAsync(Wallet, string)"/> method.
        /// </note>
        /// The blinded master secret becomes a part of the claim request.
        /// </para>
        /// </remarks>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="proverDid">The DID of the prover.</param>
        /// <param name="claimOfferJson">The claim offer JSON to generate a claim request for.</param>
        /// <param name="claimDefJson">The claim definition JSON.</param>
        /// <param name="masterSecretName">The name of the master secret in the wallet to use for generating the blinded secret.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves 
        /// to a JSON string containing the claim request.</returns>
        public static Task<string> ProverCreateAndStoreClaimReqAsync(Wallet wallet, string proverDid, string claimOfferJson, string claimDefJson, string masterSecretName)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(proverDid, "proverDid");
            ParamGuard.NotNullOrWhiteSpace(claimOfferJson, "claimOfferJson");
            ParamGuard.NotNullOrWhiteSpace(claimDefJson, "claimDefJson");
            ParamGuard.NotNullOrWhiteSpace(masterSecretName, "masterSecretName");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_create_and_store_claim_req(
                commandHandle,
                wallet.Handle,
                proverDid,
                claimOfferJson,
                claimDefJson,
                masterSecretName,
                _proverCreateAndStoreClaimReqCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Updates and stores the provided claim in the specified wallet.
        /// </summary>
        /// <remarks>
        /// <para>
        /// This method updates the claim provided in the <paramref name="claimsJson"/> parameter
        /// with a blinded master secret and stores it in the wallet specified in the 
        /// <paramref name="wallet"/> parameter. 
        /// </para>
        /// <para>
        /// The claim JSON is typically structured as follows:
        /// <code>
        /// {
        ///     "claim": {attr1:[value, value_as_int]}
        ///     "signature": &lt;signature&gt;,
        ///     "schema_seq_no": string,
        ///     "revoc_reg_seq_no", string
        ///     "issuer_did", string
        /// }
        /// </code>
        /// It contains the information about the <c>schema_seq_no</c>, <c>issuer_did</c> 
        /// and <c>revoc_reg_seq_no</c> - see the <see cref="IssuerCreateClaimAsync(Wallet, string, string, int)"/>
        /// method for details.
        /// </para>
        /// Seq_no is a sequence number of the corresponding transaction in the ledger.
        /// </remarks>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="claimsJson">The claims JSON.</param>
        /// <param name="revRegJson">revocation registry json</param>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation has completed.</returns>
        public static Task ProverStoreClaimAsync(Wallet wallet, string claimsJson, string revRegJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(claimsJson, "claimsJson");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_store_claim(
                commandHandle,
                wallet.Handle,
                claimsJson,
                revRegJson,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets claims matching the provided filter from the specified wallet.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Claims can be filtered by Issuer, claim_def and/or Schema. To filter the results set the
        /// <paramref name="filterJson"/> parameter with a JSON string that conforms to the following 
        /// format:
        /// <code>
        /// {
        ///     "issuer_did": string,
        ///     "schema_seq_no": string
        /// }
        /// </code>
        /// If <paramref name="filterJson"/> is null then all claims will be returned.
        /// </para>
        /// <para>
        /// Upon successful completion this method will return a JSON string containing an array of
        /// claims:
        /// <code>
        /// [
        ///     {
        ///         "referent": string,
        ///         "attrs": [{"attr_name" : "attr_value"}, ...],
        ///         "schema_seq_no": string,
        ///         "issuer_did": string,
        ///         "revoc_reg_seq_no": string,
        ///     },
        ///     ...
        /// ]
        /// </code>
        /// </para>
        /// </remarks>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="filterJson">The filter JSON.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves 
        /// to a JSON string containing the claim request.</returns>
        public static Task<string> ProverGetClaimsAsync(Wallet wallet, string filterJson)
        {
            ParamGuard.NotNull(wallet, "wallet");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_get_claims(
                commandHandle,
                wallet.Handle,
                filterJson,
                _proverGetClaimsCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets claims matching the provided proof request from the specified wallet.
        /// </summary>
        /// <remarks>
        /// The proof request provided in the <paramref name="proofRequestJson"/> parameter must conform 
        /// to the format:
        /// <code>
        /// {
        ///     "name": string,
        ///     "version": string,
        ///     "nonce": string,
        ///     "requested_attr1_referent": &lt;attr_info&gt;,
        ///     "requested_attr2_referent": &lt;attr_info&gt;,
        ///     "requested_attr3_referent": &lt;attr_info&gt;,
        ///     "requested_predicate_1_referent": &lt;predicate_info&gt;,
        ///     "requested_predicate_2_referent": &lt;predicate_info&gt;,
        /// }
        /// </code>
        /// The method will return a JSON string with claims matching the given proof request in the following format:
        /// <code>
        /// {
        ///     "requested_attr1_referent": [claim1, claim2],
        ///     "requested_attr2_referent": [],
        ///     "requested_attr3_referent": [claim3],
        ///     "requested_predicate_1_referent": [claim1, claim3],
        ///     "requested_predicate_2_referent": [claim2],
        /// }
        /// </code>
        /// Each claim in the result consists of a uuid (<c>referent</c>), human-readable attributes as
        /// a key-value map (<c>attrs</c>), a schema sequence number (<c>schema_seq_no</c>) an issuer DID
        /// (<c>issuer_did</c>) and a revocation registry sequence number (<c>revoc_reg_seq_no</c>):
        /// <code>
        /// {
        ///     "referent": string,
        ///     "attrs": [{"attr_name" : "attr_value"}],
        ///     "schema_seq_no": string,
        ///     "issuer_did": string,
        ///     "revoc_reg_seq_no": string,
        /// }
        /// </code>
        /// </remarks>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="proofRequestJson">The proof request JSON.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves 
        /// to a JSON string containing the claims for the proof request.</returns>
        public static Task<string> ProverGetClaimsForProofReqAsync(Wallet wallet, string proofRequestJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(proofRequestJson, "proofRequestJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_get_claims_for_proof_req(
                commandHandle,
                wallet.Handle,
                proofRequestJson,
                _proverGetClaimsForProofCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a proof for the provided proof request.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Either a corresponding claim with optionally revealed attributes or self-attested attribute 
        /// must be provided for each requested attribute - see the 
        /// <see cref="ProverGetClaimsForProofReqAsync(Wallet, string)"/> method.
        /// A proof request may request multiple claims from different schema and different issuers.
        /// All required schema, public keys and revocation registries must be provided.
        /// The proof request also contains a nonce.
        /// The proof contains either proof or self-attested attribute value for each requested attribute.
        /// </para>
        /// <para>
        /// The <paramref name="proofReqJson"/> parameter expects a JSON string containing a proof request
        /// from the party that will verify the proof.  E.g.:
        /// <code>
        ///  {
        ///     "nonce": string,
        ///     "requested_attr1_referent": &lt;attr_info&gt;,
        ///     "requested_attr2_referent": &lt;attr_info&gt;,
        ///     "requested_attr3_referent": &lt;attr_info&gt;,
        ///     "requested_predicate_1_referent": &lt;predicate_info&gt;,
        ///     "requested_predicate_2_referent": &lt;predicate_info&gt;,
        /// }
        /// </code>
        /// </para>
        /// <para>
        /// The <paramref name="requestedClaimsJson"/> parameter should contain either a claim or a 
        /// self-attested attribute for each attribute requested in the proof request.  E.g.:
        /// <code>
        /// {
        ///     "requested_attr1_referent": [claim1_referent_in_wallet, true &lt;reveal_attr&gt;],
        ///     "requested_attr2_referent": [self_attested_attribute],
        ///     "requested_attr3_referent": [claim2_seq_no_in_wallet, false]
        ///     "requested_attr4_referent": [claim2_seq_no_in_wallet, true]
        ///     "requested_predicate_1_referent": [claim2_seq_no_in_wallet],
        ///     "requested_predicate_2_referent": [claim3_seq_no_in_wallet],
        /// }
        /// </code>
        /// </para>
        /// <para>
        /// The <paramref name="schemasJson"/> parameter expects the JSON for each schema that participates
        /// in the proof request.  E.g.:
        /// <code>
        /// {
        ///     "claim1_referent_in_wallet": &lt;schema1&gt;,
        ///     "claim2_referent_in_wallet": &lt;schema2&gt;,
        ///     "claim3_referent_in_wallet": &lt;schema3&gt;,
        /// }
        /// </code>
        /// </para>
        /// <para>
        /// The <paramref name="masterSecretName"/> specifies the name of the master secret stored in 
        /// the wallet.
        /// </para>
        /// <para>
        /// The <paramref name="claimDefsJson"/> parameter expects the JSON for each claim definition 
        /// participating in the proof request. E.g.:
        /// <code>
        /// {
        ///     "claim1_referent_in_wallet": &lt;claim_def1&gt;,
        ///     "claim2_referent_in_wallet": &lt;claim_def2&gt;,
        ///     "claim3_referent_in_wallet": &lt;claim_def3&gt;,
        /// }
        /// </code>
        /// </para>
        /// <para>
        /// The <paramref name="revocRegsJson"/> parameter expects the JSON for each revocation registry
        /// participating in the proof request.  E.g.:
        /// <code>
        /// {
        ///     "claim1_referent_in_wallet": &lt;revoc_reg1&gt;,
        ///     "claim2_referent_in_wallet": &lt;revoc_reg2&gt;,
        ///     "claim3_referent_in_wallet": &lt;revoc_reg3&gt;,
        /// }
        /// </code>
        /// </para>
        /// Upon successful completion the operation will return a JSON string.
        /// For each requested attribute either a proof (with optionally revealed attribute value) or
        /// self-attested attribute value is provided.
        /// Each proof is associated with a claim and corresponding schema_seq_no, issuer_did and revoc_reg_seq_no.
        /// There is also aggregated proof part common for all claim proofs.
        /// <code>
        /// {
        ///     "requested": {
        ///         "requested_attr1_id": [claim_proof1_referent, revealed_attr1, revealed_attr1_as_int],
        ///         "requested_attr2_id": [self_attested_attribute],
        ///         "requested_attr3_id": [claim_proof2_referent]
        ///         "requested_attr4_id": [claim_proof2_referent, revealed_attr4, revealed_attr4_as_int],
        ///         "requested_predicate_1_referent": [claim_proof2_referent],
        ///         "requested_predicate_2_referent": [claim_proof3_referent],
        ///         }
        ///     "claim_proofs": {
        ///         "claim_proof1_referent": [&lt;claim_proof&gt;, issuer_did, schema_seq_no, revoc_reg_seq_no],
        ///         "claim_proof2_referent": [&lt;claim_proof&gt;, issuer_did, schema_seq_no, revoc_reg_seq_no],
        ///         "claim_proof3_referent": [&lt;claim_proof&gt;, issuer_did, schema_seq_no, revoc_reg_seq_no]
        ///     },
        ///     "aggregated_proof": &lt;aggregated_proof&gt;
        /// }
        /// </code>
        /// </remarks>
        /// <param name="wallet">The target wallet.</param>
        /// <param name="proofReqJson">The proof request JSON.</param>
        /// <param name="requestedClaimsJson">The requested claims JSON.</param>
        /// <param name="schemasJson">The schema JSON.</param>
        /// <param name="masterSecretName">The master secret name.</param>
        /// <param name="claimDefsJson">The claim definitions JSON.</param>
        /// <param name="revocRegsJson">The revocation registries JSON.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves 
        /// to a JSON string containing the proof.</returns>
        public static Task<string> ProverCreateProofAsync(Wallet wallet, string proofReqJson, string requestedClaimsJson, string schemasJson, string masterSecretName, string claimDefsJson, string revocRegsJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(proofReqJson, "proofReqJson");
            ParamGuard.NotNullOrWhiteSpace(requestedClaimsJson, "requestedClaimsJson");
            ParamGuard.NotNullOrWhiteSpace(schemasJson, "schemasJson");
            ParamGuard.NotNullOrWhiteSpace(masterSecretName, "masterSecretName");
            ParamGuard.NotNullOrWhiteSpace(claimDefsJson, "claimDefsJson");
            ParamGuard.NotNullOrWhiteSpace(revocRegsJson, "revocRegsJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_prover_create_proof(
                commandHandle,
                wallet.Handle,
                proofReqJson,
                requestedClaimsJson,
                schemasJson,
                masterSecretName,
                claimDefsJson,
                revocRegsJson,
                _proverCreateProofCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Verifies whether or not a proof is valid.
        /// </summary>
        /// <remarks>
        /// <para>
        /// This method verifies a proof that can be made up of multiple claims.
        /// All required schema, public keys and revocation registries must be provided.
        /// </para>
        /// <para>
        /// The <paramref name="proofRequestJson"/> parameter expects the initial proof request sent
        /// by the verifier.
        /// <code>
        /// {
        ///     "nonce": string,
        ///     "requested_attr1_referent": &lt;attr_info&gt;,
        ///     "requested_attr2_referent": &lt;attr_info&gt;,
        ///     "requested_attr3_referent": &lt;attr_info&gt;,
        ///     "requested_predicate_1_referent": &lt;predicate_info&gt;,
        ///     "requested_predicate_2_referent": &lt;predicate_info&gt;,
        /// }
        /// </code>
        /// </para>
        /// <para>
        /// The <paramref name="proofJson"/> parameter expects a JSON string containing,  
        /// for each requested attribute,  either a proof (with optionally revealed attribute value) or
        /// self-attested attribute value.  Each proof is associated with a claim and corresponding 
        /// schema_seq_no, issuer_did and revoc_reg_seq_no. There is also aggregated proof part 
        /// common for all claim proofs.
        /// <code>
        /// {
        ///     "requested": {
        ///         "requested_attr1_id": [claim_proof1_referent, revealed_attr1, revealed_attr1_as_int],
        ///         "requested_attr2_id": [self_attested_attribute],
        ///         "requested_attr3_id": [claim_proof2_referent]
        ///         "requested_attr4_id": [claim_proof2_referent, revealed_attr4, revealed_attr4_as_int],
        ///         "requested_predicate_1_referent": [claim_proof2_referent],
        ///         "requested_predicate_2_referent": [claim_proof3_referent],
        ///     },
        ///     "claim_proofs": {
        ///         "claim_proof1_referent": [&lt;claim_proof&gt;, issuer_did, schema_seq_no, revoc_reg_seq_no],
        ///         "claim_proof2_referent": [&lt;claim_proof&gt;, issuer_did, schema_seq_no, revoc_reg_seq_no],
        ///         "claim_proof3_referent": [&lt;claim_proof&gt;, issuer_did, schema_seq_no, revoc_reg_seq_no]
        ///     },
        ///     "aggregated_proof": &lt;aggregated_proof&gt;
        /// }
        /// </code>
        /// </para>
        /// <para>
        /// The <paramref name="schemasJson"/> parameter expects a JSON string containing all schema
        /// participating in the proof.
        /// <code>
        /// {
        ///     "claim_proof1_referent": &lt;schema&gt;,
        ///     "claim_proof2_referent": &lt;schema&gt;,
        ///     "claim_proof3_referent": &lt;schema&gt;
        /// }
        /// </code>
        /// </para> 
        /// <para>
        /// The <paramref name="claimDefsJson"/> parameter expects a JSON string containing all claim
        /// definitions participating in the proof.
        /// <code>
        /// {
        ///     "claim_proof1_referent": &lt;claim_def&gt;,
        ///     "claim_proof2_referent": &lt;claim_def&gt;,
        ///     "claim_proof3_referent": &lt;claim_def&gt;
        /// }
        /// </code>
        /// </para>
        /// <para>
        /// The <paramref name="revocRegsJson"/> parameter expects a JSON string containing all revocation
        /// registries participating in the proof.
        /// <code>
        /// {
        ///     "claim_proof1_referent": &lt;revoc_reg&gt;,
        ///     "claim_proof2_referent": &lt;revoc_reg&gt;,
        ///     "claim_proof3_referent": &lt;revoc_reg&gt;
        /// }
        /// </code>
        /// </para>
        /// </remarks>
        /// <param name="proofRequestJson">The proof request JSON.</param>
        /// <param name="proofJson">The proof JSON.</param>
        /// <param name="schemasJson">The schemas JSON.</param>
        /// <param name="claimDefsJson">The claim definitions JSON.</param>
        /// <param name="revocRegsJson">The revocation registries JSON.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves 
        /// to true if the proof is valid, otherwise false.</returns>
        public static Task<bool> VerifierVerifyProofAsync(string proofRequestJson, string proofJson, string schemasJson, string claimDefsJson, string revocRegsJson)
        {
            ParamGuard.NotNullOrWhiteSpace(proofRequestJson, "proofRequestJson");
            ParamGuard.NotNullOrWhiteSpace(proofJson, "proofJson");
            ParamGuard.NotNullOrWhiteSpace(schemasJson, "schemasJson");
            ParamGuard.NotNullOrWhiteSpace(claimDefsJson, "claimDefsJson");
            ParamGuard.NotNullOrWhiteSpace(revocRegsJson, "revocRegsJson");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_verifier_verify_proof(
                commandHandle,
                proofRequestJson,
                proofJson,
                schemasJson,
                claimDefsJson,
                revocRegsJson,
                _verifierVerifyProofCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}