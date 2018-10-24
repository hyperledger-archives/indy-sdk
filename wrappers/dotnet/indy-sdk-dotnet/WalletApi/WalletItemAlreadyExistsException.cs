namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when value with the specified key already exists in the wallet.
    /// </summary>
    /// <seealso cref="Hyperledger.Indy.IndyException" />
    public class WalletItemAlreadyExistsException : IndyException
    {
        private const string message =
            "The specified item already exists in the wallet.";

        internal WalletItemAlreadyExistsException() : base(message, (int)ErrorCode.WalletItemAlreadyExistsError)
        {
        }
    }
}
