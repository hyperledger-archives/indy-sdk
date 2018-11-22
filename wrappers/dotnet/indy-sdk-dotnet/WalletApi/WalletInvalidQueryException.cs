namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to use a wallet with the wrong pool.
    /// </summary>
    public class WalletInvalidQueryException : IndyException
    {
        const string message = "The wallet query provided was invalid.";

        /// <summary>
        /// Initializes a new WalletInvalidQueryException.
        /// </summary>
        internal WalletInvalidQueryException() : base(message, (int)ErrorCode.WalletQueryError)
        {

        }
    }

}
