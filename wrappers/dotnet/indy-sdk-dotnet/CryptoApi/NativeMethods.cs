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
        internal static extern int indy_create_key(int command_handle, int wallet_handle, string key_json, CreateKeyCompletedDelegate cb);

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
        internal static extern int indy_set_key_metadata(int command_handle, int wallet_handle, string verkey, string metadata, IndyMethodCompletedDelegate cb);

        /// <summary>
        /// Retrieves the meta information for the giving key in the wallet.
        /// </summary>
        /// <param name="command_handle">Command handle to map callback to caller context.</param>
        /// <param name="wallet_handle">Wallet handle (created by open_wallet).</param>
        /// <param name="verkey">The key (verkey, key id) to retrieve metadata.</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_get_key_metadata(int command_handle, int wallet_handle, string verkey, GetKeyMetadataCompletedDelegate cb);

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
        /// <param name="signer_vk">id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did</param>
        /// <param name="message_raw">a pointer to first byte of message to be signed</param>
        /// <param name="message_len">a message length</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_sign(int command_handle, int wallet_handle, string signer_vk, byte[] message_raw, int message_len, SignCompletedDelegate cb);

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
        /// <param name="signer_vk">verkey to use</param>
        /// <param name="message_raw">a pointer to first byte of message to be signed</param>
        /// <param name="message_len">message length</param>
        /// <param name="signature_raw">a pointer to first byte of signature to be verified</param>
        /// <param name="signature_len">signature length</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_verify(int command_handle, string signer_vk, byte[] message_raw, int message_len, byte[] signature_raw, int signature_len, VerifyCompletedDelegate cb);

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
        /// <returns>The crypto auth crypt.</returns>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="wallet_handle">wallet handler (created by open_wallet).</param>
        /// <param name="sender_vk">id (verkey) of my key.</param>
        /// <param name="recipient_vk">id (verkey) of their key</param>
        /// <param name="msg_data">a pointer to first byte of message that to be encrypted</param>
        /// <param name="msg_len">message length</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_auth_crypt(int command_handle, int wallet_handle, string sender_vk, string recipient_vk, byte[] msg_data, int msg_len, EncryptCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_crypto_auth_crypt and indy_crypto_anon_crypt
        /// </summary>
        internal delegate void EncryptCompletedDelegate(int xcommand_handle, int err, IntPtr encrypted_msg, int encrypted_len);

        /// <summary>
        /// Decrypt a message by authenticated-encryption scheme.
        /// </summary>
        /// <returns>sender verkey and decrypted message</returns>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="wallet_handle">wallet handler (created by open_wallet).</param>
        /// <param name="recipient_vk">id (verkey) of my key.</param>
        /// <param name="encrypted_msg">Encrypted message.</param>
        /// <param name="encrypted_len">Encrypted length.</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_auth_decrypt(int command_handle, int wallet_handle, string recipient_vk, byte[] encrypted_msg, int encrypted_len, AuthDecryptCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_crypto_auth_decrypt
        /// </summary>
        internal delegate void AuthDecryptCompletedDelegate(int command_handle, int err, string sender_vk, IntPtr msg_data, int msg_len);

        /// <summary>
        /// Encrypts a message by anonymous-encryption scheme.
        /// </summary>
        /// <returns>an encrypted message</returns>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="recipient_vk">id (verkey) of their key</param>
        /// <param name="msg_data">message data to be encrypted</param>
        /// <param name="msg_len">message length.</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_anon_crypt(int command_handle, string recipient_vk, byte[] msg_data, int msg_len, EncryptCompletedDelegate cb);

        /// <summary>
        /// Decrypts a message by anonymous-encryption scheme.
        /// </summary>
        /// <param name="command_handle">command handle to map callback to user context.</param>
        /// <param name="wallet_handle">wallet handler (created by open_wallet).</param>
        /// <param name="recipient_vk">id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did</param>
        /// <param name="encrypted_msg">a pointer to first byte of message that to be decrypted</param>
        /// <param name="encrypted_len">message length</param>
        /// <param name="cb">Callback that takes command result as parameter.</param>
        /// <returns>0 if the command was initiated successfully.  Any non-zero result indicates an error.</returns>
        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_crypto_anon_decrypt(int command_handle, int wallet_handle, string recipient_vk, byte[] encrypted_msg, int encrypted_len, AnonDecryptCompletedDelegate cb);

        /// <summary>
        /// Delegate to be used on completion of calls to indy_crypto_anon_decrypt.
        /// </summary>
        /// <param name="command_handle">The handle for the command that initiated the callback.</param>
        /// <param name="err">The outcome of execution of the command.</param>
        /// <param name="msg_data">A pointer to the decrypted message data.</param>
        /// <param name="msg_len">The length of the decrypted message data in bytes.</param>
        internal delegate void AnonDecryptCompletedDelegate(int command_handle, int err, IntPtr msg_data, int msg_len);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_pack_message(int command_handle, int wallet_handle, byte[] message, int message_len, string receiver_keys, string sender, PackMessageCompletedDelegate cb);

        internal delegate void PackMessageCompletedDelegate(int command_handle, int err, IntPtr jwe_data, int jwe_len);

        [DllImport(Consts.NATIVE_LIB_NAME, CharSet = CharSet.Ansi, BestFitMapping = false, ThrowOnUnmappableChar = true)]
        internal static extern int indy_unpack_message(int command_handle, int wallet_handle, byte[] jwe_data, int jwe_len, UnpackMessageCompletedDelegate cb);

        internal delegate void UnpackMessageCompletedDelegate(int command_handle, int err, IntPtr res_json_data, int res_json_len);
    }
}
