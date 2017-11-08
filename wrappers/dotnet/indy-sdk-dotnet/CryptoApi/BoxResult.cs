using System;
using System.Collections.Generic;
using System.Text;

namespace Hyperledger.Indy.CryptoApi
{
    /// <summary>
    /// The result of calling the <see cref="Crypto.BoxAsync(WalletApi.Wallet, string, string, byte[])"/> method.
    /// </summary>
    public class BoxResult
    {
        /// <summary>
        /// Initializes a new BoxResult.
        /// </summary>
        /// <param name="encryptedMessage">The encrpyted message.</param>
        /// <param name="nonce">The nonce.</param>
        public BoxResult(byte[] encryptedMessage, byte[] nonce)
        {
            EncryptedMessage = encryptedMessage;
            Nonce = nonce;
        }

        /// <summary>
        /// Gets the encrypted message.
        /// </summary>
        public byte[] EncryptedMessage { get; private set; }

        /// <summary>
        /// Gets the nonce.
        /// </summary>
        public byte[] Nonce { get; private set; }
    }
}
