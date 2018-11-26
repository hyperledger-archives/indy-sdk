using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.NonSecretsApi
{
    static class NativeMethods
    {
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_add_wallet_record(int command_handle, int wallet_handle, string type_, string id, string value, string tags_json, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_update_wallet_record_value(int command_handle, int wallet_handle, string type_, string id, string value, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_update_wallet_record_tags(int command_handle, int wallet_handle, string type_, string id, string tags_json, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_add_wallet_record_tags(int command_handle, int wallet_handle, string type_, string id, string tags_json, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_delete_wallet_record_tags(int command_handle, int wallet_handle, string type_, string id, string tag_names_json, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_delete_wallet_record(int command_handle, int wallet_handle, string type_, string id, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_wallet_record(int command_handle, int wallet_handle, string type_, string id, string options_json, GetRecordCompletedDelegate cb);

        internal delegate void GetRecordCompletedDelegate(int xcommand_handle, int err, string value);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_open_wallet_search(int command_handle, int wallet_handle, string type_, string query_json, string options_json, OpenWalletSearchCompletedDelegate cb);

        internal delegate void OpenWalletSearchCompletedDelegate(int xcommand_handle, int err, int search_handle);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_fetch_wallet_search_next_records(int command_handle, int wallet_handle, int wallet_search_handle, int count, FetchNextRecordCompletedDelegate cb);

        internal delegate void FetchNextRecordCompletedDelegate(int xcommand_handle, int err, string records_json);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_close_wallet_search(int command_handle, int wallet_search_handle, IndyMethodCompletedDelegate cb);
    }
}
