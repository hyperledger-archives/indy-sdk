using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.LibIndy;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for pool functions.
    /// </summary>
    public sealed class Pool : AsyncWrapperBase
    {
        /// <summary>
        /// Callback to use when a pool open command has completed.
        /// </summary>
        private static OpenPoolLedgerResultDelegate _openPoolLedgerCallback = (xCommandHandle, err, handle) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<Pool>(xCommandHandle);

            if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                return;

            taskCompletionSource.SetResult(new Pool(handle));
        };
        
        /// <summary>
        /// Creates a new pool configuration.
        /// </summary>
        /// <param name="configName">The name for the configuration.</param>
        /// <param name="config">The configuration JSON.</param>
        /// <returns>An asynchronous Task with no return value.</returns>
        public static Task CreatePoolLedgerConfigAsync(string configName, string config)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.sovrin_create_pool_ledger_config(
                commandHandle,
                configName,
                config,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Deletes an existing pool configuration.
        /// </summary>
        /// <param name="configName">The name of the configuration to delete.</param>
        /// <returns>An asynchronous Task with no return value.</returns>
        public static Task DeletePoolLedgerConfigAsync(string configName)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.sovrin_delete_pool_ledger_config(
                commandHandle,
                configName,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Opens a pool.
        /// </summary>
        /// <param name="configName">The name of the pool configuration to use.</param>
        /// <param name="config">The runtime configuration to use.</param>
        /// <returns>An aysnchronous Task that returns a Pool instance.</returns>
        public static Task<Pool> OpenPoolLedgerAsync(string configName, string config)
        {
            var taskCompletionSource = new TaskCompletionSource<Pool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.sovrin_open_pool_ledger(
                commandHandle,
                configName,
                config,
                _openPoolLedgerCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Refreshes a pool.
        /// </summary>
        /// <param name="poolHandle">The handle of the pool to refresh.</param>
        /// <returns>An asynchronous Task with no return value.</returns>
        private static Task RefreshPoolLedgerConfigAsync(IntPtr poolHandle)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.sovrin_refresh_pool_ledger(
                commandHandle,
                poolHandle,
                _noValueCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Closes a pool.
        /// </summary>
        /// <param name="poolHandle">The handle of the pool to close.</param>
        /// <returns>An asynchronous Task with no return value.</returns>
        private static Task ClosePoolLedgerAsync(IntPtr poolHandle)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var result = LibIndy.sovrin_close_pool_ledger(
                commandHandle,
                poolHandle,
                _noValueCallback
                );

            CheckResult(result);
            
            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the handle for the pool.
        /// </summary>
        public IntPtr Handle { get; }

        /// <summary>
        /// Initializes a new Pool instance with the specified handle.
        /// </summary>
        /// <param name="handle">The handle of the underlying unmanaged pool.</param>
        private Pool(IntPtr handle)
        {
            Handle = handle;
        }

        /// <summary>
        /// Refreshes the pool.
        /// </summary>
        /// <returns>An asynchronous Task with no return value.</returns>
        public Task RefreshAsync()
        {
            return RefreshPoolLedgerConfigAsync(this.Handle);
        }

        /// <summary>
        /// Closes the pool.
        /// </summary>
        /// <returns>An asynchronous Task with no return value.</returns>
        public Task CloseAsync()
        {
            return ClosePoolLedgerAsync(this.Handle);
        }
    }
}
