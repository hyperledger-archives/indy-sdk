using System;
using System.Runtime.InteropServices;
using static Hyperledger.Indy.Utils.CallbackHelper;

namespace Hyperledger.Indy.CryptoApi
{
    internal static class NativeMethods
    {
        /// <summary>
        /// Creates keys pair and stores in the wallet.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="key_json">Key information as json.</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_create_key(int command_handle, IntPtr wallet_handle, string key_json, CreateKeyCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_create_key.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="verkey">The verification key of the generated key pair.</param>
        internal delegate void CreateKeyCompletedDelegate(int xcommand_handle, int err, string verkey);

        /// <summary>
        /// Saves/replaces the meta information for the giving key in the wallet.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="verkey">the key (verkey, key id) to store metadata.</param>
        /// <param name="metadata">the meta information that will be store with the key.</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_set_key_metadata(int command_handle, IntPtr wallet_handle, string verkey, string metadata, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Retrieves the meta information for the giving key in the wallet.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="verkey">The key (verkey, key id) to retrieve metadata.</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_key_metadata(int command_handle, IntPtr wallet_handle, string verkey, GetKeyMetadataCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_get_key_metadata.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="metadata">The metadata associated with the key-pair.</param>
        internal delegate void GetKeyMetadataCompletedDelegate(int xcommand_handle, int err, string metadata);

        /// <summary>
        /// Signs a message with a key.
        /// </summary>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="wallet_handle">wallet handler (created by open_wallet).</param>
        /// <param name="my_vk">id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did</param>
        /// <param name="msg_raw">a pointer to first byte of message to be signed</param>
        /// <param name="msg_len">a message length</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_sign(int command_handle, IntPtr wallet_handle, string my_vk, byte[] msg_raw, int msg_len, SignCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_crypto_sign.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="signature_raw">A pointer to the signature data.</param>
        /// <param name="signature_len">The length of the signature data in bytes.</param>
        internal delegate void SignCompletedDelegate(int xcommand_handle, int err, IntPtr signature_raw, int signature_len);

        /// <summary>
        /// Verify a signature with a verkey.
        /// </summary>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="their_vk">verkey to use</param>
        /// <param name="msg_raw">a pointer to first byte of message to be signed</param>
        /// <param name="msg_len">message length</param>
        /// <param name="signature_raw">a pointer to first byte of signature to be verified</param>
        /// <param name="signature_len">signature length</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_verify(int command_handle, string their_vk, byte[] msg_raw, int msg_len, byte[] signature_raw, int signature_len, VerifyCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_crypto_verify.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="valid">True if the signature is valid for the message, otherwise false.</param>
        internal delegate void VerifyCompletedDelegate(int xcommand_handle, int err, bool valid);

        /// <summary>
        /// Encrypt a message by authenticated-encryption scheme.
        /// </summary>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="wallet_handle">wallet handle (created by open_wallet).</param>
        /// <param name="my_vk">id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did</param>
        /// <param name="their_vk">id (verkey) of their key</param>
        /// <param name="msg_raw">a pointer to first byte of message that to be encrypted</param>
        /// <param name="msg_len">message length</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_box(int command_handle, IntPtr wallet_handle, string my_vk, string their_vk, byte[] msg_raw, int msg_len, BoxCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_crypto_box.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="encrypted_msg_raw">A pointer to the encrypted message data.</param>
        /// <param name="encrypted_msg_len">The length of the encrypted message data in bytes.</param>
        /// <param name="nonce_raw">A pointer to the nonce data.</param>
        /// <param name="nonce_len">The length of the nonce data in bytes.</param>
        internal delegate void BoxCompletedDelegate(int xcommand_handle, int err, IntPtr encrypted_msg_raw, int encrypted_msg_len, IntPtr nonce_raw, int nonce_len);

        /// <summary>
        /// Decrypt a message by authenticated-encryption scheme.
        /// </summary>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="wallet_handle">wallet handler (created by open_wallet).</param>
        /// <param name="my_vk">id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did</param>
        /// <param name="their_vk">id (verkey) of their key</param>
        /// <param name="encrypted_msg_raw">a pointer to first byte of message that to be decrypted</param>
        /// <param name="encrypted_msg_len">message length</param>
        /// <param name="nonce_raw">a pointer to first byte of nonce that encrypted message</param>
        /// <param name="nonce_len">nonce length</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_box_open(int command_handle, IntPtr wallet_handle, string my_vk, string their_vk, byte[] encrypted_msg_raw, int encrypted_msg_len, byte[] nonce_raw, int nonce_len, BoxOpenCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_crypto_box_open.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="decrypted_msg_raw">A pointer to the decrypted message data.</param>
        /// <param name="decrypted_msg_len">The length of the decrypted message data in bytes.</param>
        internal delegate void BoxOpenCompletedDelegate(int xcommand_handle, int err, IntPtr decrypted_msg_raw, int decrypted_msg_len);

        /// <summary>
        /// Encrypts a message by anonymous-encryption scheme.
        /// </summary>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="their_vk">id (verkey) of their key</param>
        /// <param name="msg_raw">a pointer to first byte of message that to be encrypted</param>
        /// <param name="msg_len">message length</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_box_seal(int command_handle, string their_vk, byte[] msg_raw, int msg_len, BoxSealCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_crypto_box_seal.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="encrypted_msg_raw">A pointer to the encrypted message data.</param>
        /// <param name="encrypted_msg_len">The encrypted message data length in bytes.</param>
        internal delegate void BoxSealCompletedDelegate(int xcommand_handle, int err, IntPtr encrypted_msg_raw, int encrypted_msg_len);

        /// <summary>
        /// Decrypts a message by anonymous-encryption scheme.
        /// </summary>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="wallet_handle">wallet handler (created by open_wallet).</param>
        /// <param name="my_vk">id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did</param>
        /// <param name="encrypted_msg_raw">a pointer to first byte of message that to be decrypted</param>
        /// <param name="encrypted_msg_len">message length</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_box_seal_open(int command_handle, IntPtr wallet_handle, string my_vk, byte[] encrypted_msg_raw, int encrypted_msg_len, BoxSealOpenCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_crypto_box_seal_open.
        /// </summary>
        /// <param name="xcommand_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="decrypted_msg_raw">A pointer to the decrypted message data.</param>
        /// <param name="decrypted_msg_len">The length of the decrypted message data in bytes.</param>
        internal delegate void BoxSealOpenCompletedDelegate(int xcommand_handle, int err, IntPtr decrypted_msg_raw, int decrypted_msg_len);
    }
}
