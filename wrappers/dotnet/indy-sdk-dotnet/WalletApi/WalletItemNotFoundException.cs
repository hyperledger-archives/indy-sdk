namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when value with the specified key doesn't exists in the wallet from which it was requested.
    /// </summary>
    /// <seealso cref="Hyperledger.Indy.IndyException" />
    public class WalletItemNotFoundException : IndyException
    {
        private const string message =
            "No value with the specified key exists in the wallet from which it was requested.";

        internal WalletItemNotFoundException() : base(message, (int)ErrorCode.WalletItemNotFoundError)
        {
        }
    }
}
