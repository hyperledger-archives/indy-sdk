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

        private static ListWalletsCompletedDelegate _listWalletsCallback = (xcommand_handle, err, wallets) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(wallets);
        };

        /// <summary>
        /// Registers a custom wallet type implementation.
        /// </summary>
        /// <remarks>
        /// <para>This method allows custom wallet implementations to be registered at runtime so that alternatives
        /// to the default wallet type can be used.  Implementing a custom wallet is achieved by
        /// deriving from the <see cref="WalletType"/> class - see the <see cref="WalletType"/> and 
        /// <see cref="ICustomWallet"/> classes for further detail.
        /// </para>
        /// <para>Each custom wallet type is registered with a name which can subsequently be used when 
        /// creating a new wallet using the <see cref="CreateWalletAsync(string, string, string, string, string)"/> method.
        /// </para>
        /// </remarks>
        /// <param name="typeName">The name of the custom wallet type.</param>
        /// <param name="walletType">An instance of a class derived from <see cref="WalletType "/> containing the logic for 
        /// the custom wallet type.</param>
        /// <returns>An asynchronous <see cref="Task"/> with no return value that completes when
        /// the registration is complete.</returns>
        public static Task RegisterWalletTypeAsync(string typeName, WalletType walletType)
        {
            ParamGuard.NotNullOrWhiteSpace(typeName, "typeName");
            ParamGuard.NotNull(walletType, "walletType");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            _registeredWalletTypes.Add(walletType);          

            var result = NativeMethods.indy_register_wallet_type(
                commandHandle,
                typeName,
                walletType.CreateCallback,
                walletType.OpenCallback,
                walletType.SetCallback,
                walletType.GetCallback,
                walletType.GetNotExpiredCallback,
                walletType.ListCallback,
                walletType.CloseCallback,
                walletType.DeleteCallback,
                walletType.FreeCallback,
                CallbackHelper.TaskCompletingNoValueCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a new wallet.
        /// </summary>
        /// <remarks>
        /// <para>Each created wallet is given a name which is then subsequently used to open it
        /// with the <see cref="OpenWalletAsync(string, string, string)"/> or delete it using the
        /// <see cref="DeleteWalletAsync(string, string)"/> static methods.    
        /// <note type="note">Wallet names must be unique within a pool.</note>
        /// </para>
        /// <para>
        /// When creating a new Wallet the <paramref name="type"/> parameter can be null or "default" to
        /// use the default wallet implementation, or a type name specified in an earlier call to the 
        /// <see cref="RegisterWalletTypeAsync(string, WalletType)"/> method to use a custom wallet implementation.
        /// Attempting to use a wallet type that hasn't previously been registered will result in an error.
        /// </para>
        /// <para>The <paramref name="config"/> parameter allows configuration values to be passed to the wallet
        /// when it is created.  When using the default wallet this value can be null to use the default 
        /// wallet configuration or a JSON string with the following format can be used:
        /// <code>
        /// {
        ///     "freshness_time": int
        /// }
        /// </code>
        /// The value of the <c>freshness_time</c> key is an integer representing the number of seconds
        /// a value in the wallet will remain valid before expiring.  If not specified the default value 
        /// for <c>freshness_time</c> is 24 * 60 seconds.
        /// </para>
        /// <para>If using a custom wallet type the content of the <paramref name="config"/> parameter can
        /// be any string value; it is up to the custom wallet type implementer to interpret the value.
        /// </para>
        /// <para>The <paramref name="credentials"/> parameter is unused in the default wallet at present, 
        /// however the value can be used by custom wallet implementations; it is up to the custom wallet 
        /// type implementer to interpret the value.</para>
        /// </remarks>
        /// <param name="poolName">The name of the pool the wallet is associated with.</param>
        /// <param name="name">The name of the wallet.</param>
        /// <param name="type">The type of the wallet. </param>
        /// <param name="config">The wallet configuration JSON.</param>
        /// <param name="credentials">The wallet credentials JSON or null to use the default credentials.</param>
        /// <returns>An asynchronous <see cref="Task"/> with no return value the completes when the operation has finished.</returns>
        public static Task CreateWalletAsync(string poolName, string name, string type, string config, string credentials)
        {
            ParamGuard.NotNullOrWhiteSpace(poolName, "poolName");
            ParamGuard.NotNullOrWhiteSpace(name, "name");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_create_wallet(
                commandHandle,
                poolName,
                name,
                type,
                config,
                credentials,
                CallbackHelper.TaskCompletingNoValueCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Opens a Wallet.
        /// </summary>
        /// <remarks>
        /// <para>Opens a wallet by name using the name of a wallet created earlier using the 
        /// <see cref="CreateWalletAsync(string, string, string, string, string)"/> method.
        /// </para>
        /// <note type="note">Attempting to open the same wallet more than once will result in an error.</note>
        /// <para>
        /// The <paramref name="runtimeConfig"/> parameter allows the default configuration of the wallet
        /// to be overridden while opening the wallet; this does not replace the configuration registered
        /// when the wallet was created but instead overrides it for the duration of this opening only.
        /// See the <see cref="CreateWalletAsync(string, string, string, string, string)"/> method for 
        /// details on the configuration string supported by the default wallet type; custom wallet
        /// types will can support their own format.
        /// </para>
        /// <para>The <paramref name="credentials"/> parameter is unused in the default wallet at present, 
        /// however the value can be used by custom wallet implementations; it is up to the custom wallet 
        /// type implementer to interpret the value.
        /// </para>
        /// </remarks>
        /// <param name="name">The name of the Wallet to open.</param>
        /// <param name="runtimeConfig">The runtime configuration to override the configuration the wallet was created with.</param>
        /// <param name="credentials">The wallet credentials.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="Wallet"/> instance when the operation completes.</returns>
        public static Task<Wallet> OpenWalletAsync(string name, string runtimeConfig, string credentials)
        {
            ParamGuard.NotNullOrWhiteSpace(name, "name");

            var taskCompletionSource = new TaskCompletionSource<Wallet>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_open_wallet(
                commandHandle,
                name,
                runtimeConfig,
                credentials,
                _openWalletCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Lists created wallets as JSON array with each wallet metadata: name, type, name of associated pool
        /// </summary>
        /// <returns>The wallets async.</returns>
        public static Task<string> ListWalletsAsync()
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_list_wallets(
                commandHandle,
                _listWalletsCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Deletes a wallet.
        /// </summary>
        /// <remarks>
        /// <para>Deletes a wallet created earlier using the <see cref="CreateWalletAsync(string, string, string, string, string)"/>
        /// by name.
        /// </para>
        /// <para>The <paramref name="credentials"/> parameter is unused in the default wallet at present, 
        /// however the value can be used by custom wallet implementations; it is up to the custom wallet 
        /// type implementer to interpret the value.
        /// </para>
        /// </remarks>
        /// <param name="name">The name of the wallet to delete.</param>
        /// <param name="credentials">The wallet credentials.</param>
        /// <returns>An asynchronous <see cref="Task"/> with no return value that completes when the operation completes.</returns>
        public static Task DeleteWalletAsync(string name, string credentials)
        {
            ParamGuard.NotNullOrWhiteSpace(name, "name");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_delete_wallet(
                commandHandle,
                name,
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
