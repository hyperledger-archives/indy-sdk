using System.Runtime.InteropServices;

namespace Hyperledger.Indy.BlobStorageApi
{
    internal static class NativeMethods
    {
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_open_blob_storage_reader(int command_handle, string type_, string config_json, BlobStorageCompletedDelegate cb);

        internal delegate void BlobStorageCompletedDelegate(int xcommand_handle, int err, int handle);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_open_blob_storage_writer(int command_handle, string type_, string config_json, BlobStorageCompletedDelegate cb);
    }
}
