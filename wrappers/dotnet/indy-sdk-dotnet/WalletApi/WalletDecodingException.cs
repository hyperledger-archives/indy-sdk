namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when decoding of wallet data during input/output failed.
    /// </summary>
    public class WalletDecodingException : IndyException
    {
        const string message = "Decoding of wallet data during input/output failed.";

        /// <summary>
        /// Initializes a new WalletDecodingException.
        /// </summary>
        internal WalletDecodingException() : base(message, (int)ErrorCode.WalletDecodingError)
        {

        }
    }

}
