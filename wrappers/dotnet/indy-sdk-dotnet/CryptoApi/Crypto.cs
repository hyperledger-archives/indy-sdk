using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using static Hyperledger.Indy.CryptoApi.NativeMethods;

namespace Hyperledger.Indy.CryptoApi
{
    /// <summary>
    /// Provides methods for pure cryptographic functions.
    /// </summary>
    public static class Crypto
    {
        /// <summary>
        /// Gets the callback to use when the indy_create_key command has completed.
        /// </summary>
        private static CreateKeyCompletedDelegate _createKeyCompletedCallback = (xcommand_handle, err, verkey) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(verkey);
        };

        /// <summary>
        /// Gets the callback to use when the indy_get_key_metadata command has completed.
        /// </summary>
        private static GetKeyMetadataCompletedDelegate _getKeyMetadataCompletedCallback = (xcommand_handle, err, metadata) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(metadata);
        };

        /// <summary>
        /// Gets the callback to use when the indy_crypto_sign command has completed.
        /// </summary>
        private static SignCompletedDelegate _cryptoSignCompletedCallback = (xcommand_handle, err, signature_raw, signature_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<byte[]>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var signatureBytes = new byte[signature_len];
            Marshal.Copy(signature_raw, signatureBytes, 0, signature_len);

            taskCompletionSource.SetResult(signatureBytes);
        };

        /// <summary>
        /// Gets the callback to use when the indy_crypto_verify command  has completed.
        /// </summary>
        private static VerifyCompletedDelegate _cryptoVerifyCompletedCallback = (xcommand_handle, err, valid) =>
        {
            var taskCompletionSource = PendingCommands.Remove<bool>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(valid);
        };

        /// <summary>
        /// Gets the callback to use when indy_crypto_auth_crypt or indy_crypto_anon_crypt has completed
        /// </summary>
        private static EncryptCompletedDelegate _cryptoEncryptCompletedCallback = (xcommand_handle, err, encrypted_msg, encrypted_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<byte[]>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var messageBytes = new byte[encrypted_len];
            Marshal.Copy(encrypted_msg, messageBytes, 0, encrypted_len);

            taskCompletionSource.SetResult(messageBytes);
        };

        /// <summary>
        /// Gets the callback to use when indy_crypto_auth_decrypt has completed
        /// </summary>
        private static AuthDecryptCompletedDelegate _cryptoAuthDecryptCompletedCallback = (command_handle, err, their_vk, msg_data, msg_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<AuthDecryptResult>(command_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var messageBytes = new byte[msg_len];
            Marshal.Copy(msg_data, messageBytes, 0, msg_len);

            var result = new AuthDecryptResult(their_vk, messageBytes);

            taskCompletionSource.SetResult(result);
        };


        /// <summary>
        /// Gets the callback to use when the indy_crypto_box_seal_open command has completed.
        /// </summary>
        private static AnonDecryptCompletedDelegate _cryptoAnonDecryptCompletedCallback = (command_handle, err, msg_data, msg_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<byte[]>(command_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var decryptedMsgBytes = new byte[msg_len];
            Marshal.Copy(msg_data, decryptedMsgBytes, 0, msg_len);

            taskCompletionSource.SetResult(decryptedMsgBytes);
        };

        /// <summary>
        /// Creates a key in the provided wallet.
        /// </summary>
        /// <remarks>
        /// The <paramref name="keyJson"/> parameter must contain a JSON object although all properties of the object are optional.  The schema
        /// the object must conform to are as follows:
        /// <code>
        /// {
        ///     "seed": string, // Optional (if not set random one will be used); Seed information that allows deterministic key creation.
        ///     "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
        /// }
        /// </code>
        /// The <c>seed</c> member is optional and is used to specify the seed to use for key creation - if this parameter is not set then a random seed will be used.
        /// The <c>crypto_type</c> member is also optional and will default to ed25519 curve if not set.
        /// <note type="note">At present the crypto_type member only supports the value 'ed22519'.</note>
        /// </remarks>
        /// <param name="wallet">The wallet to create the key in.</param>
        /// <param name="keyJson">The JSON string describing the key.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the verification key of the generated key-pair.</returns>
        /// <exception cref="InvalidStructureException">Thrown if the value passed to the <paramref name="keyJson"/> parameter is malformed or contains invalid data.</exception>
        public static Task<string> CreateKeyAsync(Wallet wallet, string keyJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(keyJson, "keyJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_create_key(
                commandHandle,
                wallet.Handle,
                keyJson,
                _createKeyCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Sets user defined metadata for a key-pair in the specified wallet.
        /// </summary>
        /// <remarks>
        /// Any existing metadata stored for the key-pair will be replaced.
        /// </remarks>
        /// <param name="wallet">The wallet containing the key.</param>
        /// <param name="verKey">The verification key of the key pair.</param>
        /// <param name="metadata">The metadata to set.</param>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation completes.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if the wallet does not contain a key-pair matching the provided <paramref name="verKey"/>.</exception>
        public static Task SetKeyMetadataAsync(Wallet wallet, string verKey, string metadata)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(verKey, "verKey");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_set_key_metadata(
                commandHandle,
                wallet.Handle,
                verKey,
                metadata,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the user defined metadata stored against a key-pair in the specified wallet.
        /// </summary>
        /// <remarks>
        /// If no metadata is stored against the specified key-pair null will be returned.</remarks>
        /// <param name="wallet">The wallet containing the key-pair.</param>
        /// <param name="verKey">The verification key of the key-pair.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the metadata associated with the key-pair.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if the wallet does not contain a key-pair matching the provided <paramref name="verKey"/> or they key-pair has no metadata.</exception>
        public static Task<string> GetKeyMetadataAsync(Wallet wallet, string verKey)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(verKey, "verKey");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_get_key_metadata(
                commandHandle,
                wallet.Handle,
                verKey,
                _getKeyMetadataCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Signs the provided message with the provided key.
        /// </summary>
        /// <remarks>
        /// The key provided as the <paramref name="myVk"/> parameter must have previously been stored in the <paramref name="wallet"/> using
        /// the <see cref="CreateKeyAsync(Wallet, string)"/> method or the <see cref="Did.CreateAndStoreMyDidAsync(Wallet, string)"/> method.
        /// <note type="note">
        /// To use DID keys with this method call the <see cref="Did.KeyForDidAsync(PoolApi.Pool, Wallet, string)"/> with the desired DID to get 
        /// its verification key which can be used as the <paramref name="myVk"/> parameter when calling this method.
        /// </note>
        /// </remarks>
        /// <param name="wallet">The wallet containing the key-pair to sign with.</param>
        /// <param name="myVk">The verification key of the key-pair to sign with.</param>
        /// <param name="message">The message to sign</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a byte array containing the signature.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if <paramref name="myVk"/> is not present in the <paramref name="wallet"/>.</exception>
        public static Task<byte[]> SignAsync(Wallet wallet, string myVk, byte[] message)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(myVk, "myVk");
            ParamGuard.NotNull(message, "message");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_crypto_sign(
                commandHandle,
                wallet.Handle,
                myVk,
                message,
                message.Length,
                _cryptoSignCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Verifies a message signature with a verification key.
        /// </summary>
        /// <note type="note">
        /// To use DID keys with this method call the <see cref="Did.KeyForDidAsync(PoolApi.Pool, Wallet, string)"/> with the desired DID to get 
        /// its verification key which can be used as the <paramref name="theirVk"/> parameter when calling this method.
        /// </note>
        /// <param name="theirVk">The verification key belonging to the party that signed the message.</param>
        /// <param name="message">The message that was signed.</param>
        /// <param name="signature">The signature for the message.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that, when the operation completes, resolves to true if the signature was valid, otherwise false.</returns>
        public static Task<bool> VerifyAsync(string theirVk, byte[] message, byte[] signature)
        {
            ParamGuard.NotNullOrWhiteSpace(theirVk, "theirVk");
            ParamGuard.NotNull(message, "message");
            ParamGuard.NotNull(signature, "signature");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_crypto_verify(
                commandHandle,
                theirVk,
                message,
                message.Length,
                signature,
                signature.Length,
                _cryptoVerifyCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Encrypt a message by authenticated-encryption scheme.
        /// 
        /// Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
        /// Using Recipient's public key, Sender can compute a shared secret key.
        /// Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
        /// That shared secret key can be used to verify that the encrypted message was not tampered with,
        /// before eventually decrypting it.
        /// </summary>
        /// <remarks>Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
        /// for specific DID.
        /// </remarks>
        /// <returns>The crypt async.</returns>
        /// <param name="wallet">The wallet containing the key-pair to sign with.</param>
        /// <param name="myVk"> id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did</param>
        /// <param name="theirVk">id (verkey) of their key</param>
        /// <param name="message">message data to be encrypted</param>
        public static Task<byte[]> AuthCryptAsync(Wallet wallet, string myVk, string theirVk, byte[] message)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(theirVk, "myVk");
            ParamGuard.NotNullOrWhiteSpace(theirVk, "theirVk");
            ParamGuard.NotNull(message, "message");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_crypto_auth_crypt(
                commandHandle,
                wallet.Handle,
                myVk,
                theirVk,
                message,
                message.Length,
                _cryptoEncryptCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Decrypt a message by authenticated-encryption scheme.
        ///
        /// Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
        /// Using Recipient's public key, Sender can compute a shared secret key.
        /// Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
        /// That shared secret key can be used to verify that the encrypted message was not tampered with,
        /// before eventually decrypting it.
        /// </summary>
        /// <remarks>
        /// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
        /// for specific DID.
        /// </remarks>
        /// <returns>sender verkey and decrypted message</returns>
        /// <param name="wallet">The wallet containing the key-pair to sign with.</param>
        /// <param name="myVk">id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did</param>
        /// <param name="message">The message data to be decrypted.</param>
        public static Task<AuthDecryptResult> AuthDecryptAsync(Wallet wallet, string myVk, byte[] message)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(myVk, "myVk");
            ParamGuard.NotNull(message, "message");

            var taskCompletionSource = new TaskCompletionSource<AuthDecryptResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_crypto_auth_decrypt(
                commandHandle,
                wallet.Handle,
                myVk,
                message,
                message.Length,
                _cryptoAuthDecryptCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Encrypts a message by anonymous-encryption scheme.
        ///
        /// Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
        /// Only the Recipient can decrypt these messages, using its private key.
        /// While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
        ///
        /// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
        /// for specific DID.
        /// </summary>
        /// <returns>The crypt async.</returns>
        /// <param name="theirVk">id (verkey) of their key</param>
        /// <param name="message">Message to be encrypted</param>
        public static Task<byte[]> AnonCryptAsync(string theirVk, byte[] message)
        {
            ParamGuard.NotNullOrWhiteSpace(theirVk, "theirVk");
            ParamGuard.NotNull(message, "message");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_crypto_anon_crypt(
                commandHandle,
                theirVk,
                message,
                message.Length,
                _cryptoEncryptCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task; 
        }

        /// <summary>
        /// Decrypts a message encrypted using an anonymous-encryption scheme
        /// </summary>
        /// <remarks>
        /// <para>
        /// Sealed boxes are designed to a <c>sender</c> to anonymously send messages to a <c>recipient</c> using the
        /// recipient's public key.
        /// Only the recipient can decrypt these messages, using their private key.
        /// While the recipient can verify the integrity of the message, they cannot verify the identity of the sender.
        /// </para>
        /// <note type="note">
        /// To use DID keys with this method call the <see cref="Did.KeyForDidAsync(PoolApi.Pool, Wallet, string)"/> with the desired DID to get 
        /// its verification key which can be used as the <paramref name="myVk"/> parameter when calling this method.
        /// </note>
        /// </remarks>
        /// <param name="wallet">The wallet containing the key-pair associated with the verification key specified in the <paramref name="myVk"/> parameter.</param>
        /// <param name="myVk">The verification key of the intended recipient of the encrypted message.</param>
        /// <param name="encryptedMessage">The encrypted message to decrypt.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a byte array containing the decrypted message.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if <paramref name="myVk"/> is not present in the <paramref name="wallet"/>.</exception>
        /// <exception cref="InvalidStructureException">Thrown if <paramref name="myVk"/> was not used to encrypt <paramref name="encryptedMessage"/>.</exception>
        public static Task<byte[]> AnonDecryptAsync(Wallet wallet, string myVk, byte[] encryptedMessage)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(myVk, "myVk");
            ParamGuard.NotNull(encryptedMessage, "encryptedMessage");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_crypto_anon_decrypt(
                commandHandle,
                wallet.Handle,
                myVk,
                encryptedMessage,
                encryptedMessage.Length,
                _cryptoAnonDecryptCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}
