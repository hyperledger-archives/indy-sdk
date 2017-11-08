using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.SignusApi
{
    internal static class NativeMethods
    {
        /// <summary>
        /// Creates keys (signing and encryption keys) for a new
        /// DID (owned by the caller of the library).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet)</param>
        /// <param name="did_json">Identity information as json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_and_store_my_did(int command_handle, IntPtr wallet_handle, string did_json, CreateAndStoreMyDidCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_create_and_store_my_did.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="did">The created DID.</param>
        /// <param name="verkey">The verification key for the signature.</param>
        internal delegate void CreateAndStoreMyDidCompletedDelegate(int xcommand_handle, int err, string did, string verkey);

        /// <summary>
        /// Generates new keys (signing and encryption keys) for an existing
        /// DID (owned by the caller of the library).
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="did">Id of Identity stored in secured Wallet.</param>
        /// <param name="identity_json">Identity information as json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_replace_keys_start(int command_handle, IntPtr wallet_handle, string did, string identity_json, ReplaceKeysStartCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_replace_keys_start.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="verkey">The key for verification of signature.</param>
        internal delegate void ReplaceKeysStartCompletedDelegate(int xcommand_handle, int err, string verkey);

        /// <summary>
        /// Apply temporary keys as main for an existing DID (owned by the caller of the library).
        /// </summary>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="wallet_handle">wallet handler (created by open_wallet).</param>
        /// <param name="did">Id of Identity stored in secured Wallet.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_replace_keys_apply(int command_handle, IntPtr wallet_handle, string did, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Saves their DID for a pairwise connection in a secured Wallet,
        /// so that it can be used to verify transaction.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet)</param>
        /// <param name="identity_json">Identity information as json.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_store_their_did(int command_handle, IntPtr wallet_handle, string identity_json, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Signs a message by a signing key associated with my DID.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="did">signing DID</param>
        /// <param name="msg_raw">The message to be signed.</param>
        /// <param name="msg_len">The length of the message array.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_sign(int command_handle, IntPtr wallet_handle, string did, byte[] msg_raw, int msg_len, SignCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_sign.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="signature_raw">The raw signature bytes.</param>
        /// <param name="signature_len">The length of the signature byte array.</param>
        internal delegate void SignCompletedDelegate(int xcommand_handle, int err, IntPtr signature_raw, int signature_len);

        /// <summary>
        /// Verify a signature created by a key associated with a DID.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="pool_handle">pool handle.</param>
        /// <param name="did">DID that signed the message</param>
        /// <param name="msg_raw">The message</param>
        /// <param name="msg_len">The length of the message array.</param>
        /// <param name="signature_raw">The signature.</param>
        /// <param name="signature_len">The length of the signature array.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_verify_signature(int command_handle, IntPtr wallet_handle, IntPtr pool_handle, string did, byte[] msg_raw, int msg_len, byte[] signature_raw, int signature_len, VerifySignatureCopmpletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_verify_signature.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="valid">true if the signature is valid, otherwise false</param>
        internal delegate void VerifySignatureCopmpletedDelegate(int xcommand_handle, int err, bool valid);

        /// <summary>
        /// Encrypts a message by a public key associated with a DID.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="pool_handle"></param>
        /// <param name="my_did">encrypting DID</param>
        /// <param name="did">encrypting DID (??)</param>
        /// <param name="msg_raw">The message to encrypt.</param>
        /// <param name="msg_len">The length of the message byte array.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_encrypt(int command_handle, IntPtr wallet_handle, IntPtr pool_handle, string my_did, string did, byte[] msg_raw, int msg_len, EncryptCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_encrypt.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="encrypted_msg_raw">The encrypted message as an array of bytes.</param>
        /// <param name="encrypted_msg_len">The length of the encrypted message byte array.</param>
        /// <param name="nonce_raw">The nonce as an array of bytes.</param>
        /// <param name="nonce_len">The length of the nonce byte array.</param>
        internal delegate void EncryptCompletedDelegate(int xcommand_handle, int err, IntPtr encrypted_msg_raw, int encrypted_msg_len, IntPtr nonce_raw, int nonce_len);



        /// <summary>
        /// Encrypts a message by a public key associated with a DID.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="pool_handle"></param>
        /// <param name="did">encrypting DID (??)</param>
        /// <param name="message_raw">The message to encrypt.</param>
        /// <param name="message_len">The length of the message byte array.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_encrypt_sealed(int command_handle, IntPtr wallet_handle, IntPtr pool_handle, string did, byte[] message_raw, int message_len, EncryptSealedCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_encrypt_sealed.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="encrypted_msg_raw">The encrypted message as an array of bytes.</param>
        /// <param name="encrypted_msg_len">The length of the encrypted message byte array.</param>
        internal delegate void EncryptSealedCompletedDelegate(int xcommand_handle, int err, IntPtr encrypted_msg_raw, int encrypted_msg_len);

        /// <summary>
        /// Decrypts a message encrypted by a public key associated with my DID.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="pool_handle">pool handle (created by open_pool).</param>
        /// <param name="my_did">DID</param>
        /// <param name="did">DID that signed the message</param>
        /// <param name="encrypted_msg_raw">encrypted message as a byte array.</param>
        /// <param name="encrypted_msg_len">The length of the message byte array.</param>
        /// <param name="nonce_raw">nonce that encrypted message as a byte array.</param>
        /// <param name="nonce_len">The length of the nonce byte array.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_decrypt(int command_handle, IntPtr wallet_handle, IntPtr pool_handle, string my_did, string did, byte[] encrypted_msg_raw, int encrypted_msg_len, byte[] nonce_raw, int nonce_len, DecryptCompletedDelegate cb);

        /// <summary>
        /// Decrypts a message encrypted by a public key associated with my DID.
        /// </summary>
        /// <param name="command_handle">The handle for the command that will be passed to the callback.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="did">DID that encrypted the message</param>
        /// <param name="encrypted_msg_raw">encrypted message as a byte array.</param>
        /// <param name="encrypted_msg_len">The length of the message byte array.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_decrypt_sealed(int command_handle, IntPtr wallet_handle, string did, byte[] encrypted_msg_raw, int encrypted_msg_len, DecryptCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_decrypt and indy_decrypt_sealed.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="decrypted_msg_raw">The decrypted message as an array of bytes.</param>
        /// <param name="decrypted_msg_len">The length of the decrypted message byte array.</param>
        internal delegate void DecryptCompletedDelegate(int xcommand_handle, int err, IntPtr decrypted_msg_raw, int decrypted_msg_len);

        /// <summary>
        /// Returns ver key (key id) for the given DID.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="pool_handle">Pool handle (created by open_pool).</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="did">The DID to resolve key.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_key_for_did(int command_handle, IntPtr pool_handle, IntPtr wallet_handle, string did, SignusKeyForDidCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_key_for_did.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="key">The verification key associated with the DID.</param>
        internal delegate void SignusKeyForDidCompletedDelegate(int xcommand_handle, int err, string key);

        /// <summary>
        /// Sets the endpoint information for the given DID.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="did">The DID to resolve endpoint.</param>
        /// <param name="address">The address of the endpoint.</param>
        /// <param name="transportKey">The key for the transport.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_set_endpoint_for_did(int command_handle, IntPtr wallet_handle, string did, string address, string transportKey, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Gets the endpoint information for the given DID.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="did">The DID to set the endpoint on.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_endpoint_for_did(int command_handle, IntPtr wallet_handle, string did, SignusGetEndpointForDidCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_get_endpoint_for_did.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="endpoint">The endpoint address associated with the DID.</param>
        /// <param name="transport_vk">The transport verification key associated with the DID.</param>
        internal delegate void SignusGetEndpointForDidCompletedDelegate(int xcommand_handle, int err, string endpoint, string transport_vk);

        /// <summary>
        /// Saves/replaces the meta information for the giving DID in the wallet.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="did">the DID to store metadata.</param>
        /// <param name="metadata">the meta information that will be store with the DID.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_set_did_metadata(int command_handle, IntPtr wallet_handle, string did, string metadata, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Retrieves the meta information for the giving DID in the wallet.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="did">The DID to retrieve metadata.</param>
        /// <param name="cb">The function that will be called when the asynchronous call is complete.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_did_metadata(int command_handle, IntPtr wallet_handle, string did, SignusGetDidMetadataCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_get_did_metadata.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="metadata">The metadata associated with the DID.</param>
        internal delegate void SignusGetDidMetadataCompletedDelegate(int xcommand_handle, int err, string metadata);

    }
}
