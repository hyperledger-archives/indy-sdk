namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when requesting a value from a wallet and the specified key does not exist.
    /// </summary>
    public class WalletValueNotFoundException : IndyException
    {
        const string message = "No value with the specified key exists in the wallet from which it was requested.";

        /// <summary>
        /// Initializes a new WalletValueNotFoundException.
        /// </summary>
        internal WalletValueNotFoundException() : base(message, (int)ErrorCode.WalletNotFoundError)
        {

        }
    }

}
