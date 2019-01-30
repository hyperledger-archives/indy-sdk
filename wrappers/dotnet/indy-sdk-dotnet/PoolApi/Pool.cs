using Hyperledger.Indy.Utils;
using System;
using System.Threading.Tasks;
using static Hyperledger.Indy.PoolApi.NativeMethods;
#if __IOS__
using ObjCRuntime;
#endif

namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Represents a connection to a pool of ledger nodes and provides static methods for managing
    /// connections to pools.
    /// </summary>
    public sealed class Pool : IDisposable
    {
        /// <summary>
        /// Callback to use when a pool open command has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(OpenPoolLedgerCompletedDelegate))]
#endif
        private static void OpenPoolLedgerCallbackMethod(int command_handle, int err, int pool_handle)
        {
            var taskCompletionSource = PendingCommands.Remove<Pool>(command_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new Pool(pool_handle));
        }
        private static OpenPoolLedgerCompletedDelegate OpenPoolLedgerCallback = OpenPoolLedgerCallbackMethod;

        /// <summary>
        /// Callback to use when list pools command has completed.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(ListPoolsCompletedDelegate))]
#endif
        private static void ListPoolsCallbackMethod(int command_handle, int err, string pools)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(command_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(pools);
        }
        private static ListPoolsCompletedDelegate ListPoolsCallback = ListPoolsCallbackMethod;

        /// <summary>
        /// Creates a new local pool configuration with the specified name that can be used later to open a connection to 
        /// pool nodes.
        /// </summary>
        /// <remarks>
        /// <para>
        /// If the configuration specified in the <paramref name="config"/> parameter is null then the 
        /// default configuration will be used, however if specified the value should adhere to the following
        /// JSON format:
        /// <code>
        /// {
        ///     "genesis_txn": "path/to/genesis/transaction/file"
        /// }
        /// </code>
        /// If the value of the <c>genesis_txn</c> key in the JSON is null then a default file will be
        /// used.  If the file specified does not exist it will be created.
        /// </para>
        /// </remarks>
        /// <seealso cref="OpenPoolLedgerAsync(string, string)"/>
        /// <seealso cref="DeletePoolLedgerConfigAsync(string)"/>
        /// <param name="configName">The name for the configuration.</param>
        /// <param name="config">Pool configuration json. if NULL, then default config will be used. Example:
        /// {
        ///     "genesis_txn": string (optional), A path to genesis transaction file. If NULL, then a default one will be used.
        ///                    If file doesn't exists default one will be created.
        /// }</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> with no return value that completes when
        /// the configuration is created.</returns>
        public static Task CreatePoolLedgerConfigAsync(string configName, string config)
        {
            ParamGuard.NotNullOrWhiteSpace(configName, "configName");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_create_pool_ledger_config(
                commandHandle,
                configName,
                config,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Deletes an existing pool configuration.
        /// </summary>
        /// <seealso cref="CreatePoolLedgerConfigAsync(string, string)"/>
        /// <param name="configName">The name of the configuration to delete.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> with no return value that completes when
        /// the configuration is deleted.</returns>
        public static Task DeletePoolLedgerConfigAsync(string configName)
        {
            ParamGuard.NotNullOrWhiteSpace(configName, "configName");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_delete_pool_ledger_config(
                commandHandle,
                configName,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Opens a pool and connects to the ledger nodes.
        /// </summary>
        /// <remarks>
        /// A Pool cannot be opened unless the a pool configuration with the specified name was previously
        /// configured using the <see cref="CreatePoolLedgerConfigAsync(string, string)"/> method.
        /// 
        /// When opening a pool the runtime configuration can be specified using the <paramref name="config"/>
        /// parameter, which expects a JSON string with the following format:
        /// 
        /// <code>
        /// {
        ///     "refresh_on_open": bool (optional), Forces pool ledger to be refreshed immediately after opening.
        ///                      Defaults to true.
        ///     "auto_refresh_time": int (optional), After this time in minutes pool ledger will be automatically refreshed.
        ///                        Use 0 to disable automatic refresh. Defaults to 24*60.
        ///     "network_timeout": int (optional), Network timeout for communication with nodes in milliseconds.
        ///                       Defaults to 20000.
        /// }
        /// </code>
        /// 
        /// If the <paramref name="config"/> parameter is null then the default configuration will be used.
        /// 
        /// <note type="note">Attempting to open a pool with the same name more than once will result in an error.</note>
        /// </remarks>
        /// <param name="configName">The name of the pool configuration to use.</param>
        /// <param name="config">The runtime configuration to use.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a Pool instance once the pool is opened.</returns>
        public static Task<Pool> OpenPoolLedgerAsync(string configName, string config)
        {
            ParamGuard.NotNullOrWhiteSpace(configName, "configName");

            var taskCompletionSource = new TaskCompletionSource<Pool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_open_pool_ledger(
                commandHandle,
                configName,
                config,
                OpenPoolLedgerCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Lists names of created pool ledgers
        /// </summary>
        /// <returns>The pools json.</returns>
        public static Task<string> ListPoolsAsync()
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_list_pools(
                commandHandle,
                ListPoolsCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Whether or not the close function has been called.
        /// </summary>
        private bool _requiresClose = false;

        /// <summary>
        /// Gets the handle for the pool.
        /// </summary>
        internal int Handle { get; }

        /// <summary>
        /// Initializes a new Pool instance with the specified handle.
        /// </summary>
        /// <param name="handle">The handle of the underlying unmanaged pool.</param>
        private Pool(int handle)
        {
            Handle = handle;
            _requiresClose = true;
        }

        /// <summary>
        /// Refreshes a local copy of the pool and updates the pool's node connections.
        /// </summary>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation completes.</returns>
        public Task RefreshAsync()
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_refresh_pool_ledger(
                commandHandle,
                Handle,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Closes the pool.
        /// </summary>
        /// <remarks>
        /// <note type="note">Once a Pool instance is closed it cannot be opened again.  Instead call the 
        /// <see cref="OpenPoolLedgerAsync(string, string)"/> method to open a new Pool instance.</note>
        /// </remarks>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation completes.</returns>
        public Task CloseAsync()
        {
            _requiresClose = false;

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_close_pool_ledger(
                commandHandle,
                Handle,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

            GC.SuppressFinalize(this);

            return taskCompletionSource.Task;
        }

        /// <summary> 
        /// Set PROTOCOL_VERSION to specific version. 
        /// 
        /// There is a global property PROTOCOL_VERSION that used in every request to the pool and 
        /// specified version of Indy Node which Libindy works. 
        /// 
        /// By default PROTOCOL_VERSION=1. 
        /// </summary> 
        /// <param name="protocolVersion">Protocol version will be used: 
        /// <c> 
        ///     1 - for Indy Node 1.3 
        ///     2 - for Indy Node 1.4 and greater
        /// </c></param> 
        public static Task SetProtocolVersionAsync(int protocolVersion)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_set_protocol_version(
                commandHandle,
                protocolVersion,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(result);

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
        ~Pool()
        {
            if (_requiresClose)
            {
                NativeMethods.indy_close_pool_ledger(
                   -1,
                   Handle,
                   CallbackHelper.NoValueCallback
                );
            }
        }
    }
}
