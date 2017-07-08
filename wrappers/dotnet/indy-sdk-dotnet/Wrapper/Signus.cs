using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.LibSovrin;

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
        private static CreateAndStoreMyDidResultDelegate CreateAndStoreMyDidResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the command for ReplaceKeysAsync has completed.
        /// </summary>
        private static ReplaceKeysResultDelegate ReplaceKeysResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the command for SignAsync has completed.
        /// </summary>
        private static SignResultDelegate SignResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the command for VerifySignatureAsync has completed.
        /// </summary>
        private static VerifySignatureResultDelegate VerifySignatureResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the command for EncryptAsync has completed.
        /// </summary>
        private static EncryptResultDelegate EncryptResultCallback { get; }

        /// <summary>
        /// Gets the callback to use when the command for DecryptAsync has completed.
        /// </summary>
        private static DecryptResultDelegate DecryptResultCallback { get; }

        /// <summary>
        /// Static initalizer.
        /// </summary>
        static Signus()
        {
            CreateAndStoreMyDidResultCallback = (xCommandHandle, err, did, verKey, pk) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<CreateAndStoreMyDidResult>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                var callbackResult = new CreateAndStoreMyDidResult(did, verKey, pk);

                taskCompletionSource.SetResult(callbackResult);
            };

            ReplaceKeysResultCallback = (xCommandHandle, err, verKey, pk) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<ReplaceKeysResult>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                var callbackResult = new ReplaceKeysResult(verKey, pk);

                taskCompletionSource.SetResult(callbackResult);
            };

            SignResultCallback = (xCommandHandle, err, signature) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<string>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(signature);
            };

            VerifySignatureResultCallback = (xCommandHandle, err, valid) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<bool>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(valid);
            };

            EncryptResultCallback = (xCommandHandle, err, encryptedMsg, nonce) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<EncryptResult>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                var callbackResult = new EncryptResult(encryptedMsg, nonce);

                taskCompletionSource.SetResult(callbackResult);
            };

            DecryptResultCallback = (xCommandHandle, err, decryptedMsg) =>
            {
                var taskCompletionSource = GetTaskCompletionSourceForCommand<string>(xCommandHandle);

                if (!CheckCallback(taskCompletionSource, xCommandHandle, err))
                    return;

                taskCompletionSource.SetResult(decryptedMsg);
            };

        }

        /// <summary>
        /// Creates and stores the local party's DID in the specified wallet.
        /// </summary>
        /// <param name="wallet">The wallet to store the DID in.</param>
        /// <param name="didJson">The DID JSON.</param>
        /// <returns>An asynchronous task that returns a CreateAndStoreMyDidResult.</returns>
        public static Task<CreateAndStoreMyDidResult> CreateAndStoreMyDidAsync(Wallet wallet, string didJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<CreateAndStoreMyDidResult>(commandHandle);

            var commandResult = LibSovrin.sovrin_create_and_store_my_did(
                commandHandle,
                wallet.Handle,
                didJson,
                CreateAndStoreMyDidResultCallback);

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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<ReplaceKeysResult>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_replace_keys(
                commandHandle,
                wallet.Handle,
                did,
                identityJson,
                ReplaceKeysResultCallback);

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
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_store_their_did(
                commandHandle,
                wallet.Handle,
                identityJson,
                ResultOnlyCallback);

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
        public static Task<string> SignAsync(Wallet wallet, string did, string msg)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_sign(
                commandHandle,
                wallet.Handle,
                did,
                msg,
                SignResultCallback
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
        /// <param name="signedMsg">The signed message to verify.</param>
        /// <returns>An asynchronous task that returns true if the message is valid, otherwise false.</returns>
        public static Task<bool> VerifySignatureAsync(Wallet wallet, Pool pool, string did, string signedMsg)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_verify_signature(
                commandHandle,
                wallet.Handle,
                pool.Handle,
                did,
                signedMsg,
                VerifySignatureResultCallback
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
        public static Task<EncryptResult> EncryptAsync(Wallet wallet, Pool pool, string my_did, string did, string msg)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<EncryptResult>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_encrypt(
                commandHandle,
                wallet.Handle,
                pool.Handle,
                my_did,
                did,
                msg,
                EncryptResultCallback);

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
        public static Task<string> DecryptAsync(Wallet wallet, string my_did, string did, string encryptedMsg, string nonce)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);


            var commandResult = LibSovrin.sovrin_decrypt(
                commandHandle,
                wallet.Handle,
                my_did,
                did,
                encryptedMsg,
                nonce,
                DecryptResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}
