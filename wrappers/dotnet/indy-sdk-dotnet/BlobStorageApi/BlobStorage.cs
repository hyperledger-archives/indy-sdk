using Hyperledger.Indy.Utils;
using System.Threading.Tasks;
using static Hyperledger.Indy.BlobStorageApi.NativeMethods;
#if __IOS__
using ObjCRuntime;
#endif

namespace Hyperledger.Indy.BlobStorageApi
{
    /// <summary>
    /// BLOB storage.
    /// </summary>
    public static class BlobStorage
    {
#if __IOS__
        [MonoPInvokeCallback(typeof(BlobStorageCompletedDelegate))]
#endif
        private static void OpenReaderCallbackMethod(int xcommand_handle, int err, int handle)
        {
            var taskCompletionSource = PendingCommands.Remove<BlobStorageReader>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new BlobStorageReader(handle));
        }
        private static BlobStorageCompletedDelegate OpenReaderCallback = OpenReaderCallbackMethod;

#if __IOS__
        [MonoPInvokeCallback(typeof(BlobStorageCompletedDelegate))]
#endif
        private static void OpenWriterCallbackMethod(int xcommand_handle, int err, int handle)
        {
            var taskCompletionSource = PendingCommands.Remove<BlobStorageWriter>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new BlobStorageWriter(handle));
        }
        private static BlobStorageCompletedDelegate OpenWriterCallback = OpenWriterCallbackMethod;

        /// <summary>
        /// Opens the BLOB storage reader async.
        /// </summary>
        /// <returns>The BLOB storage reader async.</returns>
        /// <param name="type">Type.</param>
        /// <param name="configJson">Config json.</param>
        public static Task<BlobStorageReader> OpenReaderAsync(string type, string configJson)
        {
            ParamGuard.NotNullOrWhiteSpace(type, "type");
            ParamGuard.NotNullOrWhiteSpace(configJson, "configJson");

            var taskCompletionSource = new TaskCompletionSource<BlobStorageReader>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_open_blob_storage_reader(
                commandHandle,
                type,
                configJson,
                OpenReaderCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Opens the BLOB storage writer async.
        /// </summary>
        /// <returns>The BLOB storage writer async.</returns>
        /// <param name="type">Type.</param>
        /// <param name="configJson">Config json.</param>
        public static Task<BlobStorageWriter> OpenWriterAsync(string type, string configJson)
        {
            ParamGuard.NotNullOrWhiteSpace(type, "type");
            ParamGuard.NotNullOrWhiteSpace(configJson, "configJson");

            var taskCompletionSource = new TaskCompletionSource<BlobStorageWriter>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_open_blob_storage_writer(
                commandHandle,
                type,
                configJson,
                OpenWriterCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }
    }
}
