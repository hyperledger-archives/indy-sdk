namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when an error occurred during encryption-related operations.
    /// </summary>
    public class WalletEncryptionException : IndyException
    {
        const string message = "An error occurred during encryption-related operations.";

        /// <summary>
        /// Initializes a new WalletEncryptionException.
        /// </summary>
        internal WalletEncryptionException() : base(message, (int)ErrorCode.WalletEncryptionError)
        {

        }
    }

}
