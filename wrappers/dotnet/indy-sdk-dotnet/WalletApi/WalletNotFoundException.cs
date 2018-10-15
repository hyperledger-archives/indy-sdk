namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to open a wallet that does not exist.
    /// </summary>
    public class WalletNotFoundException : IndyException
    {
        const string message = "The wallet does not exist.";

        /// <summary>
        /// Initializes a new WalletNotFoundException.
        /// </summary>
        internal WalletNotFoundException() : base(message, (int)ErrorCode.WalletNotFoundError)
        {

        }
    }

}
