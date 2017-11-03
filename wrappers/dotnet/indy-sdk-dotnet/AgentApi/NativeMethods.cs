using System;
using System.Runtime.InteropServices;

namespace Hyperledger.Indy.AgentApi
{
    internal static class NativeMethods
    {
        /// <summary>
        /// Prepares a message.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="sender_pk">Primary key of the sender.</param>
        /// <param name="recipient_vk">Validation key of the recipient.</param>
        /// <param name="msg_data">Message data.</param>
        /// <param name="msg_len">Length of message data in bytes.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prep_msg(int command_handle, IntPtr wallet_handle, string sender_pk, string recipient_vk, byte[] msg_data, int msg_len, PrepareMsgCompletedDelegate cb);

        /// <summary>
        /// Prepares an anonymous message.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="recipient_vk">Validation key of the recipient.</param>
        /// <param name="msg_data">Message data.</param>
        /// <param name="msg_len">Length of message data in bytes.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_prep_anonymous_msg(int command_handle, string recipient_vk, byte[] msg_data, int msg_len, PrepareMsgCompletedDelegate cb);

        /// <summary>
        /// Delegate for agent functions that prepare messages.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="encrypted_data">The encrypted data message.</param>
        /// <param name="encrypted_len">The encrypted data length.</param>
        internal delegate void PrepareMsgCompletedDelegate(int xcommand_handle, int err, IntPtr encrypted_data, int encrypted_len);

        /// <summary>
        /// Parses a message.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="recipient_vk">Validation key of the recipient.</param>
        /// <param name="encrypted_data">The encrypted data.</param>
        /// <param name="encrypted_len">The length of the encrypted data in bytes.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_parse_msg(int command_handle, IntPtr wallet_handle, string recipient_vk, byte[] encrypted_data, int encrypted_len, ParseMsgCompletedDelegate cb);

        /// <summary>
        /// Delegate for agent callbacks that parse messages.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="sender_key">The key of the sender.</param>
        /// <param name="msg_data">The message data.</param>
        /// <param name="msg_len">The message data length</param>
        internal delegate void ParseMsgCompletedDelegate(int xcommand_handle, int err, string sender_key, IntPtr msg_data, int msg_len);
    }
}
