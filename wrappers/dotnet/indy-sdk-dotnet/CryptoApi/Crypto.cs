using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using static Hyperledger.Indy.CryptoApi.NativeMethods;

namespace Hyperledger.Indy.CryptoApi
{
    /// <summary>
    /// Provides methods for performing .
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
        /// Gets the callback to use when the indy_crypto_box command has completed.
        /// </summary>
        private static BoxCompletedDelegate _cryptoBoxCompletedCallback = (xcommand_handle, err, encrypted_msg_raw, encrypted_msg_len, nonce_raw, nonce_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<BoxResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var encryptedMsgBytes = new byte[encrypted_msg_len];
            Marshal.Copy(encrypted_msg_raw, encryptedMsgBytes, 0, encrypted_msg_len);

            var nonceBytes = new byte[nonce_len];
            Marshal.Copy(nonce_raw, nonceBytes, 0, nonce_len);

            var result = new BoxResult(encryptedMsgBytes, nonceBytes);

            taskCompletionSource.SetResult(result);
        };

        /// <summary>
        /// Gets the callback to use when the indy_crypto_box_open command has completed.
        /// </summary>
        private static BoxOpenCompletedDelegate _cryptoBoxOpenCompletedCallback = (xcommand_handle, err, decrypted_msg_raw, decrypted_msg_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<byte[]>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var decryptedMsgBytes = new byte[decrypted_msg_len];
            Marshal.Copy(decrypted_msg_raw, decryptedMsgBytes, 0, decrypted_msg_len);

            taskCompletionSource.SetResult(decryptedMsgBytes);
        };

        /// <summary>
        /// Gets the callback to use when the indy_crypto_box_seal command has completed.
        /// </summary>
        private static BoxSealCompletedDelegate _cryptoBoxSealCompletedCallback = (xcommand_handle, err, encrypted_msg_raw, encrypted_msg_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<byte[]>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var encryptedMsgBytes = new byte[encrypted_msg_len];
            Marshal.Copy(encrypted_msg_raw, encryptedMsgBytes, 0, encrypted_msg_len);

            taskCompletionSource.SetResult(encryptedMsgBytes);
        };

        /// <summary>
        /// Gets the callback to use when the indy_crypto_box_seal_open command has completed.
        /// </summary>
        private static BoxSealOpenCompletedDelegate _cryptoBoxSealOpenCompletedCallback = (xcommand_handle, err, decrypted_msg_raw, decrypted_msg_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<byte[]>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var decryptedMsgBytes = new byte[decrypted_msg_len];
            Marshal.Copy(decrypted_msg_raw, decryptedMsgBytes, 0, decrypted_msg_len);

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
        /// the <see cref="CreateKeyAsync(Wallet, string)"/> method or the <see cref="Signus.CreateAndStoreMyDidAsync(Wallet, string)"/> method.
        /// <note type="note">
        /// To use DID keys with this method call the <see cref="Signus.KeyForDidAsync(PoolApi.Pool, Wallet, string)"/> with the desired DID to get 
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
        /// To use DID keys with this method call the <see cref="Signus.KeyForDidAsync(PoolApi.Pool, Wallet, string)"/> with the desired DID to get 
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
        /// Encrypts a message using an authenticated-encryption scheme.
        /// </summary>
        /// <remarks>
        /// <para>
        /// A <c>sender</c> can use their public key to encrypt a confidential message specifically for a <c>recipient</c>.
        /// Using the recipient's public key, the sender can compute a shared secret key.
        /// The recipient, using the sender's public key and his own secret key, can compute the exact same shared secret key.
        /// That shared secret key can then be used to verify that the encrypted message was not tampered with,
        /// before eventually decrypting it.
        /// </para>
        /// <para>
        /// The recipient only needs the sender's public key, the nonce and the ciphertext to decrypt the message.
        /// The nonce doesn't have to be confidential.
        /// </para>
        /// <para>
        /// Messages encrypted using this method can be decrypted using the <see cref="BoxOpenAsync(Wallet, string, string, byte[], byte[])"/> method.
        /// </para>
        /// <note type="note">
        /// To use DID keys with this method call the <see cref="Signus.KeyForDidAsync(PoolApi.Pool, Wallet, string)"/> with the desired DID to get 
        /// its verification key which can be used as the <paramref name="myVk"/> and/or <paramref name="theirVk"/> parameter when calling this method.
        /// </note>
        /// </remarks>
        /// <param name="wallet">The wallet containing the keys.</param>
        /// <param name="myVk">The verification key of the party encrypting the message.</param>
        /// <param name="theirVk">The verification key of the party that is the intended recipient of the message.</param>
        /// <param name="message">The message to encrypt.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="BoxResult"/> when the operation completes.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if <paramref name="myVk"/> is not in the <paramref name="wallet"/>.</exception>
        public static Task<BoxResult> BoxAsync(Wallet wallet, string myVk, string theirVk, byte[] message)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(myVk, "myVk");
            ParamGuard.NotNullOrWhiteSpace(theirVk, "theirVk");
            ParamGuard.NotNull(message, "message");

            var taskCompletionSource = new TaskCompletionSource<BoxResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_crypto_box(
                commandHandle,
                wallet.Handle,
                myVk,
                theirVk,
                message,
                message.Length,
                _cryptoBoxCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Decrypts a message encrypted using an authenticated-encryption scheme.
        /// </summary>
        /// <remarks>
        /// <para>
        /// A <c>sender</c> can use their public key to encrypt a confidential message specifically for a <c>recipient</c>.
        /// Using the recipient's public key, the sender can compute a shared secret key.
        /// The recipient, using the sender's public key and his own secret key, can compute the exact same shared secret key.
        /// That shared secret key can then be used to verify that the encrypted message was not tampered with,
        /// before eventually decrypting it.
        /// </para>
        /// <para>
        /// The recipient only needs the sender's public key, the nonce and the ciphertext to decrypt the message.
        /// The nonce doesn't have to be confidential.
        /// </para>
        /// <para>
        /// The <see cref="BoxAsync(Wallet, string, string, byte[])"/> method can be used to encrypt messages suitable for decryption using this method.
        /// </para>
        /// <note type="note">
        /// To use DID keys with this method call the <see cref="Signus.KeyForDidAsync(PoolApi.Pool, Wallet, string)"/> with the desired DID to get 
        /// its verification key which can be used as the <paramref name="myVk"/> and/or <paramref name="theirVk"/> parameters when calling this method.
        /// </note>
        /// </remarks>
        /// <param name="wallet">The wallet containing the t.</param>
        /// <param name="myVk">The verification key of recipient of an encrypted message.</param>
        /// <param name="theirVk">The verification key of the party that encrypted the message.</param>
        /// <param name="encryptedMessage">The encrypted message.</param>
        /// <param name="nonce">The nonce used for encrypting the message.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a byte array containing the decrypted message.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if the <paramref name="theirVk"/> is not present in the <paramref name="wallet"/>.</exception>
        /// <exception cref="InvalidStructureException">Thrown if <paramref name="myVk"/> is not in the <paramref name="wallet"/> or the <paramref name="nonce"/>
        /// does not match the <paramref name="encryptedMessage"/>.</exception>
        public static Task<byte[]> BoxOpenAsync(Wallet wallet, string myVk, string theirVk, byte[] encryptedMessage, byte[] nonce)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(myVk, "myVk");
            ParamGuard.NotNullOrWhiteSpace(theirVk, "theirVk");
            ParamGuard.NotNull(encryptedMessage, "encryptedMessage");
            ParamGuard.NotNull(nonce, "nonce");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_crypto_box_open(
                commandHandle,
                wallet.Handle,
                myVk,
                theirVk,
                encryptedMessage,
                encryptedMessage.Length,
                nonce,
                nonce.Length,
                _cryptoBoxOpenCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Encrypts a message using an anonymous-encryption scheme.
        /// </summary>
        /// <remarks>
        /// <para>
        /// Sealed boxes are designed to a <c>sender</c> to anonymously send messages to a <c>recipient</c> using the
        /// recipient's public key.
        /// Only the recipient can decrypt these messages, using their private key.
        /// While the recipient can verify the integrity of the message, they cannot verify the identity of the sender.
        /// </para>
        /// <para>
        /// The <see cref="BoxSealOpenAsync(Wallet, string, byte[])"/> method can be used to decrypt messages encrypted using this method.
        /// </para>
        /// <note type="note">
        /// To use DID keys with this method call the <see cref="Signus.KeyForDidAsync(PoolApi.Pool, Wallet, string)"/> with the desired DID to get 
        /// its verification key which can be used as the <paramref name="theirVk"/> parameter when calling this method.
        /// </note>
        /// </remarks>
        /// <param name="theirVk">The verification key of the intended recipient of the encrypted message.</param>
        /// <param name="message">The message to encrypt.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a byte array containing the encrypted message.</returns>
        public static Task<byte[]> BoxSealAsync(string theirVk, byte[] message)
        {
            ParamGuard.NotNullOrWhiteSpace(theirVk, "theirVk");
            ParamGuard.NotNull(message, "message");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_crypto_box_seal(
                commandHandle,
                theirVk,
                message,
                message.Length,
                _cryptoBoxSealCompletedCallback
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
        /// <para>
        /// The <see cref="BoxSealAsync(string, byte[])"/> method can be used to encrypt messages suitable for decryption using this method.
        /// </para>
        /// <note type="note">
        /// To use DID keys with this method call the <see cref="Signus.KeyForDidAsync(PoolApi.Pool, Wallet, string)"/> with the desired DID to get 
        /// its verification key which can be used as the <paramref name="myVk"/> parameter when calling this method.
        /// </note>
        /// </remarks>
        /// <param name="wallet">The wallet containing the key-pair associated with the verification key specified in the <paramref name="myVk"/> parameter.</param>
        /// <param name="myVk">The verification key of the intended recipient of the encrypted message.</param>
        /// <param name="encryptedMessage">The encrypted message to decrypt.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a byte array containing the decrypted message.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if <paramref name="myVk"/> is not present in the <paramref name="wallet"/>.</exception>
        /// <exception cref="InvalidStructureException">Thrown if <paramref name="myVk"/> was not used to encrypt <paramref name="encryptedMessage"/>.</exception>
        public static Task<byte[]> BoxSealOpenAsync(Wallet wallet, string myVk, byte[] encryptedMessage)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(myVk, "myVk");
            ParamGuard.NotNull(encryptedMessage, "encryptedMessage");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = NativeMethods.indy_crypto_box_seal_open(
                commandHandle,
                wallet.Handle,
                myVk,
                encryptedMessage,
                encryptedMessage.Length,
                _cryptoBoxSealOpenCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}
