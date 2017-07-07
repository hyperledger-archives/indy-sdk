using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Wrapper class for signus functions.
    /// </summary>
    public sealed class Signus : AsyncWrapperBase
    {
        private static CreateAndStoreMyDidResultDelegate CreateAndStoreMyDidResultCallback { get; }
        private static ReplaceKeysResultDelegate ReplaceKeysResultCallback { get; }
        private static SignResultDelegate SignResultCallback { get; }
        private static VerifySignatureResultDelegate VerifySignatureResultCallback { get; }
        private static EncryptResultDelegate EncryptResultCallback { get; }
        private static DecryptResultDelegate DecryptResultCallback { get; }

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

        public static Task<bool> VerifySignatureAsync(Wallet wallet, IntPtr poolHandle, string did, string signedMsg)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_verify_signature(
                commandHandle,
                wallet.Handle,
                poolHandle,
                did,
                signedMsg,
                VerifySignatureResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<EncryptResult> EncryptAsync(Wallet wallet, string did, string msg)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<EncryptResult>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_encrypt(
                commandHandle,
                wallet.Handle,
                did,
                msg,
                EncryptResultCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<string> DecryptAsync(Wallet wallet, string did, string encryptedMsg)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);


            var commandResult = LibSovrin.sovrin_decrypt(
                commandHandle,
                wallet.Handle,
                did,
                encryptedMsg,
                DecryptResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}
