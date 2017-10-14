namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to open a wallet that was already opened.
    /// </summary>
    public class WalletAlreadyOpenedException : IndyException
    {
        const string message = "The wallet is already open.";

        internal WalletAlreadyOpenedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
