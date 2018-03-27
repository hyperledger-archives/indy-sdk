using System;

namespace Hyperledger.Indy.DidApi
{
    /// <summary>
    /// The result of encryption.
    /// </summary>
    public sealed class EncryptResult
    {
        /// <summary>
        /// Initializes a new EncryptionResult.
        /// </summary>
        /// <param name="encryptedMsg">The encrypted message.</param>
        /// <param name="nonce">The nonce.</param>
        internal EncryptResult(byte[] encryptedMsg, byte[] nonce)
        {
            EncryptedMsg = encryptedMsg ?? throw new ArgumentNullException("encryptedMsg");
            Nonce = nonce ?? throw new ArgumentNullException("nonce");
        }

        /// <summary>
        /// Gets the encrypted message.
        /// </summary>
        public byte[] EncryptedMsg { get; }

        /// <summary>
        /// Gets the nonce.
        /// </summary>
        public byte[] Nonce { get; }

    }
}
