namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when an attempt is made to use a closed or invalid wallet.
    /// </summary>
    public class InvalidWalletException : IndyException
    {
        const string message = "The wallet is closed or invalid and cannot be used.";

        /// <summary>
        /// Initializes a new WalletClosedException.
        /// </summary>
        internal InvalidWalletException() : base(message, (int)ErrorCode.WalletInvalidHandle)
        {

        }
    }

}
