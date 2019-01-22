using Hyperledger.Indy.Utils;
using System;
using System.Collections.Concurrent;
using System.Threading.Tasks;
using static Hyperledger.Indy.WalletApi.NativeMethods;
#if __IOS__
using ObjCRuntime;
#endif

namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Represents a wallet that stores key value records and provides static methods for managing
    /// wallets.
    /// </summary>
    public sealed class Wallet : IDisposable
    {
        /// <summary>
        /// Gets the callback to use when a wallet open command has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(OpenWalletCompletedDelegate))]
#endif
        private static void OpenWalletCallbackMethod(int xcommand_handle, int err, int wallet_handle)
        {
            var taskCompletionSource = PendingCommands.Remove<Wallet>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new Wallet(wallet_handle));
        }
        private static OpenWalletCompletedDelegate OpenWalletCallback = OpenWalletCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(GenerateWalletKeyCompletedDelegate))]
#endif
        private static void GenerateWalletKeyCallbackMethod(int xcommand_handle, int err, string key)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(key);
        }
        private static GenerateWalletKeyCompletedDelegate GenerateWalletKeyCallback = GenerateWalletKeyCallbackMethod;


        /// <summary>
        /// Create a new secure wallet.
        /// </summary>
        /// <returns>The wallet async.</returns>
        /// <param name="config">
        /// Wallet configuration json.
        /// <code>
        /// {
        ///   "id": string, Identifier of the wallet.
        ///         Configured storage uses this identifier to lookup exact wallet data placement.
        ///   "storage_type": optional&lt;string>, Type of the wallet storage. Defaults to 'default'.
        ///                  'Default' storage type allows to store wallet data in the local file.
        ///                  Custom storage types can be registered with indy_register_wallet_storage call.
        ///   "storage_config": optional&lt;object>, Storage configuration json. Storage type defines set of supported keys.
        ///                     Can be optional if storage supports default configuration.
        ///                     For 'default' storage type configuration is:
        ///   {
        ///     "path": optional&lt;string>, Path to the directory with wallet files.
        ///             Defaults to $HOME/.indy_client/wallet.
        ///             Wallet will be stored in the file {path}/{id}/sqlite.db
        ///   }
        /// }
        /// </code>
        /// </param>
        /// <param name="credentials">
        /// Wallet credentials json
        /// <code>
        /// {
        ///   "key": string, Passphrase used to derive wallet master key
        ///   "storage_credentials": optional&lt;object> Credentials for wallet storage. Storage type defines set of supported keys.
        ///                          Can be optional if storage supports default configuration.
        ///                          For 'default' storage type should be empty.
        ///
        /// }
        /// </code>
        /// </param>
        public static Task CreateWalletAsync(string config, string credentials)
        {
            ParamGuard.NotNullOrWhiteSpace(config, "config");
            ParamGuard.NotNullOrWhiteSpace(credentials, "credentials");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_create_wallet(
                commandHandle,
                config,
                credentials,
                CallbackHelper.TaskCompletingNoValueCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Open the wallet.
        ///
        /// Wallet must be previously created with <see cref="CreateWalletAsync(string, string)"/> method.
        /// </summary>
        /// <returns>Handle to opened wallet to use in methods that require wallet access.</returns>
        /// <param name="config">
        /// Wallet configuration json.
        /// <code>
        /// {
        ///   "id": string, Identifier of the wallet.
        ///         Configured storage uses this identifier to lookup exact wallet data placement.
        ///   "storage_type": optional&lt;string>, Type of the wallet storage. Defaults to 'default'.
        ///                  'Default' storage type allows to store wallet data in the local file.
        ///                  Custom storage types can be registered with indy_register_wallet_storage call.
        ///   "storage_config": optional&lt;object>, Storage configuration json. Storage type defines set of supported keys.
        ///                     Can be optional if storage supports default configuration.
        ///                     For 'default' storage type configuration is:
        ///   {
        ///     "path": optional&lt;string>, Path to the directory with wallet files.
        ///             Defaults to $HOME/.indy_client/wallet.
        ///             Wallet will be stored in the file {path}/{id}/sqlite.db
        ///   }
        /// }
        /// </code>
        /// </param>
        /// <param name="credentials">
        /// Wallet credentials json
        ///   {
        ///       "key": string, Passphrase used to derive current wallet master key
        ///       "rekey": optional&lt;string>, If present than wallet master key will be rotated to a new one
        ///                                  derived from this passphrase.
        ///       "storage_credentials": optional&lt;object> Credentials for wallet storage. Storage type defines set of supported keys.
        ///                              Can be optional if storage supports default configuration.
        ///                              For 'default' storage type should be empty.
        ///
        ///   }
        /// </param>
        public static Task<Wallet> OpenWalletAsync(string config, string credentials)
        {
            ParamGuard.NotNullOrWhiteSpace(config, "config");
            ParamGuard.NotNullOrWhiteSpace(credentials, "credentials");

            var taskCompletionSource = new TaskCompletionSource<Wallet>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_open_wallet(
                commandHandle,
                config,
                credentials,
                OpenWalletCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Exports opened wallet
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>The async.</returns>
        /// <param name="exportConfig">
        /// <code>
        /// JSON containing settings for input operation.
        ///   {
        ///     "path": &lt;string>, Path of the file that contains exported wallet content
        ///     "key": &lt;string>, Passphrase used to derive export key
        ///   }
        /// </code>
        /// </param>
        public Task ExportAsync(string exportConfig)
        {
            ParamGuard.NotNullOrWhiteSpace(exportConfig, "exportConfig");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_export_wallet(
                commandHandle,
                this.Handle,
                exportConfig,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a new secure wallet and then imports its content
        /// according to fields provided in import_config
        /// This can be seen as an <see cref="CreateWalletAsync(string, string)"/> call with additional content import
        ///
        /// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
        /// in the future releases.
        /// </summary>
        /// <returns>The async.</returns>
        /// <param name="config">
        /// Wallet configuration json.
        /// <code>
        /// {
        ///   "id": string, Identifier of the wallet.
        ///         Configured storage uses this identifier to lookup exact wallet data placement.
        ///   "storage_type": optional&lt;string>, Type of the wallet storage. Defaults to 'default'.
        ///                  'Default' storage type allows to store wallet data in the local file.
        ///                  Custom storage types can be registered with indy_register_wallet_storage call.
        ///   "storage_config": optional&lt;object>, Storage configuration json. Storage type defines set of supported keys.
        ///                     Can be optional if storage supports default configuration.
        ///                     For 'default' storage type configuration is:
        ///   {
        ///     "path": optional&lt;string>, Path to the directory with wallet files.
        ///             Defaults to $HOME/.indy_client/wallet.
        ///             Wallet will be stored in the file {path}/{id}/sqlite.db
        ///   }
        /// }
        /// </code>
        /// </param>
        /// <param name="credentials">Wallet credentials json
        /// <code>
        /// {
        ///   "key": string, Passphrase used to derive wallet master key
        ///   "storage_credentials": optional&lt;object> Credentials for wallet storage. Storage type defines set of supported keys.
        ///                          Can be optional if storage supports default configuration.
        ///                          For 'default' storage type should be empty.
        ///
        /// }
        /// </code>
        /// </param>
        /// <param name="importConfig">
        /// Import settings json.
        /// <code>
        /// {
        ///   "path": &lt;string>, path of the file that contains exported wallet content
        ///   "key": &lt;string>, passphrase used to derive export key
        /// }
        /// </code>
        /// </param>
        public static Task ImportAsync(string config, string credentials, string importConfig)
        {
            ParamGuard.NotNullOrWhiteSpace(config, "config");
            ParamGuard.NotNullOrWhiteSpace(credentials, "credentials");
            ParamGuard.NotNullOrWhiteSpace(importConfig, "importConfig");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_import_wallet(
                commandHandle,
                config,
                credentials,
                importConfig,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Deletes a wallet.
        /// </summary>
        /// <remarks>
        /// <para>Deletes a wallet created earlier using the <see cref="CreateWalletAsync(string, string)"/>
        /// by name.
        /// </para>
        /// <para>The <paramref name="credentials"/> parameter is unused in the default wallet at present, 
        /// however the value can be used by custom wallet implementations; it is up to the custom wallet 
        /// type implementer to interpret the value.
        /// </para>
        /// </remarks>
        /// <param name="config">The name of the wallet to delete.</param>
        /// <param name="credentials">The wallet credentials.</param>
        /// <returns>An asynchronous <see cref="Task"/> with no return value that completes when the operation completes.</returns>
        public static Task DeleteWalletAsync(string config, string credentials)
        {
            ParamGuard.NotNullOrWhiteSpace(config, "config");
            ParamGuard.NotNullOrWhiteSpace(credentials, "credentials");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_delete_wallet(
                commandHandle,
                config,
                credentials,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Generate wallet master key.
        /// Returned key is compatible with "RAW" key derivation method.
        /// It allows to avoid expensive key derivation for use cases when wallet keys can be stored in a secure enclave.
        /// </summary>
        /// <returns>The generated wallet key.</returns>
        /// <param name="config">
        /// config: (optional) key configuration json.
        /// {
        ///   "seed": optional&lt;string> Seed that allows deterministic key creation (if not set random one will be used).
        /// }</param>
        public static Task<string> GenerateWalletKeyAsync(string config)
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_generate_wallet_key(
                commandHandle,
                config,
                GenerateWalletKeyCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Status indicating whether or not the wallet is open.
        /// </summary>
        public bool IsOpen { get; private set; }

        /// <summary>
        /// Gets the SDK handle for the Wallet instance.
        /// </summary>
        internal int Handle { get; }
        
        /// <summary>
        /// Initializes a new Wallet instance with the specified handle.
        /// </summary>
        /// <param name="handle">The SDK handle for the wallet.</param>
        private Wallet(int handle)
        {
            Handle = handle;
            IsOpen = true;
        }

        /// <summary>
        /// Closes the wallet.
        /// </summary>
        /// <returns>An asynchronous <see cref="Task"/> with no return value that completes when the operation completes.</returns>
        public Task CloseAsync()
        {
            IsOpen = false;

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_close_wallet(
                commandHandle,
                Handle,
                CallbackHelper.TaskCompletingNoValueCallback);

            CallbackHelper.CheckResult(result);

            GC.SuppressFinalize(this);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Disposes of resources.
        /// </summary>
        public async void Dispose()
        {
            if (IsOpen)
                await CloseAsync();
        }

        /// <summary>
        /// Finalizes the resource during GC if it hasn't been already.
        /// </summary>
        ~Wallet()
        {
            if (IsOpen)
            {
                NativeMethods.indy_close_wallet(
                   -1,
                   Handle,
                   CallbackHelper.NoValueCallback
                );
            }
        }
    }
}
