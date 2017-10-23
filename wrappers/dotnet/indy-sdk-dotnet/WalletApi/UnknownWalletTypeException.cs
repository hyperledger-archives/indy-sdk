namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when attempting to open a wallet with a type that has not been registered.
    /// </summary>
    public class UnknownWalletTypeException : IndyException
    {
        const string message = "The wallet type specified has not been registered.";

        /// <summary>
        /// Initializes a new UnknownWalletTypeException.
        /// </summary>
        internal UnknownWalletTypeException() : base(message, (int)ErrorCode.WalletUnknownTypeError)
        {

        }
    }

}
