namespace Hyperledger.Indy.DidApi
{
    /// <summary>
    /// Exception thrown when an unknown crypto format is used for DID entity keys.
    /// </summary>
    public class UnknownCryptoTypeException : IndyException
    {
        const string message = "An unknown crypto format has been used for a DID entity key.";

        /// <summary>
        /// Initializes a new UnknownCryptoTypeException.
        /// </summary>
        internal UnknownCryptoTypeException() : base(message, (int)ErrorCode.UnknownCryptoTypeError)
        {

        }
    }

}
