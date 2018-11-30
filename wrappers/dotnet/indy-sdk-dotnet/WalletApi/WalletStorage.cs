using System;
using System.Diagnostics.CodeAnalysis;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.WalletApi.NativeMethods;
#if __IOS__
using ObjCRuntime;
#endif

/* Due to limitations in the Mono runtime explained here https://docs.microsoft.com/en-us/xamarin/ios/internals/limitations#Reverse_Callbacks
 * iOS cannot support multiple plugged storage implementations. Callback methods must be marked static, and therefore cannot be used
 * for different instances of plugged storage. This limitation only affects iOS runtime. 
 */
namespace Hyperledger.Indy.WalletApi
{
    [SuppressMessage("ReSharper", "InconsistentNaming")]
    internal class WalletStorage
    {
#if __IOS__
        private static IWalletStorage _storage;
#else
        private readonly IWalletStorage _storage;
#endif

        public WalletStorage(IWalletStorage storage)
        {
            _storage = storage;

            WalletCreateCallback = WalletCreateHandler;
            WalletOpenCallback = WalletOpenHandler;
            WalletCloseCallback = WalletCloseHandler;
            WalletDeleteCallback = WalletDeleteHandler;
            WalletAddRecordCallback = WalletAddRecordHandler;
            WalletUpdateRecordValueCallback = WalletUpdateRecordValueHandler;
            WalletUpdateRecordTagsCallback = WalletUpdateRecordTagsHandler;
            WalletAddRecordTagsCallback = WalletAddRecordTagsHandler;
            WalletDeleteRecordTagsCallback = WalletDeleteRecordTagsHandler;
            WalletDeleteRecordCallback = WalletDeleteRecordHandler;
            WalletGetRecordCallback = WalletGetRecordHandler;
            WalletGetRecordIdCallback = WalletGetRecordIdHandler;
            WalletGetRecordTypeCallback = WalletGetRecordTypeHandler;
            WalletGetRecordValueCallback = WalletGetRecordValueHandler;
            WalletGetRecordTagsCallback = WalletGetRecordTagsHandler;
            WalletFreeRecordCallback = WalletFreeRecordHandler;
            WalletGetStorageMetadataCallback = WalletGetStorageMetadataHandler;
            WalletSetStorageMetadataCallback = WalletSetStorageMetadataHandler;
            WalletFreeStorageMetadataCallback = WalletFreeStorageMetadataHandler;
            WalletSearchRecordsCallback = WalletSearchRecordsHandler;
            WalletSearchAllRecordsCallback = WalletSearchAllRecordsHandler;
            WalletGetSearchTotalCountCallback = WalletGetSearchTotalCountHandler;
            WalletFetchSearchNextRecordCallback = WalletFetchSearchNextRecordHandler;
            WalletFreeSearchCallback = WalletFreeSearchHandler;
        }

        public WalletCreateDelegate WalletCreateCallback { get; }

        public WalletOpenDelegate WalletOpenCallback { get; }

        public WalletCloseDelegate WalletCloseCallback { get; }

        public WalletDeleteDelegate WalletDeleteCallback { get; }

        public WalletAddRecordDelegate WalletAddRecordCallback { get; }

        public WalletUpdateRecordValueDelegate WalletUpdateRecordValueCallback { get; }

        public WalletUpdateRecordTagsDelegate WalletUpdateRecordTagsCallback { get; }

        public WalletAddRecordTagsDelegate WalletAddRecordTagsCallback { get; }

        public WalletDeleteRecordTagsDelegate WalletDeleteRecordTagsCallback { get; }

        public WalletDeleteRecordDelegate WalletDeleteRecordCallback { get; }

        public WalletGetRecordDelegate WalletGetRecordCallback { get; }

        public WalletGetRecordIdDelegate WalletGetRecordIdCallback { get; }

        public WalletGetRecordTypeDelegate WalletGetRecordTypeCallback { get; }

        public WalletGetRecordValueDelegate WalletGetRecordValueCallback { get; }

        public WalletGetRecordTagsDelegate WalletGetRecordTagsCallback { get; }

        public WalletFreeRecordDelegate WalletFreeRecordCallback { get; }

        public WalletGetStorageMetadataDelegate WalletGetStorageMetadataCallback { get; }

        public WalletSetStorageMetadataDelegate WalletSetStorageMetadataCallback { get; }

        public WalletFreeStorageMetadataDelegate WalletFreeStorageMetadataCallback { get; }

        public WalletSearchRecordsDelegate WalletSearchRecordsCallback { get; }

        public WalletSearchAllRecordsDelegate WalletSearchAllRecordsCallback { get; }

        public WalletGetSearchTotalCountDelegate WalletGetSearchTotalCountCallback { get; }

        public WalletFetchSearchNextRecordDelegate WalletFetchSearchNextRecordCallback { get; }

        public WalletFreeSearchDelegate WalletFreeSearchCallback { get; }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletCreateDelegate))]
        static
#endif
        private ErrorCode WalletCreateHandler(string name, string config, string credentials_json, string metadata)
        {
            try
            {
                _storage.CreateAsync(name, config, credentials_json, metadata).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletOpenDelegate))]
        static
#endif
        private ErrorCode WalletOpenHandler(string name, string config, string credentials_json,
            ref int storage_handle_p)
        {
            try
            {
                storage_handle_p = _storage.OpenAsync(name, config, credentials_json).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletCloseDelegate))]
        static
#endif
        private ErrorCode WalletCloseHandler(int storage_handle)
        {
            try
            {
                _storage.CloseAsync(storage_handle).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletDeleteDelegate))]
        static
#endif
        private ErrorCode WalletDeleteHandler(string name, string config, string credentials_json)
        {
            try
            {
                _storage.DeleteAsync(name, config, credentials_json).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletAddRecordDelegate))]
        static
#endif
        private ErrorCode WalletAddRecordHandler(int storage_handle, string type_, string id, IntPtr value,
            int value_len, string tags_json)
        {
            try
            {
                var valueBytes = new byte[value_len];
                Marshal.Copy(value, valueBytes, 0, value_len);

                _storage.AddRecordAsync(storage_handle, type_, id, valueBytes, tags_json).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletUpdateRecordValueDelegate))]
        static
#endif
        private ErrorCode WalletUpdateRecordValueHandler(int storage_handle, string type_, string id, IntPtr value,
            int value_len)
        {
            try
            {
                var valueBytes = new byte[value_len];
                Marshal.Copy(value, valueBytes, 0, value_len);

                _storage.UpdateRecordValueAsync(storage_handle, type_, id, valueBytes).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletUpdateRecordTagsDelegate))]
        static
#endif
        private ErrorCode WalletUpdateRecordTagsHandler(int storage_handle, string type_, string id, string tags_json)
        {
            try
            {
                _storage.UpdateRecordTagsAsync(storage_handle, type_, id, tags_json).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletAddRecordTagsDelegate))]
        static
#endif
        private ErrorCode WalletAddRecordTagsHandler(int storage_handle, string type_, string id, string tags_json)
        {
            try
            {
                _storage.AddRecordTagsAsync(storage_handle, type_, id, tags_json).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletDeleteRecordTagsDelegate))]
        static
#endif
        private ErrorCode WalletDeleteRecordTagsHandler(int storage_handle, string type_, string id,
            string tag_names_json)
        {
            try
            {
                _storage.DeleteRecordTagsAsync(storage_handle, type_, id, tag_names_json).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletDeleteRecordDelegate))]
        static
#endif
        private ErrorCode WalletDeleteRecordHandler(int storage_handle, string type_, string id)
        {
            try
            {
                _storage.DeleteRecordAsync(storage_handle, type_, id).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletGetRecordDelegate))]
        static
#endif
        private ErrorCode WalletGetRecordHandler(int storage_handle, string type_, string id,
            string options_json, ref int record_handle_p)
        {
            try
            {
                record_handle_p = _storage.GetRecordAsync(storage_handle, type_, id, options_json).GetAwaiter()
                    .GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletItemNotFoundError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletGetRecordIdDelegate))]
        static
#endif
        private ErrorCode WalletGetRecordIdHandler(int storage_handle, int record_handle,
            ref string record_id_p)
        {
            try
            {
                record_id_p = _storage.GetRecordIdAsync(storage_handle, record_handle).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletGetRecordTypeDelegate))]
        static
#endif
        private ErrorCode WalletGetRecordTypeHandler(int storage_handle, int record_handle,
            ref string record_type_p)
        {
            try
            {
                record_type_p = _storage.GetRecordTypeAsync(storage_handle, record_handle).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletGetRecordValueDelegate))]
        static
#endif
        private ErrorCode WalletGetRecordValueHandler(int storage_handle, int record_handle, ref IntPtr record_value_p,
            ref IntPtr record_value_len_p)
        {
            try
            {
                var buffer = _storage.GetRecordValueAsync(storage_handle, record_handle).GetAwaiter().GetResult();

                IntPtr unmanagedMemoryPtr = Marshal.AllocHGlobal(buffer.Length);
                Marshal.Copy(buffer, 0, unmanagedMemoryPtr, buffer.Length);

                record_value_p = unmanagedMemoryPtr;
                record_value_len_p = new IntPtr(buffer.Length);

                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletGetRecordTagsDelegate))]
        static
#endif
        private ErrorCode WalletGetRecordTagsHandler(int storage_handle, int record_handle,
            ref string record_tags_p)
        {
            try
            {
                record_tags_p = _storage.GetRecordTagsAsync(storage_handle, record_handle).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletFreeRecordDelegate))]
        static
#endif
        private ErrorCode WalletFreeRecordHandler(int storage_handle, int record_handle)
        {
            try
            {
                _storage.FreeRecordAsync(storage_handle, record_handle).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletGetStorageMetadataDelegate))]
        static
#endif
        private ErrorCode WalletGetStorageMetadataHandler(int storage_handle, ref string metadata_p,
            ref int metadata_handle)
        {
            try
            {
                var result = _storage.GetStorageMetadataAsync(storage_handle).GetAwaiter().GetResult();
                metadata_p = result.Item1;
                metadata_handle = result.Item2;

                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletSetStorageMetadataDelegate))]
        static
#endif
        private ErrorCode WalletSetStorageMetadataHandler(int storage_handle, string metadata_p)
        {
            try
            {
                _storage.SetStorageMetadataAsync(storage_handle, metadata_p).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletFreeStorageMetadataDelegate))]
        static
#endif
        private ErrorCode WalletFreeStorageMetadataHandler(int storage_handle, int metadata_handle)
        {
            try
            {
                _storage.FreeStorageMetadataAsync(storage_handle, metadata_handle).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletSearchRecordsDelegate))]
        static
#endif
        private ErrorCode WalletSearchRecordsHandler(int storage_handle, string type_, string query_json,
            string options_json, ref int search_handle_p)
        {
            try
            {
                search_handle_p = _storage.SearchRecordsAsync(storage_handle, type_, query_json, options_json)
                    .GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletSearchAllRecordsDelegate))]
        static
#endif
        private ErrorCode WalletSearchAllRecordsHandler(int storage_handle, ref int search_handle_p)
        {
            try
            {
                search_handle_p = _storage.SearchAllRecordsAsync(storage_handle).GetAwaiter().GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletGetSearchTotalCountDelegate))]
        static
#endif
        private ErrorCode WalletGetSearchTotalCountHandler(int storage_handle, int search_handle, ref IntPtr total_count_p)
        {
            try
            {
                total_count_p = new IntPtr(_storage.GetSearchTotalCountAsync(storage_handle, search_handle).GetAwaiter()
                    .GetResult());
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletFetchSearchNextRecordDelegate))]
        static
#endif
        private ErrorCode WalletFetchSearchNextRecordHandler(int storage_handle, int search_handle,
            ref int record_handle_p)
        {
            try
            {
                record_handle_p = _storage.FetchSearchNextRecordAsync(storage_handle, search_handle).GetAwaiter()
                    .GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletItemNotFoundError;
            }
        }

#if __IOS__
        [MonoPInvokeCallback(typeof(WalletFreeSearchDelegate))]
        static
#endif
        private ErrorCode WalletFreeSearchHandler(int storage_handle, int search_handle)
        {
            try
            {
                _storage.FreeSearchAsync(storage_handle, search_handle).GetAwaiter()
                    .GetResult();
                return ErrorCode.Success;
            }
            catch
            {
                return ErrorCode.WalletStorageError;
            }
        }
    }
}