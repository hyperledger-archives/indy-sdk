using System;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.LibIndy;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Basic wrapper API for Wallet functions.
    /// </summary>
    public sealed class Wallet : AsyncWrapperBase
    {
        /// <summary>
        /// Gets the callback to use when a wallet open command has completed.
        /// </summary>
        private static OpenWalletResultDelegate _openWalletCallback = (xCommandHandle, err, handle) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<Wallet>(xCommandHandle);

            if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                return;

            taskCompletionSource.SetResult(new Wallet(handle));
        };

        /// <summary>
        /// Creates a new wallet.
        /// </summary>
        /// <param name="poolName">The name of the pool the wallet is associated with.</param>
        /// <param name="name">The name of the wallet.</param>
        /// <param name="type">The type of the wallet.  Use null to indicate the 'default' type.</param>
        /// <param name="config">The wallet configuration JSON.  Use null to indicate the default config.</param>
        /// <param name="credentials">The wallet credentials JSON or null to use the default credentials.</param>
        /// <returns>An asynchronous Task with no return value.</returns>
        public static Task CreateWalletAsync(string poolName, string name, string type, string config, string credentials)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.indy_create_wallet(
                commandHandle,
                poolName,
                name,
                type,
                config,
                credentials,
                _noValueCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Opens an existing Wallet.
        /// </summary>
        /// <param name="name">The name of the Wallet to open.</param>
        /// <param name="runtimeConfig">The runtime wallet configuration JSON or null to use the default configuration.</param>
        /// <param name="credentials">The wallet credentials JSON or null to use the default credentials.</param>
        /// <remarks>The wallet with the name specified must have already been created using the CreateWalletAsync method.</remarks>
        /// <returns>An asynchronous Task that returns a Wallet instance.</returns>
        public static Task<Wallet> OpenWalletAsync(string name, string runtimeConfig, string credentials)
        {
            var taskCompletionSource = new TaskCompletionSource<Wallet>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.indy_open_wallet(
                commandHandle,
                name,
                runtimeConfig,
                credentials,
                _openWalletCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Closes an open wallet.
        /// </summary>
        /// <param name="handle">The handle of the wallet to close.</param>
        /// <returns>An asynchronous Task with no return value.</returns>
        private static Task CloseWalletAsync(IntPtr handle)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.indy_close_wallet(
                commandHandle,
                handle,
                _noValueCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Deletes a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to delete.</param>
        /// <param name="credentials">The wallet credentials JSON or null to use the default credentials.</param>
        /// <returns>An asyncronous Task with no return value.</returns>
        public static Task DeleteWalletAsync(string name, string credentials)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.indy_delete_wallet(
                commandHandle,
                name,
                credentials,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Sets the sequence number on the specified wallet for the specified key.
        /// </summary>
        /// <param name="walletHandle">The handle of the wallet.</param>
        /// <param name="walletKey">The key to set the sequence number for.</param>
        /// <returns></returns>
        private static Task WalletSetSeqNoForValueAsync(IntPtr walletHandle, string walletKey)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.indy_wallet_set_seq_no_for_value(
                commandHandle,
                walletHandle,
                walletKey,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the SDK handle for the Wallet instance.
        /// </summary>
        public IntPtr Handle { get; }

        /// <summary>
        /// Initializes a new Wallet instance with the specified handle.
        /// </summary>
        /// <param name="handle">The SDK handle for the wallet.</param>
        private Wallet(IntPtr handle)
        {
            Handle = handle;
        }

        /// <summary>
        /// Closes the wallet.
        /// </summary>
        /// <returns>An asyncronous Task with no return value.</returns>
        public Task CloseAsync()
        {
            return CloseWalletAsync(this.Handle);
        }

        /// <summary>
        /// Sets the sequence number for the specified key.
        /// </summary>
        /// <param name="walletKey">The key to set the sequence number for.</param>
        /// <returns>An asyncronous Task with no return value.</returns>
        public Task SetSeqNoForValueAsync(string walletKey)
        {
            return WalletSetSeqNoForValueAsync(this.Handle, walletKey);
        }
    }
}    
