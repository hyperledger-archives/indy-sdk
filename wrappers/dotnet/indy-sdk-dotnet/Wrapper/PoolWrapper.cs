using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper.Pool
{
    /// <summary>
    /// Async wrapper for Pool functions.
    /// </summary>
    public sealed class PoolWrapper : AsyncWrapperBase
    {       
        
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

        public static Task<IntPtr> OpenPoolLedgerAsync(string configName, string config)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<IntPtr>(commandHandle);

            var result = LibSovrin.sovrin_open_pool_ledger(
                commandHandle,
                configName,
                config,
                ResultWithHandleCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        public static Task RefreshPoolLedgerAsync(IntPtr poolHandle)
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

        public static Task ClosePoolLedgerAsync(IntPtr poolHandle)
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
    }
}
