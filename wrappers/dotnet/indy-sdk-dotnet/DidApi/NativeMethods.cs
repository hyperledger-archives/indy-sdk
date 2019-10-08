using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.DidApi
{
    internal static class NativeMethods
    {
        /// <summary>
        /// Creates keys (signing and encryption keys) for a new
        /// DID (owned by the caller of the library).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet)</param>
        /// <param name="did_info">Identity information as json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_and_store_my_did(int command_handle, int wallet_handle, string did_info, CreateAndStoreMyDidCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_create_and_store_my_did.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="did">The created DID.</param>
        /// <param name="verkey">The verification key for the signature.</param>
        internal delegate void CreateAndStoreMyDidCompletedDelegate(int xcommand_handle, int err, string did, string verkey);

        /// <summary>
        /// Generates new keys (signing and encryption keys) for an existing
        /// DID (owned by the caller of the library).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="did">Id of Identity stored in secured Wallet.</param>
        /// <param name="key_info">Identity information as json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_replace_keys_start(int command_handle, int wallet_handle, string did, string key_info, ReplaceKeysStartCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_replace_keys_start.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="verkey">The key for verification of signature.</param>
        internal delegate void ReplaceKeysStartCompletedDelegate(int xcommand_handle, int err, string verkey);

        /// <summary>
        /// Apply temporary keys as main for an existing DID (owned by the caller of the library).
        /// </summary>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="wallet_handle">wallet handler (created by open_wallet).</param>
        /// <param name="did">Id of Identity stored in secured Wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_replace_keys_apply(int command_handle, int wallet_handle, string did, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Saves their DID for a pairwise connection in a secured Wallet,
        /// so that it can be used to verify transaction.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet)</param>
        /// <param name="identity_json">Identity information as json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_store_their_did(int command_handle, int wallet_handle, string identity_json, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Returns ver key (key id) for the given DID.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="pool_handle">Pool handle (created by open_pool).</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="did">The DID to resolve key.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_key_for_did(int command_handle, int pool_handle, int wallet_handle, string did, DidKeyForDidCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_key_for_did.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="key">The verification key associated with the DID.</param>
        internal delegate void DidKeyForDidCompletedDelegate(int xcommand_handle, int err, string key);

        /// <summary>
        /// Returns ver key (key id) for the given DID.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="did">The DID to get the key for.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_key_for_local_did(int command_handle, int wallet_handle, string did, DidKeyForLocalDidCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_key_for_local_did.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="key">The key associated with the DID.</param>
        internal delegate void DidKeyForLocalDidCompletedDelegate(int xcommand_handle, int err, string key);

        /// <summary>
        /// Sets the endpoint information for the given DID.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="did">The DID to resolve endpoint.</param>
        /// <param name="address">The address of the endpoint.</param>
        /// <param name="transport_key">The key for the transport.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_set_endpoint_for_did(int command_handle, int wallet_handle, string did, string address, string transport_key, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Gets the endpoint information for the given DID.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="pool_handle">Pool handle (created by open_pool).</param>
        /// <param name="did">The DID to set the endpoint on.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_endpoint_for_did(int command_handle, int wallet_handle, int pool_handle, string did, DidGetEndpointForDidCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_get_endpoint_for_did.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="address">The endpoint address associated with the DID.</param>
        /// <param name="transport_vk">The transport verification key associated with the DID.</param>
        internal delegate void DidGetEndpointForDidCompletedDelegate(int xcommand_handle, int err, string address, string transport_vk);

        /// <summary>
        /// Saves/replaces the meta information for the giving DID in the wallet.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="did">the DID to store metadata.</param>
        /// <param name="metadata">the meta information that will be store with the DID.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_set_did_metadata(int command_handle, int wallet_handle, string did, string metadata, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Retrieves the meta information for the giving DID in the wallet.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="did">The DID to retrieve metadata.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_did_metadata(int command_handle, int wallet_handle, string did, DidGetDidMetadataCompletedDelegate cb);
        internal delegate void DidGetDidMetadataCompletedDelegate(int xcommand_handle, int err, string metadata);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_my_did_with_meta(int command_handle, int wallet_handle, string my_did, GetMyDidWithMetaCompletedDelegate cb);
        internal delegate void GetMyDidWithMetaCompletedDelegate(int xcommand_handle, int err, string did_with_meta);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_list_my_dids_with_meta(int command_handle, int wallet_handle, ListMyDidsWithMetaCompletedDelegate cb);
        internal delegate void ListMyDidsWithMetaCompletedDelegate(int xcommand_handle, int err, string dids);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_abbreviate_verkey(int command_handle, string did, string full_verkey, AbbreviateVerkeyCompletedDelegate cb);
        internal delegate void AbbreviateVerkeyCompletedDelegate(int xcommand_handle, int err, string verkey);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_qualify_did(int command_handle, int wallet_handle, string did, string method, QualifyDidCompletedDelegate cb);
        internal delegate void QualifyDidCompletedDelegate(int xcommand_handle, int err, string full_qualified_did);
    }
}
