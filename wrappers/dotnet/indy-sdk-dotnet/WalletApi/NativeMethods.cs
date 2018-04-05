using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.WalletApi
{
    internal static class NativeMethods
    {
        /// <summary>
        /// Registers custom wallet implementation.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="xtype">Wallet type name.</param>
        /// <param name="create">WalletType create operation handler</param>
        /// <param name="open">WalletType open operation handler</param>
        /// <param name="set">Wallet set operation handler</param>
        /// <param name="get">Wallet get operation handler</param>
        /// <param name="get_not_expired">Wallet get_not_expired operation handler</param>
        /// <param name="list">Wallet list operation handler</param>
        /// <param name="close">Wallet close operation handler</param>
        /// <param name="delete">WalletType delete operation handler</param>
        /// <param name="free">Handler that allows to de-allocate strings allocated in caller code</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_register_wallet_type(int command_handle, string xtype, WalletTypeCreateDelegate create, WalletTypeOpenDelegate open, WalletTypeSetDelegate set, WalletTypeGetDelegate get, WalletTypeGetNotExpiredDelegate get_not_expired, WalletTypeListDelegate list, WalletTypeCloseDelegate close, WalletTypeDeleteDelegate delete, WalletTypeFreeDelegate free, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Delegate for the function called back to when a wallet of a custom type is created.
        /// </summary>
        /// <param name="name">The name of the wallet.</param>
        /// <param name="config">The configuration of the wallet.</param>
        /// <param name="credentials">The credentials for the wallet.</param>
        internal delegate ErrorCode WalletTypeCreateDelegate(string name, string config, string credentials);

        /// <summary>
        /// Delegate for the function called back to when a wallet of a custom type is opened.
        /// </summary>
        /// <param name="name">The name of the wallet to open.</param>
        /// <param name="config">The configuration for the wallet.</param>
        /// <param name="runtime_config">The runtime configuration for the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        /// <param name="handle">A handle to use when tracking the wallet instance.</param>
        internal delegate ErrorCode WalletTypeOpenDelegate(string name, string config, string runtime_config, string credentials, ref int handle);

        /// <summary>
        /// Delegate for the function called back to when value is set on a wallet of a custom type.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance the action is being performed on.</param>
        /// <param name="key">The key of the value to set.</param>
        /// <param name="value">The value to set.</param>
        internal delegate ErrorCode WalletTypeSetDelegate(int handle, string key, string value);

        /// <summary>
        /// Delegate for the function called back to when value is requested from a wallet of a custom type.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance the action is being performed on.</param>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value_ptr">The pointer to the value associated with the key.</param>
        internal delegate ErrorCode WalletTypeGetDelegate(int handle, string key, ref IntPtr value_ptr);

        /// <summary>
        /// Delegate for the function called back to when an unexpired value is requested from a wallet of a custom type.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance the action is being performed on.</param>
        /// <param name="key">The key of the value to get.</param>
        /// <param name="value_ptr">The pointer to the value associated with the key.</param>
        internal delegate ErrorCode WalletTypeGetNotExpiredDelegate(int handle, string key, ref IntPtr value_ptr);

        /// <summary>
        /// Delegate for the function called back to when an list of values is requested from a wallet of a custom type.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance the action is being performed on.</param>
        /// <param name="keyPrefix">The key prefix for the values requested.</param>
        /// <param name="values_json_ptr">The pointer to the values associated with the key prefix.</param>
        internal delegate ErrorCode WalletTypeListDelegate(int handle, string keyPrefix, ref IntPtr values_json_ptr);

        /// <summary>
        /// Delegate for the function called back to when a wallet of a custom type is closed.
        /// </summary>
        /// <param name="handle">The handle of the wallet instance the action is being performed on.</param>
        internal delegate ErrorCode WalletTypeCloseDelegate(int handle);

        /// <summary>
        /// Delegate for the function called back to when a wallet of a custom type is deleted.
        /// </summary>
        /// <param name="name">The name of the wallet being deleted</param>
        /// <param name="config">The configuration of the wallet.</param>
        /// <param name="credentials">The credentials of the wallet.</param>
        internal delegate ErrorCode WalletTypeDeleteDelegate(string name, string config, string credentials);

        /// <summary>
        /// Delegate for the function called back to when a value in a  wallet of a custom type is freed.
        /// </summary>
        /// <param name="handle">The handle of the wallet the action is being performed on.</param>
        /// <param name="value">A pointer to the value to be freed.</param>
        internal delegate ErrorCode WalletTypeFreeDelegate(int handle, IntPtr value);


        /// <summary>
        /// Creates a new secure wallet with the given unique name.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="pool_name">Name of the pool that corresponds to this wallet.</param>
        /// <param name="name">Name of the wallet.</param>
        /// <param name="xtype">Type of the wallet. Defaults to 'default'.</param>
        /// <param name="config">Wallet configuration json.</param>
        /// <param name="credentials">Wallet credentials json. </param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_wallet(int command_handle, string pool_name, string name, string xtype, string config, string credentials, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Opens the wallet with specific name.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="name">Name of the wallet.</param>
        /// <param name="runtime_config">Runtime wallet configuration json. if NULL, then default runtime_config will be used. </param>
        /// <param name="credentials">Wallet credentials json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <remarks>
        /// Wallet with corresponded name must be previously created with indy_create_wallet method.
        /// It is impossible to open wallet with the same name more than once.
        /// </remarks>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_open_wallet(int command_handle, string name, string runtime_config, string credentials, OpenWalletCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_open_wallet.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="wallet_handle">The handle for the opened wallet.</param>
        internal delegate void OpenWalletCompletedDelegate(int xcommand_handle, int err, IntPtr wallet_handle);

        /// <summary>
        /// Closes opened wallet and frees allocated resources.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="handle">wallet handle returned by indy_open_wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_close_wallet(int command_handle, IntPtr handle, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Deletes created wallet.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="name">Name of the wallet to delete.</param>
        /// <param name="credentials">Wallet credentials json</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_delete_wallet(int command_handle, string name, string credentials, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Lists created wallets as JSON array with each wallet metadata: name, type, name of associated pool
        /// </summary>
        /// <returns>The list wallets.</returns>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false)]
        internal static extern int indy_list_wallets(int command_handle, ListWalletsCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_list_wallets.
        /// </summary>
        internal delegate void ListWalletsCompletedDelegate(int xcommand_handle, int err, string wallets);
    }
}
