using Hyperledger.Indy.Utils;
using System;
using System.Collections.Concurrent;
using System.Threading.Tasks;
using static Hyperledger.Indy.WalletApi.NativeMethods;

namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Represents a wallet that stores key value records and provides static methods for managing
    /// wallets.
    /// </summary>
    public sealed class Wallet : IDisposable
    {
        /// <summary>
        /// Wallet type registrations by type name.
        /// </summary>
        private static ConcurrentBag<WalletType> _registeredWalletTypes = new ConcurrentBag<WalletType>();

        /// <summary>
        /// Gets the callback to use when a wallet open command has completed.
        /// </summary>
        private static OpenWalletCompletedDelegate _openWalletCallback = (xcommand_handle, err, wallet_handle) =>
        {
            var taskCompletionSource = PendingCommands.Remove<Wallet>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new Wallet(wallet_handle));
        };

        ///// <summary>
        ///// Registers a custom wallet type implementation.
        ///// </summary>
        ///// <remarks>
        ///// <para>This method allows custom wallet implementations to be registered at runtime so that alternatives
        ///// to the default wallet type can be used.  Implementing a custom wallet is achieved by
        ///// deriving from the <see cref="WalletType"/> class - see the <see cref="WalletType"/> and 
        ///// <see cref="ICustomWallet"/> classes for further detail.
        ///// </para>
        ///// <para>Each custom wallet type is registered with a name which can subsequently be used when 
        ///// creating a new wallet using the <see cref="CreateWalletAsync(string, string, string, string, string)"/> method.
        ///// </para>
        ///// </remarks>
        ///// <param name="typeName">The name of the custom wallet type.</param>
        ///// <param name="walletType">An instance of a class derived from <see cref="WalletType "/> containing the logic for 
        ///// the custom wallet type.</param>
        ///// <returns>An asynchronous <see cref="Task"/> with no return value that completes when
        ///// the registration is complete.</returns>
        //public static Task RegisterWalletTypeAsync(string typeName, WalletType walletType)
        //{
        //    ParamGuard.NotNullOrWhiteSpace(typeName, "typeName");
        //    ParamGuard.NotNull(walletType, "walletType");

        //    var taskCompletionSource = new TaskCompletionSource<bool>();
        //    var commandHandle = PendingCommands.Add(taskCompletionSource);

        //    _registeredWalletTypes.Add(walletType);

        //    var result = NativeMethods.indy_register_wallet_type(
        //        commandHandle,
        //        typeName,
        //        walletType.CreateCallback,
        //        walletType.OpenCallback,
        //        walletType.SetCallback,
        //        walletType.GetCallback,
        //        walletType.GetNotExpiredCallback,
        //        walletType.ListCallback,
        //        walletType.CloseCallback,
        //        walletType.DeleteCallback,
        //        walletType.FreeCallback,
        //        CallbackHelper.TaskCompletingNoValueCallback);

        //    CallbackHelper.CheckResult(result);

        //    return taskCompletionSource.Task;
        //}

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
        ///             Defaults to $HOME/.indy_client/wallets.
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
        ///             Defaults to $HOME/.indy_client/wallets.
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
                _openWalletCallback
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
        ///             Defaults to $HOME/.indy_client/wallets.
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
        /// Whether or not the close function has been called.
        /// </summary>
        private bool _requiresClose = false;

        /// <summary>
        /// Gets the SDK handle for the Wallet instance.
        /// </summary>
        internal IntPtr Handle { get; }

        /// <summary>
        /// Initializes a new Wallet instance with the specified handle.
        /// </summary>
        /// <param name="handle">The SDK handle for the wallet.</param>
        private Wallet(IntPtr handle)
        {
            Handle = handle;
            _requiresClose = true;
        }

        /// <summary>
        /// Closes the wallet.
        /// </summary>
        /// <returns>An asynchronous <see cref="Task"/> with no return value that completes when the operation completes.</returns>
        public Task CloseAsync()
        {
            _requiresClose = false;

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
            if (_requiresClose)
                await CloseAsync();
        }

        /// <summary>
        /// Finalizes the resource during GC if it hasn't been already.
        /// </summary>
        ~Wallet()
        {
            if (_requiresClose)
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
