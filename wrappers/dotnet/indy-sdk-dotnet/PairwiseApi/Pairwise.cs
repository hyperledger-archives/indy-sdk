using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System.Threading.Tasks;
using static Hyperledger.Indy.PairwiseApi.NativeMethods;
#if __IOS__
using ObjCRuntime;
#endif

namespace Hyperledger.Indy.PairwiseApi
{
    /// <summary>
    /// Provides methods for managing pairwise identifiers.
    /// </summary>
    /// <remarks>
    /// A Pairwise is a record of the relationship between a DID owned by the caller of the API and
    /// one belonging to another party, referred to respectively in this API  as <c>myDID</c>and <c>theirDID</c>.
    /// Pairwise records can also hold additional optional metadata.
    /// </remarks>
    public static class Pairwise
    {
        /// <summary>
        /// Gets the callback to use when the IsExistsAsync command completes.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(IsPairwiseExistsCompletedDelegate))]
#endif
        private static void IsPairwiseExistsCallbackMethod(int xcommand_handle, int err, bool exists)
        {
            var taskCompletionSource = PendingCommands.Remove<bool>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(exists);
        }
        private static IsPairwiseExistsCompletedDelegate IsPairwiseExistsCallback = IsPairwiseExistsCallbackMethod;

        /// <summary>
        /// Gets the callback to use when the ListAsync command completes.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(ListPairwiseCompletedDelegate))]
#endif
        private static void ListPairwiseCallbackMethod(int xcommand_handle, int err, string list_pairwise)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(list_pairwise);
        }
        private static ListPairwiseCompletedDelegate ListPairwiseCallback = ListPairwiseCallbackMethod;

        /// <summary>
        /// Gets the callback to use when the GetAsync command completes.
        /// </summary>
#if __IOS__
        [MonoPInvokeCallback(typeof(GetPairwiseCompletedDelegate))]
#endif
        private static void GetPairwiseCallbackMethod(int xcommand_handle, int err, string get_pairwise_json)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(get_pairwise_json);
        }
        private static GetPairwiseCompletedDelegate GetPairwiseCallback = GetPairwiseCallbackMethod;

        /// <summary>
        /// Gets whether or not a pairwise record exists in the provided wallet for the specified DID .
        /// </summary>
        /// <param name="wallet">The wallet to check for a pairwise record.</param>
        /// <param name="theirDid">The DID to check.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to true if a pairwise exists for the 
        /// DID, otherwise false.</returns>
        public static Task<bool> IsExistsAsync(Wallet wallet, string theirDid)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(theirDid, "theirDid");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = NativeMethods.indy_is_pairwise_exists(
                commandHandle,
                wallet.Handle,
                theirDid,
                IsPairwiseExistsCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Creates a new pairwise record between two specified DIDs in the provided wallet.
        /// </summary>
        /// <param name="wallet">The wallet to store create the pairwise record in.</param>
        /// <param name="theirDid">The DID of the remote party.</param>
        /// <param name="myDid">The DID belonging to the owner of the wallet.</param>
        /// <param name="metadata">Optional metadata to store with the record.</param>
        /// <returns>An asynchronous <see cref="Task"/> completes once the operation completes.</returns>
        public static Task CreateAsync(Wallet wallet, string theirDid, string myDid, string metadata)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(theirDid, "theirDid");
            ParamGuard.NotNullOrWhiteSpace(myDid, "myDid");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = NativeMethods.indy_create_pairwise(
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
        /// Lists all pairwise relationships stored in the specified wallet.
        /// </summary>
        /// <remarks>
        /// The JSON string that this method resolves to will contain a array of objects each of which
        /// describes a pairwise record for two DIDs, a DID belonging to the record owner (my_did) and the 
        /// associated DID belonging to the other party (their_did).
        /// 
        /// <code>
        /// [
        ///     {"my_did":"my_did_for_A","their_did":"A's_did_for_me"},
        ///     {"my_did":"my_did_for_B","their_did":"B's_did_for_me"}
        ///     ...
        /// ]
        /// </code>
        /// 
        /// Note that this call does not return any metadata associated with the pairwise records; to get the
        /// metadata use the <see cref="GetAsync(Wallet, string)"/> method.
        /// </remarks>
        /// <param name="wallet">The wallet to get the pairwise records from.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a JSON string containing
        /// an array of all pairwise relationships stored in the wallet.</returns>
        public static Task<string> ListAsync(Wallet wallet)
        {
            ParamGuard.NotNull(wallet, "wallet");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = NativeMethods.indy_list_pairwise(
                commandHandle,
                wallet.Handle,
                ListPairwiseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the pairwise record associated with the specified DID from the provided wallet.
        /// </summary>
        /// <remarks>
        /// The JSON string that this method resolves to will contain a single pairwise record for two DIDs, 
        /// the DID belonging to the record owner (my_did), the associated DID belonging to the other party 
        /// (their_did) and any metadata associated with the record (metadata).
        /// 
        /// <code>
        /// [
        ///     {"my_did":"my_did_for_A","their_did":"A's_did_for_me","metadata":"some metadata"},
        ///     {"my_did":"my_did_for_B","their_did":"B's_did_for_me"}
        ///     ...
        /// ]
        /// </code>
        /// 
        /// Note that if no metadata is present in a record the JSON will omit the <c>metadata</c>key.
        /// </remarks>
        /// <param name="wallet">The wallet to get the pairwise record from.</param>
        /// <param name="theirDid">The DID belonging to another party to get the pairwise record for.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a JSON string containing
        /// a pairwise record.</returns>
        public static Task<string> GetAsync(Wallet wallet, string theirDid)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(theirDid, "theirDid");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = NativeMethods.indy_get_pairwise(
                commandHandle,
                wallet.Handle,
                theirDid,
                GetPairwiseCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Sets the metadata on the existing pairwise record for the specified DID in the provided wallet.
        /// </summary>
        /// <remarks>
        /// If the pairwise record already contains any existing metadata it will be replaced with the value provided 
        /// in the <paramref name="metadata"/> parameter.  To remove all metadata for a record provide <c>null</c> in the
        /// <paramref name="metadata"/> parameter.
        /// </remarks>
        /// <param name="wallet">The wallet containing the pairwise record.</param>
        /// <param name="theirDid">The DID belonging to another party the pairwise record exists for.</param>
        /// <param name="metadata">The metadata to set on the pairwise record.</param>
        /// <returns>An asynchronous <see cref="Task"/> completes once the operation completes.</returns>
        public static Task SetMetadataAsync(Wallet wallet, string theirDid, string metadata)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(theirDid, "theirDid");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            int result = NativeMethods.indy_set_pairwise_metadata(
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
