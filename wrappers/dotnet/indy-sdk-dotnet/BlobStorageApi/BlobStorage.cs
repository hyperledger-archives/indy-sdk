using System;
using System.Threading.Tasks;
using Hyperledger.Indy.Utils;
using static Hyperledger.Indy.BlobStorageApi.NativeMethods;

namespace Hyperledger.Indy.BlobStorageApi
{
    /// <summary>
    /// BLOB storage.
    /// </summary>
    public static class BlobStorage
    {
        private static BlobStorageCompletedDelegate _openReaderCallback = (xcommand_handle, err, handle) =>
        {
            var taskCompletionSource = PendingCommands.Remove<BlobStorageReader>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new BlobStorageReader(handle));
        };

        private static BlobStorageCompletedDelegate _openWriterCallback = (xcommand_handle, err, handle) =>
        {
            var taskCompletionSource = PendingCommands.Remove<BlobStorageWriter>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(new BlobStorageWriter(handle));
        };

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
                _openReaderCallback
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
                _openWriterCallback
                );

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }
    }
}
