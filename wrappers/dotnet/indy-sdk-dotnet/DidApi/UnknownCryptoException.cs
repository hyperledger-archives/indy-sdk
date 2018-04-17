namespace Hyperledger.Indy.DidApi
{
    /// <summary>
    /// Exception thrown when an unknown crypto format is used for DID entity keys.
    /// </summary>
    public class UnknownCryptoException : IndyException
    {
        const string message = "An unknown crypto format has been used for a DID entity key.";

        /// <summary>
        /// Initializes a new UnknownCryptoException.
        /// </summary>
        internal UnknownCryptoException() : base(message, (int)ErrorCode.SignusUnknownCryptoError)
        {

        }
    }

}
