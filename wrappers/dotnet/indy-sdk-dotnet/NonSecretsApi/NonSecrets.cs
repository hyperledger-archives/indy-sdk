using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Threading.Tasks;
using static Hyperledger.Indy.NonSecretsApi.NativeMethods;
using static Hyperledger.Indy.Utils.CallbackHelper;
#if __IOS__
using ObjCRuntime;
#endif

namespace Hyperledger.Indy.NonSecretsApi
{
    /// <summary>
    /// Non secrets.
    /// </summary>
    public static class NonSecrets
    {
        #region Static callback methods

#if __IOS__
        [MonoPInvokeCallback(typeof(GetRecordCompletedDelegate))]
#endif
        private static void GetRecordCallbackMethod(int xcommand_handle, int err, string value)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(value);
        }
        private static GetRecordCompletedDelegate GetRecordCallback = GetRecordCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(OpenWalletSearchCompletedDelegate))]
#endif
        private static void OpenSearchCallbackMethod(int xcommand_handle, int err, int search_handle)
        {
            var taskCompletionSource = PendingCommands.Remove<WalletSearch>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new WalletSearch(search_handle));
        }
        private static OpenWalletSearchCompletedDelegate OpenSearchCallback = OpenSearchCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(FetchNextRecordCompletedDelegate))]
#endif
        private static void FetchNextCallbackMethod(int xcommand_handle, int err, string records_json)
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(records_json);
        }
        private static FetchNextRecordCompletedDelegate FetchNextCallback = FetchNextCallbackMethod;

        #endregion

        /// <summary>
        /// Create a new non-secret record in the wallet
        /// </summary>
        /// <returns>The record async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="type">Allows to separate different record types collections.</param>
        /// <param name="id">The id of record.</param>
        /// <param name="value">The value of record.</param>
        /// <param name="tagsJson">
        /// the record tags used for search and storing meta information as json:
        /// <code>
        ///   {
        ///     "tagName1": &lt;str>, // string tag (will be stored encrypted)
        ///     "tagName2": &lt;str>, // string tag (will be stored encrypted)
        ///     "~tagName3": &lt;str>, // string tag (will be stored un-encrypted)
        ///     "~tagName4": &lt;str>, // string tag (will be stored un-encrypted)
        ///   }
        /// </code>
        ///   Note that null means no tags
        ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
        ///   usage of this tag in complex search queries (comparison, predicates)
        ///   Encrypted tags can be searched only for exact matching
        /// </param>
        public static Task AddRecordAsync(Wallet wallet, string type, string id, string value, string tagsJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(type, "type");
            ParamGuard.NotNullOrWhiteSpace(id, "id");
            ParamGuard.NotNullOrWhiteSpace(value, "value");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_add_wallet_record(
                commandHandle,
                wallet.Handle,
                type,
                id,
                value,
                tagsJson,
                CallbackHelper.TaskCompletingNoValueCallback);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Update a non-secret wallet record value
        /// </summary>
        /// <returns>The record async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="type">Allows to separate different record types collections.</param>
        /// <param name="id">The id of record.</param>
        /// <param name="value">The value of record.</param>
        public static Task UpdateRecordValueAsync(Wallet wallet, string type, string id, string value)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(type, "type");
            ParamGuard.NotNullOrWhiteSpace(id, "id");
            ParamGuard.NotNullOrWhiteSpace(value, "value");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_update_wallet_record_value(
                commandHandle,
                wallet.Handle,
                type,
                id,
                value,
                CallbackHelper.TaskCompletingNoValueCallback);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Update a non-secret wallet record tags
        /// </summary>
        /// <returns>The record async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="type">Allows to separate different record types collections.</param>
        /// <param name="id">The id of record.</param>
        /// <param name="tagsJson">
        /// the record tags used for search and storing meta information as json:
        /// <code>
        ///   {
        ///     "tagName1": &lt;str>, // string tag (will be stored encrypted)
        ///     "tagName2": &lt;str>, // string tag (will be stored encrypted)
        ///     "~tagName3": &lt;str>, // string tag (will be stored un-encrypted)
        ///     "~tagName4": &lt;str>, // string tag (will be stored un-encrypted)
        ///   }
        /// </code>
        ///   Note that null means no tags
        ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
        ///   usage of this tag in complex search queries (comparison, predicates)
        ///   Encrypted tags can be searched only for exact matching
        /// </param>
        public static Task UpdateRecordTagsAsync(Wallet wallet, string type, string id, string tagsJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(type, "type");
            ParamGuard.NotNullOrWhiteSpace(id, "id");
            ParamGuard.NotNullOrWhiteSpace(tagsJson, "tagsJson");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_update_wallet_record_tags(
                commandHandle,
                wallet.Handle,
                type,
                id,
                tagsJson,
                CallbackHelper.TaskCompletingNoValueCallback);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Add new tags to the wallet record
        /// </summary>
        /// <returns>The record async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="type">Allows to separate different record types collections.</param>
        /// <param name="id">The id of record.</param>
        /// <param name="tagsJson">
        /// the record tags used for search and storing meta information as json:
        /// <code>
        ///   {
        ///     "tagName1": &lt;str>, // string tag (will be stored encrypted)
        ///     "tagName2": &lt;str>, // string tag (will be stored encrypted)
        ///     "~tagName3": &lt;str>, // string tag (will be stored un-encrypted)
        ///     "~tagName4": &lt;str>, // string tag (will be stored un-encrypted)
        ///   }
        /// </code>
        ///   Note that null means no tags
        ///   If tag name starts with "~" the tag will be stored un-encrypted that will allow
        ///   usage of this tag in complex search queries (comparison, predicates)
        ///   Encrypted tags can be searched only for exact matching
        /// </param>
        public static Task AddRecordTagsAsync(Wallet wallet, string type, string id, string tagsJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(type, "type");
            ParamGuard.NotNullOrWhiteSpace(id, "id");
            ParamGuard.NotNullOrWhiteSpace(tagsJson, "tagsJson");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_add_wallet_record_tags(
                commandHandle,
                wallet.Handle,
                type,
                id,
                tagsJson,
                CallbackHelper.TaskCompletingNoValueCallback);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Delete tags from the wallet record
        /// </summary>
        /// <returns>The record tags async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="type">Allows to separate different record types collections.</param>
        /// <param name="id">The id of record.</param>
        /// <param name="tagsJson">the list of tag names to remove from the record as json array:
        /// <c>  ["tagName1", "tagName2", ...]</c></param>
        public static Task DeleteRecordTagsAsync(Wallet wallet, string type, string id, string tagsJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(type, "type");
            ParamGuard.NotNullOrWhiteSpace(id, "id");
            ParamGuard.NotNullOrWhiteSpace(tagsJson, "tagsJson");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_delete_wallet_record_tags(
                commandHandle,
                wallet.Handle,
                type,
                id,
                tagsJson,
                CallbackHelper.TaskCompletingNoValueCallback);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Delete an existing wallet record in the wallet
        /// </summary>
        /// <returns>The record async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="type">Type.</param>
        /// <param name="id">Identifier.</param>
        public static Task DeleteRecordAsync(Wallet wallet, string type, string id)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(type, "type");
            ParamGuard.NotNullOrWhiteSpace(id, "id");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_delete_wallet_record(
                commandHandle,
                wallet.Handle,
                type,
                id,
                CallbackHelper.TaskCompletingNoValueCallback);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Get a wallet record by id.
        /// </summary>
        /// <returns>The record async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="type">Type.</param>
        /// <param name="id">Identifier.</param>
        /// <param name="optionsJson">
        /// <code>
        ///  {
        ///    retrieveType: (optional, false by default) Retrieve record type,
        ///    retrieveValue: (optional, true by default) Retrieve record value,
        ///    retrieveTags: (optional, false by default) Retrieve record tags
        ///  }
        /// </code>
        /// </param>
        public static Task<string> GetRecordAsync(Wallet wallet, string type, string id, string optionsJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(type, "type");
            ParamGuard.NotNullOrWhiteSpace(id, "id");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_get_wallet_record(
                commandHandle,
                wallet.Handle,
                type,
                id,
                optionsJson,
                GetRecordCallback);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Search for wallet records.
        ///
        /// Note instead of immediately returning of fetched records
        /// this call returns wallet_search_handle that can be used later
        /// to fetch records by small batches (with indy_fetch_wallet_search_next_records).
        /// </summary>
        /// <returns>The search async.</returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="type">Type.</param>
        /// <param name="queryJson">
        /// MongoDB style query to wallet record tags:
        /// <code>
        ///  {
        ///    "tagName": "tagValue",
        ///    $or: {
        ///      "tagName2": { $regex: 'pattern' },
        ///      "tagName3": { $gte: '123' },
        ///    },
        ///  }
        /// </code>
        /// </param>
        /// <param name="optionsJson">
        /// <code>
        /// {
        ///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
        ///    retrieveTotalCount: (optional, false by default) Calculate total count,
        ///    retrieveType: (optional, false by default) Retrieve record type,
        ///    retrieveValue: (optional, true by default) Retrieve record value,
        ///    retrieveTags: (optional, false by default) Retrieve record tags,
        ///  }
        /// </code></param>
        public static Task<WalletSearch> OpenSearchAsync(Wallet wallet, string type, string queryJson, string optionsJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(type, "type");
            ParamGuard.NotNullOrWhiteSpace(queryJson, "queryJson");
            ParamGuard.NotNullOrWhiteSpace(optionsJson, "optionsJson");

            var taskCompletionSource = new TaskCompletionSource<WalletSearch>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_open_wallet_search(
                commandHandle,
                wallet.Handle,
                type,
                queryJson,
                optionsJson,
                OpenSearchCallback);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Fetch next records for wallet search.
        /// </summary>
        /// <returns>
        /// <code>
        /// {
        ///   totalCount: &lt;str>, // present only if retrieveTotalCount set to true
        ///   records: [{ // present only if retrieveRecords set to true
        ///       id: "Some id",
        ///       type: "Some type", // present only if retrieveType set to true
        ///       value: "Some value", // present only if retrieveValue set to true
        ///       tags: &lt;tags json>, // present only if retrieveTags set to true
        ///   }],
        /// }
        /// </code>
        /// </returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="walletSearch">Wallet search handle (created by <see cref="OpenSearchAsync(Wallet, string, string, string)"/>).</param>
        /// <param name="count">Count of records to fetch.</param>
        public static Task<string> FetchNextRecordsAsync(Wallet wallet, WalletSearch walletSearch, int count)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNull(walletSearch, "walletSearch");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_fetch_wallet_search_next_records(
                commandHandle,
                wallet.Handle,
                walletSearch.Handle,
                count,
                FetchNextCallback);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Close wallet search (make search handle invalid)
        /// </summary>
        /// <returns>The wallet search async.</returns>
        /// <param name="walletSearch">Wallet search.</param>
        public static Task CloseWalletSearchAsync(WalletSearch walletSearch)
        {
            ParamGuard.NotNull(walletSearch, "walletSearch");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_close_wallet_search(
                commandHandle,
                walletSearch.Handle,
                CallbackHelper.TaskCompletingNoValueCallback);

            return taskCompletionSource.Task;
        }
    }
}
