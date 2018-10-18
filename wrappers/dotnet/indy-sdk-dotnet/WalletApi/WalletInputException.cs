namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Exception thrown when the input provided to a wallet operation is invalid.
    /// </summary>
    public class WalletInputException : IndyException
    {
        const string message = "The input provided to a wallet operation is invalid.";

        /// <summary>
        /// Initializes a new WalletInputException.
        /// </summary>
        internal WalletInputException() : base(message, (int)ErrorCode.WalletInputError)
        {

        }
    }

}
