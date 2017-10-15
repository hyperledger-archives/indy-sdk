namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when creating a wallet and a wallet with the same name already exists.
    /// </summary>
    public class WalletExistsException : IndyException
    {
        const string message = "A wallet with the specified name already exists.";

        /// <summary>
        /// Initializes a new WalletExistsException.
        /// </summary>
        internal WalletExistsException() : base(message, (int)ErrorCode.WalletAlreadyExistsError)
        {

        }
    }

}
