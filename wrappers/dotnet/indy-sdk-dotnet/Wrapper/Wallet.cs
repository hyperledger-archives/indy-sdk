using System;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for Wallet functions.
    /// </summary>
    public sealed class Wallet : AsyncWrapperBase
    {
        private static ResultWithHandleDelegate OpenWalletResultCallback { get; }

        public IntPtr Handle { get; }

        private Wallet(IntPtr handle)
        {
            Handle = handle;
        }

        static Wallet()
        {
            OpenWalletResultCallback = (xCommandHandle, err, handle) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<Wallet>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(new Wallet(handle));
            };

        }
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

        public static Task<Wallet> OpenWalletAsync(string name, string runtimeConfig, string credentials)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<Wallet>(commandHandle);

            var result = LibSovrin.sovrin_open_wallet(
                commandHandle,
                name,
                runtimeConfig,
                credentials,
                OpenWalletResultCallback
                );

            CheckResult(result);

            return taskCompletionSource.Task;
        }

        private static Task CloseWalletAsync(IntPtr handle)
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

        private static Task WalletSetSeqNoForValueAsync(IntPtr walletHandle, string walletKey)
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

        public Task CloseAsync()
        {
            return CloseWalletAsync(this.Handle);
        }

        public Task SetSeqNoForValueAsync(string walletKey)
        {
            return WalletSetSeqNoForValueAsync(this.Handle, walletKey);
        }
    }
}    
