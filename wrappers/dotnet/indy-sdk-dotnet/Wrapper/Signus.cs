using System.Runtime.InteropServices;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.IndyNativeMethods;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for signus functions.
    /// </summary>
    public sealed class Signus : AsyncWrapperBase
    {
        /// <summary>
        /// Gets the callback to use when the command for CreateAndStoreMyDidResultAsync has completed.
        /// </summary>
        private static CreateAndStoreMyDidResultDelegate _createAndStoreMyDidCallback = (xcommand_handle, err, did, verkey, pk) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<CreateAndStoreMyDidResult>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            var callbackResult = new CreateAndStoreMyDidResult(did, verkey, pk);

            taskCompletionSource.SetResult(callbackResult);
        };

        /// <summary>
        /// Gets the callback to use when the command for ReplaceKeysAsync has completed.
        /// </summary>
        private static ReplaceKeysResultDelegate _replaceKeysCallback = (xcommand_handle, err, verkey, pk) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<ReplaceKeysResult>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            var callbackResult = new ReplaceKeysResult(verkey, pk);

            taskCompletionSource.SetResult(callbackResult);
        };

        /// <summary>
        /// Gets the callback to use when the command for SignAsync has completed.
        /// </summary>
        private static SignResultDelegate _signCallback = (xcommand_handle, err, signature_raw, signature_len) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<byte[]>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            var bytes = new byte[signature_len];
            Marshal.Copy(signature_raw, bytes, 0, signature_len);

            taskCompletionSource.SetResult(bytes);
        };


        /// <summary>
        /// Gets the callback to use when the command for VerifySignatureAsync has completed.
        /// </summary>
        private static VerifySignatureResultDelegate _verifySignatureCallback = (xcommand_handle, err, valid) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<bool>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(valid);
        };

        /// <summary>
        /// Gets the callback to use when the command for EncryptAsync has completed.
        /// </summary>
        private static EncryptResultDelegate _encryptCallback = (xcommand_handle, err, encrypted_msg_raw, encrypted_msg_len, nonce_raw, nonce_len) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<EncryptResult>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            var encryptedMessageBytes = new byte[encrypted_msg_len];
            Marshal.Copy(encrypted_msg_raw, encryptedMessageBytes, 0, encrypted_msg_len);

            var nonceBytes = new byte[nonce_len];
            Marshal.Copy(nonce_raw, nonceBytes, 0, nonce_len);

            var callbackResult = new EncryptResult(encryptedMessageBytes, nonceBytes);

            taskCompletionSource.SetResult(callbackResult);
        };

        /// <summary>
        /// Gets the callback to use when the command for DecryptAsync has completed.
        /// </summary>
        private static DecryptResultDelegate _decryptCallback = (xcommand_handle, err, decrypted_msg_raw, decrypted_msg_len) =>
        {
            var taskCompletionSource = RemoveTaskCompletionSource<byte[]>(xcommand_handle);

            if (!CheckCallback(taskCompletionSource, err))
                return;

            var decryptedMsgBytes = new byte[decrypted_msg_len];
            Marshal.Copy(decrypted_msg_raw, decryptedMsgBytes, 0, decrypted_msg_len);

            taskCompletionSource.SetResult(decryptedMsgBytes);
        };

        
        /// <summary>
        /// Creates and stores the local party's DID in the specified wallet.
        /// </summary>
        /// <param name="wallet">The wallet to store the DID in.</param>
        /// <param name="didJson">The DID JSON.</param>
        /// <returns>An asynchronous task that returns a CreateAndStoreMyDidResult.</returns>
        public static Task<CreateAndStoreMyDidResult> CreateAndStoreMyDidAsync(Wallet wallet, string didJson)
        {
            var taskCompletionSource = new TaskCompletionSource<CreateAndStoreMyDidResult>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_create_and_store_my_did(
                commandHandle,
                wallet.Handle,
                didJson,
                _createAndStoreMyDidCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Replaces the keys for the specified DID in the specified wallet.
        /// </summary>
        /// <param name="wallet">The wallet the DID is stored in.</param>
        /// <param name="did">The did to replace the keys for.</param>
        /// <param name="identityJson">The identity JSON.</param>
        /// <returns>An asynchronous task that returns a ReplaceKeysResult.</returns>
        public static Task<ReplaceKeysResult> ReplaceKeysAsync(Wallet wallet, string did, string identityJson)
        {
            var taskCompletionSource = new TaskCompletionSource<ReplaceKeysResult>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_replace_keys(
                commandHandle,
                wallet.Handle,
                did,
                identityJson,
                _replaceKeysCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Stores a remote party's DID in the specified wallet.
        /// </summary>
        /// <param name="wallet">The wallet to store the DID in.</param>
        /// <param name="identityJson">The identity JSON.</param>
        /// <returns>An asynchronous task with no return value.</returns>
        public static Task StoreTheirDidAsync(Wallet wallet, string identityJson)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_store_their_did(
                commandHandle,
                wallet.Handle,
                identityJson,
                _noValueCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Signs the specified message using the specified DID.
        /// </summary>
        /// <param name="wallet">The wallet that contains the DID.</param>
        /// <param name="did">The did to sign with.</param>
        /// <param name="msg">The message to sign.</param>
        /// <returns>An asynchronous task that returns the signed message.</returns>
        public static Task<byte[]> SignAsync(Wallet wallet, string did, byte[] msg)
        {
            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_sign(
                commandHandle,
                wallet.Handle,
                did,
                msg,
                msg.Length,
                _signCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }


        /// <summary>
        /// Verifies a signed message.
        /// </summary>
        /// <param name="wallet">The wallet containing the DID of the signed message.</param>
        /// <param name="pool">The ledger pool to verify the message against.</param>
        /// <param name="did">The did the message is associated with.</param>
        /// <param name="msg">The message to verify.</param>
        /// <param name="signature">The signature to verify.</param>
        /// <returns>An asynchronous task that returns true if the message is valid, otherwise false.</returns>
        public static Task<bool> VerifySignatureAsync(Wallet wallet, Pool pool, string did, byte[] msg, byte[]signature)
        {
            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_verify_signature(
                commandHandle,
                wallet.Handle,
                pool.Handle,
                did,
                msg,
                msg.Length,
                signature,
                signature.Length,
                _verifySignatureCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Encrypts the specified message for the specified DID.
        /// </summary>
        /// <param name="wallet">The wallet containing the DID to use for encyption.</param>
        /// <param name="pool">The pool</param>
        /// <param name="my_did">My DID</param>
        /// <param name="did">The did to encrypt for.</param>
        /// <param name="msg">The message to encrypt.</param>
        /// <returns>An asynchronous task that returns an EncryptResult.</returns>
        public static Task<EncryptResult> EncryptAsync(Wallet wallet, Pool pool, string my_did, string did, byte[] msg)
        {
            var taskCompletionSource = new TaskCompletionSource<EncryptResult>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_encrypt(
                commandHandle,
                wallet.Handle,
                pool.Handle,
                my_did,
                did,
                msg,
                msg.Length,
                _encryptCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Decrypts the specified messaage.
        /// </summary>
        /// <param name="wallet">The wallet containing the DID to use for decrpytion.</param>
        /// <param name="my_did">My DID.</param>
        /// <param name="did">The DID</param>
        /// <param name="encryptedMsg">The message to decrypt.</param>
        /// <param name="nonce">The nonce.</param>
        /// <returns>An asynchronous task that returns the decrypted message.</returns>
        public static Task<byte[]> DecryptAsync(Wallet wallet, string my_did, string did, byte[] encryptedMsg, byte[] nonce)
        {
            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = AddTaskCompletionSource(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_decrypt(
                commandHandle,
                wallet.Handle,
                my_did,
                did,
                encryptedMsg,
                encryptedMsg.Length,
                nonce,
                nonce.Length,
                _decryptCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}
