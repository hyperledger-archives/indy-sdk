using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;
using static Hyperledger.Indy.IndyNativeMethods;

namespace Hyperledger.Indy.CryptoApi
{
    /// <summary>
    /// Provides methods for performing .
    /// </summary>
    public static class Crypto
    {
        /// <summary>
        /// Gets the callback to use when the command for CreateKeyAsync has completed.
        /// </summary>
        private static SignusCreateKeyCompletedDelegate _createKeyCompletedCallback = (xcommand_handle, err, verkey) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(verkey);
        };

        /// <summary>
        /// Gets the callback to use when the command for GetKeyMetaDataAsync has completed.
        /// </summary>
        private static SignusGetKeyMetadataCompletedDelegate _getKeyMetadataCompletedCallback = (xcommand_handle, err, metadata) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(metadata);
        };

        /// <summary>
        /// Creates a key in the provided wallet.
        /// </summary>
        /// <remarks>
        /// The <paramref name="keyJson"/> parameter must contain a JSON object although all properties of the object are optional.  The schema
        /// the object must conform to are as follows:
        /// <code>
        /// {
        ///     "seed": string, // Optional (if not set random one will be used); Seed information that allows deterministic key creation.
        ///     "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
        /// }
        /// </code>
        /// The <c>seed</c> member is optional and is used to specify the seed to use for key creation - if this parameter is not set then a random seed will be used.
        /// The <c>crypto_type</c> member is also optional and will default to ed25519 curve if not set.
        /// <note type="note">At present the crypto_type member only supports the value 'ed22519'.</note>
        /// </remarks>
        /// <param name="wallet">The wallet to create the key in.</param>
        /// <param name="keyJson">The JSON string describing the key.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the verification key of the generated key-pair.</returns>
        /// <exception cref="InvalidStructureException">Thrown if the value passed to the <paramref name="keyJson"/> parameter is malformed or contains invalid data.</exception>
        public static Task<string> CreateKeyAsync(Wallet wallet, string keyJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(keyJson, "keyJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_create_key(
                commandHandle,
                wallet.Handle,
                keyJson,
                _createKeyCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Sets user defined metadata for a key-pair in the specified wallet.
        /// </summary>
        /// <remarks>
        /// Any existing metadata stored for the key-pair will be replaced.
        /// </remarks>
        /// <param name="wallet">The wallet containing the key.</param>
        /// <param name="verKey">The verification key of the key pair.</param>
        /// <param name="metadata">The metadata to set.</param>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation completes.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if the wallet does not contain a key-pair matching the provided <paramref name="verKey"/>.</exception>
        public static Task SetKeyMetadataAsync(Wallet wallet, string verKey, string metadata)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(verKey, "verKey");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_set_key_metadata(
                commandHandle,
                wallet.Handle,
                verKey,
                metadata,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the user defined metadata stored against a key-pair in the specified wallet.
        /// </summary>
        /// <remarks>
        /// If no metadata is stored against the specified key-pair null will be returned.</remarks>
        /// <param name="wallet">The wallet containing the key-pair.</param>
        /// <param name="verKey">The verification key of the key-pair.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the metadata associated with the key-pair.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if the wallet does not contain a key-pair matching the provided <paramref name="verKey"/> or they key-pair has no metadata.</exception>
        public static Task<string> GetKeyMetadataAsync(Wallet wallet, string verKey)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(verKey, "verKey");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_get_key_metadata(
                commandHandle,
                wallet.Handle,
                verKey,
                _getKeyMetadataCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

    }
}
