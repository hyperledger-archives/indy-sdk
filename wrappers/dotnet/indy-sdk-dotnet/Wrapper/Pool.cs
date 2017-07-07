using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for pool functions.
    /// </summary>
    public sealed class Pool : AsyncWrapperBase
    {       
        private static ResultWithHandleDelegate OpenPoolLedgerResultCallback { get;  }

        public IntPtr Handle { get; }

        private Pool(IntPtr handle)
        {
            Handle = handle;
        }

        static Pool()
        {
            OpenPoolLedgerResultCallback = (xCommandHandle, err, handle) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<Pool>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;
                
                taskCompletionSource.SetResult(new Pool(handle));
            };
        }
        
        public static Task CreatePoolLedgerConfigAsync(string configName, string config)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var result = LibSovrin.sovrin_create_pool_ledger_config(
                commandHandle,
                configName,
                config,
                ResultOnlyCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }               

        public static Task DeletePoolLedgerConfigAsync(string configName)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var result = LibSovrin.sovrin_delete_pool_ledger_config(
                commandHandle,
                configName,
                ResultOnlyCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        public static Task<Pool> OpenPoolLedgerAsync(string configName, string config)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<Pool>(commandHandle);

            var result = LibSovrin.sovrin_open_pool_ledger(
                commandHandle,
                configName,
                config,
                OpenPoolLedgerResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        private static Task RefreshPoolLedgerConfigAsync(IntPtr poolHandle)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var result = LibSovrin.sovrin_refresh_pool_ledger(
                commandHandle,
                poolHandle,
                ResultOnlyCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        private static Task ClosePoolLedgerAsync(IntPtr poolHandle)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var result = LibSovrin.sovrin_close_pool_ledger(
                commandHandle,
                poolHandle,
                ResultOnlyCallback
                );

            CheckResult(result);
            
            return taskCompletionSource.Task;
        }

        public Task RefreshAsync()
        {
            return RefreshPoolLedgerConfigAsync(this.Handle);
        }

        public Task CloseAsync()
        {
            return ClosePoolLedgerAsync(this.Handle);
        }
    }
}
