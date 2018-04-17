namespace Hyperledger.Indy.CryptoApi
{
    /// <summary>
    /// Auth decrypt result.
    /// </summary>
    public class AuthDecryptResult
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="T:Hyperledger.Indy.CryptoApi.AuthDecryptResult"/> class.
        /// </summary>
        /// <param name="their_vk">Their vk.</param>
        /// <param name="messageData">Message data.</param>
        public AuthDecryptResult(string their_vk, byte[] messageData)
        {
            TheirVk = their_vk;
            MessageData = messageData;
        }

        /// <summary>
        /// Gets the decrypted message.
        /// </summary>
        public byte[] MessageData { get; private set; }

        /// <summary>
        /// Gets the sender verkey
        /// </summary>
        /// <value>Their verkey.</value>
        public string TheirVk { get; private set; }
    }
}
