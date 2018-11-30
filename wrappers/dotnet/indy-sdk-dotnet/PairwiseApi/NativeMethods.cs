using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.PairwiseApi
{
    internal static class NativeMethods
    {
        /// <summary>
        /// Checks whether a pairwise exists.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="their_did">encrypted DID</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_is_pairwise_exists(int command_handle, int wallet_handle, string their_did, IsPairwiseExistsCompletedDelegate cb);

        /// <summary>
        /// Delegate for pairwise exists that indicates whether or not a pairwise exists.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="exists">Whether or not the pairwise exists.</param>
        internal delegate void IsPairwiseExistsCompletedDelegate(int xcommand_handle, int err, bool exists);

        /// <summary>
        /// Creates pairwise.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="their_did">encrypted DID</param>
        /// <param name="my_did">encrypted DID</param>
        /// <param name="metadata">Optional: extra information for pairwise</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_pairwise(int command_handle, int wallet_handle, string their_did, string my_did, string metadata, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Get list of saved pairwise.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_list_pairwise(int command_handle, int wallet_handle, ListPairwiseCompletedDelegate cb);

        /// <summary>
        /// Delegate for listing saved pairwise.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="list_pairwise">list of saved pairwise</param>
        internal delegate void ListPairwiseCompletedDelegate(int xcommand_handle, int err, string list_pairwise);

        /// <summary>
        /// Gets pairwise information for specific their_did.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="their_did">encrypted DID</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_pairwise(int command_handle, int wallet_handle, string their_did, GetPairwiseCompletedDelegate cb);

        /// <summary>
        /// Delegate for getting a saved pairwise.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="pairwise_info_json">did info associated with their did</param>
        internal delegate void GetPairwiseCompletedDelegate(int xcommand_handle, int err, string pairwise_info_json);

        /// <summary>
        /// Save some data in the Wallet for pairwise associated with Did.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="their_did">encrypted DID</param>
        /// <param name="metadata">some extra information for pairwise</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_set_pairwise_metadata(int command_handle, int wallet_handle, string their_did, string metadata, IndyMethodCompletedDelegate cb);

    }
}
