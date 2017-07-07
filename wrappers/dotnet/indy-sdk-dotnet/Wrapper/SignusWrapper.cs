using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.LibSovrin;

namespace Indy.Sdk.Dotnet.Wrapper
{
    public sealed class SignusWrapper : AsyncWrapperBase
    {
        private static CreateAndStoreMyDidResultDelegate CreateAndStoreMyDidResultCallback { get; }
        private static ReplaceKeysResultDelegate ReplaceKeysResultCallback { get; }
        private static SignResultDelegate SignResultCallback { get; }
        private static VerifySignatureResultDelegate VerifySignatureResultCallback { get; }
        private static EncryptResultDelegate EncryptResultCallback { get; }
        private static DecryptResultDelegate DecryptResultCallback { get; }

        static SignusWrapper()
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

        public static Task<CreateAndStoreMyDidResult> CreateAndStoreMyDidAsync(IntPtr walletHandle, string didJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<CreateAndStoreMyDidResult>(commandHandle);

            var commandResult = LibSovrin.sovrin_create_and_store_my_did(
                commandHandle,
                walletHandle,
                didJson,
                CreateAndStoreMyDidResultCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<ReplaceKeysResult> ReplaceKeysAsync(IntPtr walletHandle, string did, string identityJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<ReplaceKeysResult>(commandHandle);


            var commandResult = LibSovrin.sovrin_replace_keys(
                commandHandle,
                walletHandle,
                did,
                identityJson,
                ReplaceKeysResultCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task StoreTheirDidAsync(IntPtr walletHandle, string identityJson)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_store_their_did(
                commandHandle,
                walletHandle,
                identityJson,
                ResultOnlyCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<string> SignAsync(IntPtr walletHandle, string did, string msg)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_sign(
                commandHandle,
                walletHandle,
                did,
                msg,
                SignResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<bool> VerifySignatureAsync(IntPtr walletHandle, IntPtr poolHandle, string did, string signedMsg)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<bool>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_verify_signature(
                commandHandle,
                walletHandle,
                poolHandle,
                did,
                signedMsg,
                VerifySignatureResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<EncryptResult> EncryptAsync(IntPtr walletHandle, string did, string msg)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<EncryptResult>(commandHandle);
            
            var commandResult = LibSovrin.sovrin_encrypt(
                commandHandle,
                walletHandle,
                did,
                msg,
                EncryptResultCallback);

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        public static Task<string> DecryptAsync(IntPtr walletHandle, string did, string encryptedMsg)
        {
            var commandHandle = GetNextCommandHandle();
            var taskCompletionSource = CreateTaskCompletionSourceForCommand<string>(commandHandle);


            var commandResult = LibSovrin.sovrin_decrypt(
                commandHandle,
                walletHandle,
                did,
                encryptedMsg,
                DecryptResultCallback
                );

            CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}
