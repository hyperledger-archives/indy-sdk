﻿using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using static Hyperledger.Indy.IndyNativeMethods;

namespace Hyperledger.Indy.SignusApi
{
    /// <summary>
    /// Provides cryptographic functionality.
    /// </summary>
    public static class Signus 
    {
        /// <summary>
        /// Gets the callback to use when the command for CreateAndStoreMyDidResultAsync has completed.
        /// </summary>
        private static CreateAndStoreMyDidResultDelegate _createAndStoreMyDidCallback = (xcommand_handle, err, did, verkey) =>
        {
            var taskCompletionSource = PendingCommands.Remove<CreateAndStoreMyDidResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var callbackResult = new CreateAndStoreMyDidResult(did, verkey);

            taskCompletionSource.SetResult(callbackResult);
        };

        /// <summary>
        /// Gets the callback to use when the command for ReplaceKeysAsync has completed.
        /// </summary>
        private static ReplaceKeysStartResultDelegate _replaceKeysCallback = (xcommand_handle, err, verkey) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(verkey);
        };

        /// <summary>
        /// Gets the callback to use when the command for SignAsync has completed.
        /// </summary>
        private static SignResultDelegate _signCallback = (xcommand_handle, err, signature_raw, signature_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<byte[]>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
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
            var taskCompletionSource = PendingCommands.Remove<bool>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(valid);
        };

        /// <summary>
        /// Gets the callback to use when the command for EncryptAsync has completed.
        /// </summary>
        private static EncryptResultDelegate _encryptCallback = (xcommand_handle, err, encrypted_msg_raw, encrypted_msg_len, nonce_raw, nonce_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<EncryptResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
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
            var taskCompletionSource = PendingCommands.Remove<byte[]>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var decryptedMsgBytes = new byte[decrypted_msg_len];
            Marshal.Copy(decrypted_msg_raw, decryptedMsgBytes, 0, decrypted_msg_len);

            taskCompletionSource.SetResult(decryptedMsgBytes);
        };

        /// <summary>
        /// Gets the callback to use when the command for EncryptAsync has completed.
        /// </summary>
        private static EncryptSealedResultDelegate _encryptSealedCallback = (xcommand_handle, err, encrypted_msg_raw, encrypted_msg_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<byte[]>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var encryptedMessageBytes = new byte[encrypted_msg_len];
            Marshal.Copy(encrypted_msg_raw, encryptedMessageBytes, 0, encrypted_msg_len);

            taskCompletionSource.SetResult(encryptedMessageBytes);
        };

        /// <summary>
        /// Gets the callback to use when the command for DecryptAsync has completed.
        /// </summary>
        private static DecryptResultDelegate _decryptSealedCallback = (xcommand_handle, err, decrypted_msg_raw, decrypted_msg_len) =>
        {
            var taskCompletionSource = PendingCommands.Remove<byte[]>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var decryptedMsgBytes = new byte[decrypted_msg_len];
            Marshal.Copy(decrypted_msg_raw, decryptedMsgBytes, 0, decrypted_msg_len);

            taskCompletionSource.SetResult(decryptedMsgBytes);
        };

        /// <summary>
        /// Gets the callback to use when the command for CreateKeyAsync has completed.
        /// </summary>
        private static SignusCreateKeyCompletedDelegate _createKeyCompletedCallback = (xcommand_handle, err, verkey) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(verkey);
        };

        /// <summary>
        /// Gets the callback to use when the command for GetKeyMetaDataAsync has completed.
        /// </summary>
        private static SignusGetKeyMetadataCompletedDelegate _getKeyMetadataCompletedCallback = (xcommand_handle, err, metadata) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(metadata);
        };

        /// <summary>
        /// Gets the callback to use when the command for KeyForDidAsync has completed.
        /// </summary>
        private static SignusKeyForDidCompletedDelegate _keyForDidCompletedCallback = (xcommand_handle, err, key) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(key);
        };

        /// <summary>
        /// Gets the callback to use when the command for GetEndpointForDidAsync has completed.
        /// </summary>
        private static SignusGetEndpointForDidCompletedDelegate _getEndpointForDidCompletedCallback = (xcommand_handle, err, endpoint, transport_vk) =>
        {
            var taskCompletionSource = PendingCommands.Remove<EndpointForDidResult>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            var result = new EndpointForDidResult(endpoint, transport_vk);

            taskCompletionSource.SetResult(result);
        };

        /// <summary>
        /// Gets the callback to use when the command for GetDidMetadataAsync has completed.
        /// </summary>
        private static SignusGetDidMetadataCompletedDelegate _getDidMetadataCompletedCallback = (xcommand_handle, err, metadata) =>
        {
            var taskCompletionSource = PendingCommands.Remove<string>(xcommand_handle);

            if (!CallbackHelper.CheckCallback(taskCompletionSource, err))
                return;

            taskCompletionSource.SetResult(metadata);
        };

        /// <summary>
        /// Creates signing and encryption keys in specified wallet for a new DID owned by the caller.
        /// </summary>
        /// <remarks>
        /// <para>Saves the identity DID with keys in a wallet so that it can be used to sign
        /// and encrypt transactions.  Control over the created DID is provided through the 
        /// <paramref name="didJson"/> parameter which accepts a JSON string with the following
        /// optional parameters:
        /// </para>
        /// <code>
        /// {
        ///     "did": string,
        ///     "seed": string, 
        ///     "crypto_type": string, 
        ///     "cid": bool
        /// }
        /// </code>
        /// <para>The <c>did</c> member specifies the DID of the new entry.  If not 
        /// provided and the <c>cid</c> member is <c>false</c> then the first 16 bits of the VerKey value 
        /// generated will be used as a new DID.  If not provided and the <c>cid</c> member is <c>true</c> then the full 
        /// VerKey value will be used as a new DID.  If the <c>did</c> member is provided then the keys will be 
        /// replaced - this is normally used in the case of key rotation.</para>
        /// <para>The <c>seed</c> member specifies the seed to use when generating keys.  If not provided 
        /// then a random seed value will be created.</para>
        /// <para>The <c>crypto_type</c> member specifies the cryptographic algorithm used for generating
        /// keys.  If not provided then ed25519 curve is used.
        /// <note type="note">The only value currently supported for this member is 'ed25519'.</note>
        /// </para>
        /// <para>The <c>cid</c> member indicates whether the DID should be used in creating the DID.
        /// If not provided then the value defaults to false.</para>
        /// </remarks>
        /// <param name="wallet">The wallet to store the DID in.</param>
        /// <param name="didJson">The DID JSON.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a <see cref="CreateAndStoreMyDidResult"/> when the operation completes.</returns>
        public static Task<CreateAndStoreMyDidResult> CreateAndStoreMyDidAsync(Wallet wallet, string didJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(didJson, "didJson");

            var taskCompletionSource = new TaskCompletionSource<CreateAndStoreMyDidResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_create_and_store_my_did(
                commandHandle,
                wallet.Handle,
                didJson,
                _createAndStoreMyDidCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Generates new signing and encryption keys in the specified wallet for an existing DID owned by the caller
        /// </summary>
        /// <remarks>
        /// The developer has some control over the generation of the new keys through the value passed to
        /// the <paramref name="identityJson"/> parameter.  This parameter expects a valid JSON string
        /// with the following optional members:
        /// <code>
        /// {
        ///     "seed": string, 
        ///     "crypto_type": string
        /// }
        /// </code>
        /// <para>The <c>seed</c> member controls the seed that will be used to generate they keys.
        /// If not provided a random one will be created.</para>
        /// <para>The <c>crypto_type</c> member specifies the type of cryptographic algorithm will be 
        /// used to generate they keys.  If not provided then ed22519 curve will be used.
        /// <note type="note">The only value currently supported for this member is 'ed25519'.</note>
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet the DID is stored in.</param>
        /// <param name="did">The did to replace the keys for.</param>
        /// <param name="identityJson">The identity information as JSON.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the new verification key when the operation completes.</returns>
        public static Task<string> ReplaceKeysStartAsync(Wallet wallet, string did, string identityJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNullOrWhiteSpace(identityJson, "identityJson");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_replace_keys_start(
                commandHandle,
                wallet.Handle,
                did,
                identityJson,
                _replaceKeysCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Applies temporary signing and encryption keys as main in the specified wallet for an existing DID owned by the caller
        /// </summary>
        /// <param name="wallet">The wallet the DID is stored in.</param>
        /// <param name="did">The did to replace the keys for.</param>
        /// <returns>An asynchronous <see cref="Task"/> that  with no return value the completes when the operation completes.</returns>
        public static Task ReplaceKeysApplyAsync(Wallet wallet, string did)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_replace_keys_apply(
                commandHandle,
                wallet.Handle,
                did,
                CallbackHelper.TaskCompletingNoValueCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Stores a remote party's DID for a pairwise connection in the specified wallet.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The DID and optional associated parameters must be provided in the <paramref name="identityJson"/>
        /// parameter as a JSON string:
        /// </para>
        /// <code>
        /// {
        ///        "did": string, 
        ///        "verkey": string,
        ///        "crypto_type": string
        /// }
        /// </code>
        /// <para>The <c>did</c> member specifies the DID to store.  This value is required.</para>
        /// <para>The <c>verkey</c> member specifies the verification key and is optional.</para>
        /// <para>The <c>crypto_type</c> member specifies the type of cryptographic algorithm will be 
        /// used to generate they keys.  If not provided then ed22519 curve will be used.
        /// <note type="note">The only value currently supported for this member is 'ed25519'.</note>
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet to store the DID in.</param>
        /// <param name="identityJson">The identity JSON.</param>
        /// <returns>An asynchronous <see cref="Task"/> that  with no return value the completes when the operation completes.</returns>
        public static Task StoreTheirDidAsync(Wallet wallet, string identityJson)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(identityJson, "identityJson");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_store_their_did(
                commandHandle,
                wallet.Handle,
                identityJson,
                CallbackHelper.TaskCompletingNoValueCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Signs the provided message using the specified DID.
        /// </summary>
        /// <remarks>
        /// The DID specified in the <paramref name="did"/> parameter  must already be stored 
        /// in the <see cref="Wallet"/> specified in the <paramref name="wallet"/> parameter
        /// with a signing key in order to be able to sign a message.  See the 
        /// <see cref="CreateAndStoreMyDidAsync(Wallet, string)"/> method for details.
        /// </remarks>
        /// <param name="wallet">The wallet that contains the DID.</param>
        /// <param name="did">The DID to sign with.</param>
        /// <param name="msg">The message to sign.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a byte array that contains signed message when signing is complete.</returns>
        public static Task<byte[]> SignAsync(Wallet wallet, string did, byte[] msg)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNull(msg, "msg");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_sign(
                commandHandle,
                wallet.Handle,
                did,
                msg,
                msg.Length,
                _signCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
        
        /// <summary>
        /// Verifies a signature created by a key associated with the specified DID.
        /// </summary>
        /// <remarks>
        /// <para>If the wallet specified in the <paramref name="wallet"/> parameter contains a 
        /// verkey associated with the DID provided in the <paramref name="did"/> parameter and the
        /// value has not expired this verkey will be used to verify the signature. 
        /// </para>
        /// <para>On the other hand, if the verkey value in the wallet has expired or the verkey was not 
        /// stored in the wallet then the verkey is read from the ledger in the node pool specified in the 
        /// <paramref name="pool"/> parameter and the wallet will be updated with the new verkey if required. 
        /// </para>
        /// <para>For further information on registering a verkey for a DID see the <see cref="StoreTheirDidAsync(Wallet, string)"/>
        /// method and for information on the expiry of values in a wallet see the 
        /// <see cref="Wallet.CreateWalletAsync(string, string, string, string, string)"/>
        /// and <see cref="Wallet.OpenWalletAsync(string, string, string)"/> methods.
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet containing the DID of the signed message.</param>
        /// <param name="pool">The node pool to obtain the verkey from if required.</param>
        /// <param name="did">The DID used to sign the message.</param>
        /// <param name="msg">The message that was signed.</param>
        /// <param name="signature">The signature to verify.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to true if the message is valid, otherwise false.</returns>
        public static Task<bool> VerifySignatureAsync(Wallet wallet, Pool pool, string did, byte[] msg, byte[]signature)
        {
            ParamGuard.NotNull(wallet, "wallet");            
            ParamGuard.NotNull(pool, "pool");
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNull(msg, "msg");
            ParamGuard.NotNull(signature, "signature");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

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

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Encrypts the provided message with the public key of the specified DID.
        /// </summary>
        /// <remarks>
        /// <para>If the wallet specified in the <paramref name="wallet"/> parameter contains the public key
        /// associated with the DID specified in the <paramref name="did"/> parameter and the value has
        /// not expired then this key will be used for encryption.
        /// </para>
        /// <para>On the other hand, if the public key is not present in the wallet or has expired the public
        /// key will be read from the ledger in the node pool specified in the <paramref name="pool"/> 
        /// parameter and the wallet will be updated with the new public key if required.
        /// </para>
        /// <para>For further information on registering a public key for a DID see the 
        /// <see cref="StoreTheirDidAsync(Wallet, string)"/>method and for information on the expiry of 
        /// values in a wallet see the <see cref="Wallet.CreateWalletAsync(string, string, string, string, string)"/>
        /// and <see cref="Wallet.OpenWalletAsync(string, string, string)"/> methods.
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet containing the DID to use for encryption.</param>
        /// <param name="pool">The node pool to read the public key from if required.</param>
        /// <param name="myDid">The DID used to encrypt the message.</param>
        /// <param name="did">The DID the message is to be encrypted for.</param>
        /// <param name="msg">The message to encrypt.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to an <see cref="EncryptResult"/> once encryption is complete.</returns>
        public static Task<EncryptResult> EncryptAsync(Wallet wallet, Pool pool, string myDid, string did, byte[] msg)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNull(pool, "pool");
            ParamGuard.NotNullOrWhiteSpace(myDid, "myDid");
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNull(msg, "msg");

            var taskCompletionSource = new TaskCompletionSource<EncryptResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_encrypt(
                commandHandle,
                wallet.Handle,
                pool.Handle,
                myDid,
                did,
                msg,
                msg.Length,
                _encryptCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Decrypts the provided message using the public key associated with my DID.
        /// </summary>
        /// <remarks>
        /// <para>
        /// The DID specified in the <paramref name="myDid"/> parameter must have been previously created 
        /// with a secret key and stored in the wallet specified in the <paramref name="wallet"/> parameter or
        /// exist on the ledger on the node pool specified in the <paramref name="pool"/> parameter.
        /// </para>
        /// <para>
        /// For further information on storing a DID and its secret key see the 
        /// <see cref="CreateAndStoreMyDidAsync(Wallet, string)"/> method.
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet containing the DID and associated secret key to use for decryption.</param>
        /// <param name="pool">The pool to use for resolving keys associated with the <paramref name="did"/> if not present in the wallet.</param>
        /// <param name="myDid">The DID to use for decryption.</param>
        /// <param name="did">The DID of the encrypting party to use for verification.</param>
        /// <param name="encryptedMsg">The message to decrypt.</param>
        /// <param name="nonce">The nonce used to encrypt the message.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a byte array containing the decrypted message.</returns>
        public static Task<byte[]> DecryptAsync(Wallet wallet, Pool pool, string myDid, string did, byte[] encryptedMsg, byte[] nonce)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNull(pool, "pool");
            ParamGuard.NotNullOrWhiteSpace(myDid, "myDid");
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNull(encryptedMsg, "encryptedMsg");
            ParamGuard.NotNull(nonce, "nonce");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_decrypt(
                commandHandle,
                wallet.Handle,
                pool.Handle,
                myDid,
                did,
                encryptedMsg,
                encryptedMsg.Length,
                nonce,
                nonce.Length,
                _decryptCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Encrypts the provided message with the public key of the specified DID using the anonymous-encryption scheme.
        /// </summary>
        /// <remarks>
        /// <para>If the wallet specified in the <paramref name="wallet"/> parameter contains the public key
        /// associated with the DID specified in the <paramref name="did"/> parameter and the value has
        /// not expired then this key will be used for encryption.
        /// </para>
        /// <para>On the other hand, if the public key is not present in the wallet or has expired the public
        /// key will be read from the ledger in the node pool specified in the <paramref name="pool"/> 
        /// parameter and the wallet will be updated with the new public key if required.
        /// </para>
        /// <para>For further information on registering a public key for a DID see the 
        /// <see cref="StoreTheirDidAsync(Wallet, string)"/>method and for information on the expiry of 
        /// values in a wallet see the <see cref="Wallet.CreateWalletAsync(string, string, string, string, string)"/>
        /// and <see cref="Wallet.OpenWalletAsync(string, string, string)"/> methods.
        /// </para>
        /// </remarks>
        /// <param name="wallet">The wallet containing the DID to use for encryption.</param>
        /// <param name="pool">The node pool to read the public key from if required.</param>
        /// <param name="did">The DID the message is to be encrypted for.</param>
        /// <param name="msg">The message to encrypt.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a byte array containing the encrypted message once encryption is complete.</returns>
        public static Task<byte[]> EncryptSealedAsync(Wallet wallet, Pool pool, string did, byte[] msg)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNull(pool, "pool");
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNull(msg, "msg");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_encrypt_sealed(
                commandHandle,
                wallet.Handle,
                pool.Handle,
                did,
                msg,
                msg.Length,
                _encryptSealedCallback);

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Decrypts the provided message using the public key associated with the specified DID using the anonymous-encryption scheme.
        /// </summary>        
        /// <param name="wallet">The wallet containing the DID and associated secret key to use for decryption.</param>
        /// <param name="did">The DID of the encrypting party to use for verification.</param>
        /// <param name="encryptedMsg">The message to decrypt.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a byte array containing the decrypted message.</returns>
        public static Task<byte[]> DecryptSealedAsync(Wallet wallet, string did, byte[] encryptedMsg)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNull(encryptedMsg, "encryptedMsg");

            var taskCompletionSource = new TaskCompletionSource<byte[]>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_decrypt_sealed(
                commandHandle,
                wallet.Handle,
                did,
                encryptedMsg,
                encryptedMsg.Length,
                _decryptSealedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

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

            var commandResult = IndyNativeMethods.indy_create_key(
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

            var commandResult = IndyNativeMethods.indy_set_key_metadata(
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

            var commandResult = IndyNativeMethods.indy_get_key_metadata(
                commandHandle,
                wallet.Handle,
                verKey,
                _getKeyMetadataCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the verification key for the specified DID.
        /// </summary>
        /// <remarks>
        /// If the provided <paramref name="wallet"/> does not contain the verification key associated with the specified DID then 
        /// an attempt will be made to look up the key from the provided <paramref name="pool"/>. If resolved from the <paramref name="pool"/>
        /// then the DID and key will be automatically cached in the <paramref name="wallet"/>.
        /// <note type="note">
        /// The <see cref="CreateAndStoreMyDidAsync(Wallet, string)"/> and <see cref="CreateKeyAsync(Wallet, string)"/> methods both create
        /// similar wallet records so the returned verification key in all generic crypto and messaging functions.
        /// </note>
        /// </remarks>
        /// <param name="pool">The pool to use for resolving the DID if it does not exist in the <paramref name="wallet"/>.</param>
        /// <param name="wallet">The wallet to resolve the DID from.</param>
        /// <param name="did">The DID to get the verification key for.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the verification key associated with the DID.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if the DID could not be resolved from the <paramref name="wallet"/> and <paramref name="pool"/>.</exception>
        public static Task<string> KeyForDidAsync(Pool pool, Wallet wallet, string did)
        {
            ParamGuard.NotNull(pool, "pool");
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_key_for_did(
                commandHandle,
                pool.Handle,
                wallet.Handle,
                did,
                _keyForDidCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Sets the endpoint details for the specified DID.
        /// </summary>
        /// <param name="wallet">The wallet containing the DID.</param>
        /// <param name="did">The DID to set the endpoint details on.</param>
        /// <param name="address">The address of the endpoint.</param>
        /// <param name="transportKey">The transport key.</param>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation completes.</returns>
        /// <exception cref="InvalidStructureException">Thrown if the <paramref name="did"/> or <paramref name="transportKey"/> values are malformed.</exception>
        public static Task SetEndpointForDidAsync(Wallet wallet, string did, string address, string transportKey)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");
            ParamGuard.NotNullOrWhiteSpace(address, "address");
            ParamGuard.NotNullOrWhiteSpace(transportKey, "transportKey");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_set_endpoint_for_did(
                commandHandle,
                wallet.Handle,
                did,
                address,
                transportKey,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the endpoint details for the specified DID.
        /// </summary>
        /// <param name="wallet">The wallet containing the DID.</param>
        /// <param name="did">The DID to get the endpoint data for.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to an <see cref="EndpointForDidResult"/> containing the endpoint information 
        /// associated with the DID.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if the <paramref name="did"/> does not exist in the <paramref name="wallet"/>.</exception>
        public static Task<EndpointForDidResult> GetEndpointForDidAsync(Wallet wallet, string did)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<EndpointForDidResult>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_get_endpoint_for_did(
                commandHandle,
                wallet.Handle,
                did,
                _getEndpointForDidCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Sets metadata for the specified DID.
        /// </summary>
        /// <remarks>
        /// Any existing metadata stored for the DID will be replaced.
        /// </remarks>
        /// <param name="wallet">The wallet containing the DID.</param>
        /// <param name="did">The DID to set the metadata on.</param>
        /// <param name="metadata">The metadata to store.</param>
        /// <returns>An asynchronous <see cref="Task"/> that completes when the operation completes.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if the <paramref name="wallet"/> does not contain the specified <paramref name="did"/>.</exception>
        /// <exception cref="InvalidStructureException">Thrown if the value provided to the <paramref name="did"/> parameter is malformed.</exception>
        public static Task SetDidMetadataAsync(Wallet wallet, string did, string metadata)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<bool>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_set_did_metadata(
                commandHandle,
                wallet.Handle,
                did,
                metadata,
                CallbackHelper.TaskCompletingNoValueCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }

        /// <summary>
        /// Gets the metadata associated with the specified DID.
        /// </summary>
        /// <param name="wallet">The wallet that contains the DID.</param>
        /// <param name="did">The DID to get the metadata for.</param>
        /// <returns>An asynchronous <see cref="Task{T}"/> that resolves to a string containing the metadata associated with the DID.</returns>
        /// <exception cref="WalletValueNotFoundException">Thrown if the wallet does not contain the specified <paramref name="did"/> or the DID did not have any metadata.</exception>
        /// <exception cref="InvalidStructureException">Thrown if the value provided in the <paramref name="did"/> parameter is malformed.</exception>
        public static Task<string> GetDidMetadataAsync(Wallet wallet, string did)
        {
            ParamGuard.NotNull(wallet, "wallet");
            ParamGuard.NotNullOrWhiteSpace(did, "did");

            var taskCompletionSource = new TaskCompletionSource<string>();
            var commandHandle = PendingCommands.Add(taskCompletionSource);

            var commandResult = IndyNativeMethods.indy_get_did_metadata(
                commandHandle,
                wallet.Handle,
                did,
                _getDidMetadataCompletedCallback
                );

            CallbackHelper.CheckResult(commandResult);

            return taskCompletionSource.Task;
        }
    }
}
