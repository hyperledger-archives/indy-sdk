using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;
using static Hyperledger.Indy.IndyNativeMethods;

namespace Hyperledger.Indy.PairwiseApi
{
    /// <summary>
    /// Provides methods for managing pairwise identifiers.
    /// </summary>
    public static class Pairwise
    {
        /// <summary>
        /// Gets the callback to use when the IsExistsAsync command completes.
        /// </summary>
        private static IsPairwiseExistsDelegate _isPairwiseExistsCallback = (xcommand_handle, err, exists) =>
        {
            var taskCompletionSource = PendingCommands.Remove<bool>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(exists);
        };

        /// <summary>
        /// Gets the callback to use when the ListAsync command completes.
        /// </summary>
        private static ListPairwiseDelegate _listPairwiseCallback = (xcommand_handle, err, list_pairwise) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(list_pairwise);
        };

        /// <summary>
        /// Gets the callback to use when the GetAsync command completes.
        /// </summary>
        private static GetPairwiseDelegate _getPairwiseCallback = (xcommand_handle, err, get_pairwise_json) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(get_pairwise_json);
        };

        /// <summary>
        /// Gets whether or not a pairwise for the specified DID exists in the provided wallet.
        /// </summary>
        /// <param name="wallet">The wallet to check in.</param>
        /// <param name="theirDid">The DID to check.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to true if a pairwise exists for the 
        /// DID, otherwise false.</returns>
        public static Task<bool> IsExistsAsync(Wallet wallet, string theirDid)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = IndyNativeMethods.indy_is_pairwise_exists(
                commandHandle,
                wallet.Handle,
                theirDid,
                _isPairwiseExistsCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="wallet"></param>
        /// <param name="theirDid"></param>
        /// <param name="myDid"></param>
        /// <param name="metadata"></param>
        /// <returns></returns>
        public static Task CreateAsync(Wallet wallet, string theirDid, string myDid, string metadata)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = IndyNativeMethods.indy_create_pairwise(
                commandHandle,
                wallet.Handle,
                theirDid,
                myDid,
                metadata,
                CallbackHelper.TaskCompletingNoValueCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="wallet"></param>
        /// <returns></returns>
        public static Task<string> ListAsync(Wallet wallet)
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = IndyNativeMethods.indy_list_pairwise(
                commandHandle,
                wallet.Handle,
                _listPairwiseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="wallet"></param>
        /// <param name="theirDid"></param>
        /// <returns></returns>
        public static Task<string> GetAsync(Wallet wallet, string theirDid)
        {
            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = IndyNativeMethods.indy_get_pairwise(
                commandHandle,
                wallet.Handle,
                theirDid,
                _getPairwiseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="wallet"></param>
        /// <param name="theirDid"></param>
        /// <param name="metadata"></param>
        /// <returns></returns>
        public static Task SetMetadataAsync(Wallet wallet, string theirDid, string metadata)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = IndyNativeMethods.indy_set_pairwise_metadata(
                commandHandle,
                wallet.Handle,
                theirDid,
                metadata,
                CallbackHelper.TaskCompletingNoValueCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }
    }
}
