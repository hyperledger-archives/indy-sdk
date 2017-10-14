namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to use a wallet with the wrong pool.
    /// </summary>
    public class WrongWalletForPoolException : IndyException
    {
        const string message = "The wallet specified is not compatible with the open pool.";

        internal WrongWalletForPoolException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
