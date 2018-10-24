namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when a storage error occurs during a wallet operation.
    /// </summary>
    public class WalletStorageException : IndyException
    {
        const string message = "A storage error occurred during the wallet operation.";

        /// <summary>
        /// Initializes a new WalletStorageException.
        /// </summary>
        internal WalletStorageException() : base(message, (int)ErrorCode.WalletStorageError)
        {

        }
    }

}
