using System;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Wrapper.Wallet
{
    /// <summary>
    /// Async wrapper class for Wallet functions.
    /// </summary>
    public sealed class WalletWrapper : AsyncWrapperBase
    {
        public static Task CreateWalletAsync(string poolName, string name, string type, string config, string credentials)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var result = LibSovrin.sovrin_create_wallet(
                commandHandle,
                poolName,
                name,
                type,
                config,
                credentials,
                ResultOnlyCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        public static Task<IntPtr> OpenWalletAsync(string name, string runtimeConfig, string credentials)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<IntPtr>(commandHandle);

            var result = LibSovrin.sovrin_open_wallet(
                commandHandle,
                name,
                runtimeConfig,
                credentials,
                ResultWithHandleCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        public static Task CloseWalletAsync(IntPtr handle)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var result = LibSovrin.sovrin_close_wallet(
                commandHandle,
                handle,
                ResultOnlyCallback);

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        public static Task DeleteWalletAsync(string name, string credentials)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var result = LibSovrin.sovrin_delete_wallet(
                commandHandle,
                name,
                credentials,
                ResultOnlyCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        public static Task WalletSetSeqNoForValueAsync(IntPtr walletHandle, string walletKey)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);

            var result = LibSovrin.sovrin_wallet_set_seq_no_for_value(
                commandHandle,
                walletHandle,
                walletKey,
                ResultOnlyCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }
    }
}    
