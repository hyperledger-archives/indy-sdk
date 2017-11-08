using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using static Hyperledger.Indy.AgentApi.NativeMethods;

namespace Hyperledger.Indy.AgentApi
{
    /// <summary>
    /// Provides functionality to enable communication between agents.
    /// </summary>
    public static class Agent
    {
        /// <summary>
        /// Gets the callback to use when commands that prepare messages have completed.
        /// </summary>
        private static PrepareMsgCompletedDelegate _agentMessagePreparedCallback = (xcommand_handle, err, encrypted_data, encrypted_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<byte[]>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var bytes = new byte[encrypted_len];
            Marshal.Copy(encrypted_data, bytes, 0, encrypted_len);

            taskCompletionSource.SetResult(bytes);
        };

        /// <summary>
        /// Gets the callback to use when the ParsMsgAsync command has completed.
        /// </summary>
        private static ParseMsgCompletedDelegate _agentMessageParsedCallback = (xcommand_handle, err, sender_key, msg_data, msg_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<ParseMsgResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var bytes = new byte[msg_len];
            Marshal.Copy(msg_data, bytes, 0, msg_len);

            var result = new ParseMsgResult(sender_key, bytes);

            taskCompletionSource.SetResult(result);
        };

        /// <summary>
        /// Prepares a message so that it can be securely sent to a recipient.
        /// </summary>
        /// <param name="wallet">The wallet containing the keys of the sender and the recipient.</param>
        /// <param name="senderKey">The public key of the sender.</param>
        /// <param name="recipientKey">The verification key of the intended recipient of the message.</param>
        /// <param name="message">The message content to prepare.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to an array of bytes containing the prepared message.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if the sender key does not exist in the <paramref name="wallet"/>.</exception>
        /// <exception cref="InvalidStructureException">Thrown if the <paramref name="recipientKey"/> is invalid.</exception>
        public static Task<byte[]> PrepMsgAsync(Wallet wallet, string senderKey, string recipientKey, byte[] message)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(senderKey, "senderKey");
            ParamGuard.NotNullOrWhiteSpace(recipientKey, "recipientKey");
            ParamGuard.NotNull(message, "message");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_prep_msg(
                commandHandle,
                wallet.Handle,
                senderKey,
                recipientKey,
                message,
                message.Length,
                _agentMessagePreparedCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Prepares an anonymous message so that it can be securely sent to a recipient.
        /// </summary>
        /// <param name="recipientKey">The verification key of the intended recipient of the message.</param>
        /// <param name="message">The message content to prepare.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to an array of bytes containing the prepared message.</returns>
        /// <exception cref="InvalidStructureException">Thrown if the value passed to the <paramref name="recipientKey"/> is malformed or the content of the <paramref name="message"/> parameter is invalid.</exception>
        public static Task<byte[]> PrepAnonymousMsgAsync(string recipientKey, byte[] message)
        {
            ParamGuard.NotNullOrWhiteSpace(recipientKey, "recipientKey");
            ParamGuard.NotNull(message, "message");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_prep_anonymous_msg(
                commandHandle,
                recipientKey,
                message,
                message.Length,
                _agentMessagePreparedCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Parses an encrypted message prepared by another party.
        /// </summary>
        /// <param name="wallet">The wallet containing the keys.</param>
        /// <param name="recipientKey">The verification key of the recipient.</param>
        /// <param name="encryptedMsg">The encrypted message to parse.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="ParseMsgResult"/> containing the sender key and the parsed message 
        /// content.</returns>
        /// <exception cref="InvalidStructureException">Thrown if the value passed to the <paramref name="recipientKey"/> parameter is malformed or the <paramref name="encryptedMsg"/> is invalid.</exception>
        /// <exception cref="WalletValueNotFoundException">Thrown if the <paramref name="recipientKey"/> does not exist in the <paramref name="wallet"/>.</exception>
        public static Task<ParseMsgResult> ParseMsgAsync(Wallet wallet, string recipientKey, byte[] encryptedMsg)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(recipientKey, "recipientKey");
            ParamGuard.NotNull(encryptedMsg, "encryptedMsg");

            var taskCompletionSource = new TaskCompletionSource<ParseMsgResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var result = NativeMethods.indy_parse_msg(
                commandHandle,
                wallet.Handle,
                recipientKey,
                encryptedMsg,
                encryptedMsg.Length,
                _agentMessageParsedCallback);

            CallbackHelper.CheckResult(result);

            return taskCompletionSource.Task;
        }
    }
}
