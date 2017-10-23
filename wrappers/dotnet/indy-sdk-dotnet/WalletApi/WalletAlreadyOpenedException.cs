namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to open a wallet that was already opened.
    /// </summary>
    public class WalletAlreadyOpenedException : IndyException
    {
        const string message = "The wallet is already open.";

        /// <summary>
        /// Initializes a new WalletAlreadyOpenedException.
        /// </summary>
        internal WalletAlreadyOpenedException() : base(message, (int)ErrorCode.WalletAlreadyOpenedError)
        {

        }
    }

}
