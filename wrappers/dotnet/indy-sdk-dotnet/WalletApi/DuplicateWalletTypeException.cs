namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when registering a wallet type that has already been registered.
    /// </summary>
    public class DuplicateWalletTypeException : IndyException
    {
        const string message = "A wallet type with the specified name has already been registered.";

        /// <summary>
        /// Initializes a new DuplicateWalletTypeException.
        /// </summary>
        internal DuplicateWalletTypeException() : base(message, (int)ErrorCode.WalletTypeAlreadyRegisteredError)
        {

        }
    }

}
