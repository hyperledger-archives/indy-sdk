namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when an attempt is made to use a closed wallet.
    /// </summary>
    public class WalletClosedException : IndyException
    {
        const string message = "The wallet is closed and cannot be used.";

        internal WalletClosedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
