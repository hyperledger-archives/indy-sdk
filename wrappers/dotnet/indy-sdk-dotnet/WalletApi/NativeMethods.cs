using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.WalletApi
{
    internal static class NativeMethods
    {
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false,
            ThrowOnUnmappableChar = true)]
        internal static extern int indy_register_wallet_storage(int command_handle, string type_,
            WalletCreateDelegate create,
            WalletOpenDelegate open,
            WalletCloseDelegate close,
            WalletDeleteDelegate delete,
            WalletAddRecordDelegate add_record,
            WalletUpdateRecordValueDelegate update_record_value,
            WalletUpdateRecordTagsDelegate update_record_tags,
            WalletAddRecordTagsDelegate add_record_tags,
            WalletDeleteRecordTagsDelegate delete_record_tags,
            WalletDeleteRecordDelegate delete_record,
            WalletGetRecordDelegate get_record,
            WalletGetRecordIdDelegate get_record_id,
            WalletGetRecordTypeDelegate get_record_type,
            WalletGetRecordValueDelegate get_record_value,
            WalletGetRecordTagsDelegate get_record_tags,
            WalletFreeRecordDelegate free_record,
            WalletGetStorageMetadataDelegate get_storage_metadata,
            WalletSetStorageMetadataDelegate set_storage_metadata,
            WalletFreeStorageMetadataDelegate free_storage_metadata,
            WalletSearchRecordsDelegate search_records,
            WalletSearchAllRecordsDelegate search_all_records,
            WalletGetSearchTotalCountDelegate get_search_total_count,
            WalletFetchSearchNextRecordDelegate fetch_search_next_record,
            WalletFreeSearchDelegate free_search,
            IndyMethodCompletedDelegate cb);

        internal delegate ErrorCode WalletCreateDelegate(string name, string config, string credentials_json,
            string metadata);

        internal delegate ErrorCode WalletOpenDelegate(string name, string config, string credentials_json,
            ref int storage_handle_p);

        internal delegate ErrorCode WalletCloseDelegate(int storage_handle);

        internal delegate ErrorCode WalletDeleteDelegate(string name, string config, string credentials_json);

        internal delegate ErrorCode WalletAddRecordDelegate(int storage_handle, string type_, string id, IntPtr value,
            int value_len, string tags_json);

        internal delegate ErrorCode WalletUpdateRecordValueDelegate(int storage_handle, string type_, string id,
            IntPtr value, int value_len);

        internal delegate ErrorCode WalletUpdateRecordTagsDelegate(int storage_handle, string type_, string id,
            string tags_json);

        internal delegate ErrorCode WalletAddRecordTagsDelegate(int storage_handle, string type_, string id,
            string tags_json);

        internal delegate ErrorCode WalletDeleteRecordTagsDelegate(int storage_handle, string type_, string id,
            string tag_names_json);

        internal delegate ErrorCode WalletDeleteRecordDelegate(int storage_handle, string type_, string id);

        internal delegate ErrorCode WalletGetRecordDelegate(int storage_handle, string type_, string id,
            string options_json, ref int record_handle_p);

        internal delegate ErrorCode WalletGetRecordIdDelegate(int storage_handle, int record_handle,
            ref string record_id_p);

        internal delegate ErrorCode WalletGetRecordTypeDelegate(int storage_handle, int record_handle,
            ref string record_type_p);

        internal delegate ErrorCode WalletGetRecordValueDelegate(int storage_handle, int record_handle,
            ref IntPtr record_value_p, ref IntPtr record_value_len_p);

        internal delegate ErrorCode WalletGetRecordTagsDelegate(int storage_handle, int record_handle,
            ref string record_tags_p);

        internal delegate ErrorCode WalletFreeRecordDelegate(int storage_handle, int record_handle);

        internal delegate ErrorCode WalletGetStorageMetadataDelegate(int storage_handle, ref string metadata_p,
            ref int metadata_handle);

        internal delegate ErrorCode WalletSetStorageMetadataDelegate(int storage_handle, string metadata_p);

        internal delegate ErrorCode WalletFreeStorageMetadataDelegate(int stroage_handle, int metadata_handle);

        internal delegate ErrorCode WalletSearchRecordsDelegate(int storage_handle, string type_, string query_json,
            string options_json, ref int search_handle_p);

        internal delegate ErrorCode WalletSearchAllRecordsDelegate(int storage_handle, ref int search_handle_p);

        internal delegate ErrorCode WalletGetSearchTotalCountDelegate(int storage_handle, int search_handle,
            ref IntPtr total_count_p);

        internal delegate ErrorCode WalletFetchSearchNextRecordDelegate(int storage_handle, int search_handle,
            ref int record_handle_p);

        internal delegate ErrorCode WalletFreeSearchDelegate(int storage_handle, int search_handle);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false,
            ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_wallet(int command_handle, string config, string credentials,
            IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false,
            ThrowOnUnmappableChar = true)]
        internal static extern int indy_open_wallet(int command_handle, string config, string credentials,
            OpenWalletCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false,
            ThrowOnUnmappableChar = true)]
        internal static extern int indy_export_wallet(int command_handle, IntPtr wallet_handle, string export_config,
            IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false,
            ThrowOnUnmappableChar = true)]
        internal static extern int indy_import_wallet(int command_handle, string config, string credentials,
            string import_config, IndyMethodCompletedDelegate cb);

        internal delegate void OpenWalletCompletedDelegate(int xcommand_handle, int err, IntPtr wallet_handle);

        /// <summary>
        /// Closes opened wallet and frees allocated resources.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle returned by indy_open_wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false,
            ThrowOnUnmappableChar = true)]
        internal static extern int indy_close_wallet(int command_handle, IntPtr wallet_handle,
            IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Deletes created wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="config">Name of the wallet to delete.</param>
        /// <param name="credentials">Wallet credentials json</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false,
            ThrowOnUnmappableChar = true)]
        internal static extern int indy_delete_wallet(int command_handle, string config, string credentials,
            IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false,
            ThrowOnUnmappableChar = true)]
        internal static extern int indy_generate_wallet_key(int command_handle, string config,
            GenerateWalletKeyCompletedDelegate cb);

        internal delegate void GenerateWalletKeyCompletedDelegate(int xcommand_handle, int err, string key);

    }
}