using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System.Threading.Tasks;
using static Hyperledger.Indy.DidApi.NativeMethods;
#if __IOS__
using ObjCRuntime;
#endif

namespace Hyperledger.Indy.DidApi
{
    /// <summary>
    /// Provides cryptographic functionality related to DIDs.
    /// </summary>
    public static class Did
    {
        /// <summary>
        /// Gets the callback to use when the command for CreateAndStoreMyDidResultAsync has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(CreateAndStoreMyDidCompletedDelegate))]
#endif
        private static void CreateAndStoreMyDidCallbackMethod(int xcommand_handle, int err, string did, string verkey)
        {
            var taskCompletionSource = PendingCommands.Remove<CreateAndStoreMyDidResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var callbackResult = new CreateAndStoreMyDidResult(did, verkey);

            taskCompletionSource.SetResult(callbackResult);
        }
        private static CreateAndStoreMyDidCompletedDelegate CreateAndStoreMyDidCallback = CreateAndStoreMyDidCallbackMethod;

        /// <summary>
        /// Gets the callback to use when the command for ReplaceKeysAsync has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(ReplaceKeysStartCompletedDelegate))]
#endif
        private static void ReplaceKeysCallbackMethod(int xcommand_handle, int err, string verkey)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(verkey);
        }
        private static ReplaceKeysStartCompletedDelegate ReplaceKeysCallback = ReplaceKeysCallbackMethod;

        /// <summary>
        /// Gets the callback to use when the command for KeyForDidAsync has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(DidKeyForDidCompletedDelegate))]
#endif
        private static void KeyForDidCompletedCallbackMethod(int xcommand_handle, int err, string key)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(key);
        }
        private static DidKeyForDidCompletedDelegate KeyForDidCompletedCallback = KeyForDidCompletedCallbackMethod;

        /// <summary>
        /// Gets the callback to use when the command for KeyForLocalDidAsync has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(DidKeyForLocalDidCompletedDelegate))]
#endif
        private static void KeyForLocalDidCompletedCallbackMethod(int xcommand_handle, int err, string key)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(key);
        }
        private static DidKeyForLocalDidCompletedDelegate KeyForLocalDidCompletedCallback = KeyForLocalDidCompletedCallbackMethod;

        /// <summary>
        /// Gets the callback to use when the command for GetEndpointForDidAsync has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(DidGetEndpointForDidCompletedDelegate))]
#endif
        private static void GetEndpointForDidCompletedCallbackMethod(int xcommand_handle, int err, string endpoint, string transport_vk)
        {
            var taskCompletionSource = PendingCommands.Remove<EndpointForDidResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var result = new EndpointForDidResult(endpoint, transport_vk);

            taskCompletionSource.SetResult(result);
        }
        private static DidGetEndpointForDidCompletedDelegate GetEndpointForDidCompletedCallback = GetEndpointForDidCompletedCallbackMethod;

        /// <summary>
        /// Gets the callback to use when the command for GetDidMetadataAsync has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(DidGetDidMetadataCompletedDelegate))]
#endif
        private static void GetDidMetadataCompletedCallbackMethod(int xcommand_handle, int err, string metadata)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(metadata);
        }
        private static DidGetDidMetadataCompletedDelegate GetDidMetadataCompletedCallback = GetDidMetadataCompletedCallbackMethod;

        /// <summary>
        /// Gets the callback to use when the command for GetMyDidWithMetaAsync has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(GetMyDidWithMetaCompletedDelegate))]
#endif
        private static void GetMyDidWithMetaCompletedCallbackMethod(int xcommand_handle, int err, string didWithMeta)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(didWithMeta);
        }
        private static GetMyDidWithMetaCompletedDelegate GetMyDidWithMetaCompletedCallback = GetMyDidWithMetaCompletedCallbackMethod;

        /// <summary>
        /// Gets the callback to use when the command for GetMyDidWithMetaAsync has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(ListMyDidsWithMetaCompletedDelegate))]
#endif
        private static void ListMyDidsWithMetaCompletedCallbackMethod(int xcommand_handle, int err, string dids)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(dids);
        }
        private static ListMyDidsWithMetaCompletedDelegate ListMyDidsWithMetaCompletedCallback = ListMyDidsWithMetaCompletedCallbackMethod;

        /// <summary>
        /// Gets the callback to use when the command for AbbreviateVerkeyAsync has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(AbbreviateVerkeyCompletedDelegate))]
#endif
        private static void AbbreviateVerkeyCompletedCallbackMethod(int xcommand_handle, int err, string verkey)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(verkey);
        }
        private static AbbreviateVerkeyCompletedDelegate AbbreviateVerkeyCompletedCallback = AbbreviateVerkeyCompletedCallbackMethod;

        /// <summary>
        /// Creates signing and encryption keys in specified wallet for a new DID owned by the caller.
        /// </summary>
        /// <remarks>
        /// <para>Saves the identity DID with keys in a wallet so that it can be used to sign
        /// and encrypt transactions.  Control over the created DID is provided through the 
        /// <paramref name="didJson"/> parameter which accepts a JSON string with the following
        /// optional parameters:
        /// </para>
        /// <code>
        /// {
        ///     "did": string,
        ///     "seed": string, 
        ///     "crypto_type": string, 
        ///     "cid": bool
        /// }
        /// </code>
        /// <para>The <c>did</c> member specifies the DID of the new entry.  If not 
        /// provided and the <c>cid</c> member is <c>false</c> then the first 16 bits of the VerKey value 
        /// generated will be used as a new DID.  If not provided and the <c>cid</c> member is <c>true</c> then the full 
        /// VerKey value will be used as a new DID.  If the <c>did</c> member is provided then the keys will be 
        /// replaced - this is normally used in the case of key rotation.</para>
        /// <para>The <c>seed</c> member specifies the seed to use when generating keys.  If not provided 
        /// then a random seed value will be created.</para>
        /// <para>The <c>crypto_type</c> member specifies the cryptographic algorithm used for generating
        /// keys.  If not provided then ed25519 curve is used.
        /// <note type="note">The only value currently supported for this member is 'ed25519'.</note>
        /// </para>
        /// <para>The <c>cid</c> member indicates whether the DID should be used in creating the DID.
        /// If not provided then the value defaults to false.</para>
        /// </remarks>
        /// <param name="wallet">The wallet to store the DID in.</param>
        /// <param name="didJson">The DID JSON.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="CreateAndStoreMyDidResult"/> when the operation completes.</returns>
        public static Task<CreateAndStoreMyDidResult> CreateAndStoreMyDidAsync(Wallet wallet, string didJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(didJson, "didJson");

            var taskCompletionSource = new TaskCompletionSource<CreateAndStoreMyDidResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_create_and_store_my_did(
                commandHandle,
                wallet.Handle,
                didJson,
                CreateAndStoreMyDidCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Generates new signing and encryption keys in the specified wallet for an existing DID owned by the caller
        /// </summary>
        /// <remarks>
        /// The developer has some control over the generation of the new keys through the value passed to
        /// the <paramref name="identityJson"/> parameter.  This parameter expects a valid JSON string
        /// with the following optional members:
        /// <code>
        /// {
        ///     "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
        ///                                Can be UTF-8, base64 or hex string.
        ///     "crypto_type": string, (optional) if not set then ed25519 curve is used;
        ///               currently only 'ed25519' value is supported for this field)
        /// }
        /// </code>
        /// <para>The <c>seed</c> member controls the seed that will be used to generate they keys.
        /// If not provided a random one will be created.</para>
        /// <para>The <c>crypto_type</c> member specifies the type of cryptographic algorithm will be 
        /// used to generate they keys.  If not provided then ed22519 curve will be used.
        /// <note type="note">The only value currently supported for this member is 'ed25519'.</note>
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet the DID is stored in.</param>
        /// <param name="did">The did to replace the keys for.</param>
        /// <param name="identityJson">The identity information as JSON.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the new verification key when the operation completes.</returns>
        public static Task<string> ReplaceKeysStartAsync(Wallet wallet, string did, string identityJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNullOrWhiteSpace(identityJson, "identityJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_replace_keys_start(
                commandHandle,
                wallet.Handle,
                did,
                identityJson,
                ReplaceKeysCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Applies temporary signing and encryption keys as main in the specified wallet for an existing DID owned by the caller
        /// </summary>
        /// <param name="wallet">The wallet the DID is stored in.</param>
        /// <param name="did">The did to replace the keys for.</param>
        /// <returns>An asynchronous <see cref="Task"/> that  with no return value the completes when the operation completes.</returns>
        public static Task ReplaceKeysApplyAsync(Wallet wallet, string did)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_replace_keys_apply(
                commandHandle,
                wallet.Handle,
                did,
                CallbackHelper.TaskCompletingNoValueCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Stores a remote party's DID for a pairwise connection in the specified wallet.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The DID and optional associated parameters must be provided in the <paramref name="identityJson"/>
        /// parameter as a JSON string:
        /// </para>
        /// <code>
        /// {
        ///        "did": string, (required)
        ///        "verkey": string (optional, can be avoided if did is cryptonym: did == verkey),
        /// }
        /// </code>
        /// <para>The <c>did</c> member specifies the DID to store.  This value is required.</para>
        /// <para>The <c>verkey</c> member specifies the verification key and is optional.</para>
        /// <para>The <c>crypto_type</c> member specifies the type of cryptographic algorithm will be 
        /// used to generate they keys.  If not provided then ed22519 curve will be used.
        /// <note type="note">The only value currently supported for this member is 'ed25519'.</note>
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet to store the DID in.</param>
        /// <param name="identityJson">The identity JSON.</param>
        /// <returns>An asynchronous <see cref="Task"/> that  with no return value the completes when the operation completes.</returns>
        public static Task StoreTheirDidAsync(Wallet wallet, string identityJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(identityJson, "identityJson");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_store_their_did(
                commandHandle,
                wallet.Handle,
                identityJson,
                CallbackHelper.TaskCompletingNoValueCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the verification key for the specified DID.
        /// </summary>
        /// <remarks>
        /// If the provided <paramref name="wallet"/> does not contain the verification key associated with the specified DID then 
        /// an attempt will be made to look up the key from the provided <paramref name="pool"/>. If resolved from the <paramref name="pool"/>
        /// then the DID and key will be automatically cached in the <paramref name="wallet"/>.
        /// <note type="note">
        /// The <see cref="CreateAndStoreMyDidAsync(Wallet, string)"/> and <see cref="Crypto.CreateKeyAsync(Wallet, string)"/> methods both create
        /// similar wallet records so the returned verification key in all generic crypto and messaging functions.
        /// </note>
        /// </remarks>
        /// <param name="pool">The pool to use for resolving the DID if it does not exist in the <paramref name="wallet"/>.</param>
        /// <param name="wallet">The wallet to resolve the DID from.</param>
        /// <param name="did">The DID to get the verification key for.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the verification key associated with the DID.</returns>
        /// <exception cref="WalletItemNotFoundException">Thrown if the DID could not be resolved from the <paramref name="wallet"/> and <paramref name="pool"/>.</exception>
        public static Task<string> KeyForDidAsync(Pool pool, Wallet wallet, string did)
        {
            ParamGuard.NotNull(pool, "pool");
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_key_for_did(
                commandHandle,
                pool.Handle,
                wallet.Handle,
                did,
                KeyForDidCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the verification key for the specified DID.
        /// </summary>
        /// <remarks>
        /// This method will obtain the verification key associated with the specified <paramref name="did"/>from the provided <paramref name="wallet"/> but will
        /// not attempt to retrieve the key from the ledger if not present in the wallet, nor will it perform any freshness check against the ledger to determine 
        /// if the key is up-to-date.  To ensure that the key is fresh use the <see cref="KeyForDidAsync(Pool, Wallet, string)"/> method instead.
        /// <note type="note">
        /// The <see cref="CreateAndStoreMyDidAsync(Wallet, string)"/> and <see cref="Crypto.CreateKeyAsync(Wallet, string)"/> methods both create
        /// similar wallet records so the returned verification key in all generic crypto and messaging functions.
        /// </note>
        /// </remarks>
        /// <param name="wallet">The wallet to resolve the DID from.</param>
        /// <param name="did">The DID to get the verification key for.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the verification key associated with the DID.</returns>
        /// <exception cref="WalletItemNotFoundException">Thrown if the DID could not be resolved from the <paramref name="wallet"/>.</exception>
        public static Task<string> KeyForLocalDidAsync(Wallet wallet, string did)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_key_for_local_did(
                commandHandle,
                wallet.Handle,
                did,
                KeyForLocalDidCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Sets the endpoint details for the specified DID.
        /// </summary>
        /// <param name="wallet">The wallet containing the DID.</param>
        /// <param name="did">The DID to set the endpoint details on.</param>
        /// <param name="address">The address of the endpoint.</param>
        /// <param name="transportKey">The transport key.</param>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation completes.</returns>
        /// <exception cref="InvalidStructureException">Thrown if the <paramref name="did"/> or <paramref name="transportKey"/> values are malformed.</exception>
        public static Task SetEndpointForDidAsync(Wallet wallet, string did, string address, string transportKey)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNullOrWhiteSpace(address, "address");
            ParamGuard.NotNullOrWhiteSpace(transportKey, "transportKey");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_set_endpoint_for_did(
                commandHandle,
                wallet.Handle,
                did,
                address,
                transportKey,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the endpoint details for the specified DID.
        /// </summary>
        /// <remarks>
        /// If the <paramref name="did"/> is present in the <paramref name="wallet"/> and is considered "fresh" then
        /// the endpoint will be resolved from the wallet.  If, on the other hand, the DID is not present in the wallet or
        /// is not fresh then the details will be resolved from the <paramref name="pool"/> and will be cached in the wallet.
        /// </remarks>
        /// <param name="wallet">The wallet containing the DID.</param>
        /// <param name="pool">The pool to resolve the endpoint data from if not present in the wallet.</param>
        /// <param name="did">The DID to get the endpoint data for.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to an <see cref="EndpointForDidResult"/> containing the endpoint information 
        /// associated with the DID.</returns>
        public static Task<EndpointForDidResult> GetEndpointForDidAsync(Wallet wallet, Pool pool, string did)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNull(pool, "pool");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<EndpointForDidResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_get_endpoint_for_did(
                commandHandle,
                wallet.Handle,
                pool.Handle,
                did,
                GetEndpointForDidCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Sets metadata for the specified DID.
        /// </summary>
        /// <remarks>
        /// Any existing metadata stored for the DID will be replaced.
        /// </remarks>
        /// <param name="wallet">The wallet containing the DID.</param>
        /// <param name="did">The DID to set the metadata on.</param>
        /// <param name="metadata">The metadata to store.</param>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation completes.</returns>
        /// <exception cref="WalletItemNotFoundException">Thrown if the <paramref name="wallet"/> does not contain the specified <paramref name="did"/>.</exception>
        /// <exception cref="InvalidStructureException">Thrown if the value provided to the <paramref name="did"/> parameter is malformed.</exception>
        public static Task SetDidMetadataAsync(Wallet wallet, string did, string metadata)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_set_did_metadata(
                commandHandle,
                wallet.Handle,
                did,
                metadata,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the metadata associated with the specified DID.
        /// </summary>
        /// <param name="wallet">The wallet that contains the DID.</param>
        /// <param name="did">The DID to get the metadata for.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the metadata associated with the DID.</returns>
        /// <exception cref="WalletItemNotFoundException">Thrown if the wallet does not contain the specified <paramref name="did"/> or the DID did not have any metadata.</exception>
        /// <exception cref="InvalidStructureException">Thrown if the value provided in the <paramref name="did"/> parameter is malformed.</exception>
        public static Task<string> GetDidMetadataAsync(Wallet wallet, string did)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_get_did_metadata(
                commandHandle,
                wallet.Handle,
                did,
                GetDidMetadataCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Get info about My DID in format: DID, verkey, metadata
        /// </summary>
        /// <returns>The my did with meta async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="myDid">My did.</param>
        public static Task<string> GetMyDidWithMetaAsync(Wallet wallet, string myDid)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(myDid, "myDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_get_my_did_with_meta(
                commandHandle,
                wallet.Handle,
                myDid,
                GetMyDidWithMetaCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Lists created DIDs with metadata as JSON array with each DID in format: DID, verkey, metadata
        /// </summary>
        /// <returns>The my dids with meta async.</returns>
        /// <param name="wallet">Wallet.</param>
        public static Task<string> ListMyDidsWithMetaAsync(Wallet wallet)
        {
            ParamGuard.NotNull(wallet, "wallet");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_list_my_dids_with_meta(
                commandHandle,
                wallet.Handle,
                ListMyDidsWithMetaCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Retrieves abbreviated verkey if it is possible otherwise return full verkey.
        /// </summary>
        /// <returns>The verkey async.</returns>
        /// <param name="did">Did.</param>
        /// <param name="fullVerkey">Full verkey.</param>
        public static Task<string> AbbreviateVerkeyAsync(string did, string fullVerkey)
        {
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNullOrWhiteSpace(fullVerkey, "fullVerkey");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_abbreviate_verkey(
                commandHandle,
                did,
                fullVerkey,
                AbbreviateVerkeyCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}
