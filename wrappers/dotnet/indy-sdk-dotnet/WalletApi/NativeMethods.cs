using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.WalletApi
{
    internal static class NativeMethods
    {
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_wallet(int command_handle, string config, string credentials, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_open_wallet(int command_handle, string config, string credentials, OpenWalletCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_export_wallet(int command_handle, int wallet_handle, string export_config, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_import_wallet(int command_handle, string config, string credentials, string import_config, IndyMethodCompletedDelegate cb);

        internal delegate void OpenWalletCompletedDelegate(int xcommand_handle, int err, int wallet_handle);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_close_wallet(int command_handle, int wallet_handle, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_delete_wallet(int command_handle, string config, string credentials, IndyMethodCompletedDelegate cb);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_generate_wallet_key(int command_handle, string config, GenerateWalletKeyCompletedDelegate cb);

        internal delegate void GenerateWalletKeyCompletedDelegate(int xcommand_handle, int err, string key);
    }
}
