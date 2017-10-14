namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating that the SDK library experienced an unexpected internal error.
    /// </summary>
    public class InvalidStateException : IndyException
    {
        private const string message = "The SDK library experienced an unexpected internal error.";

        internal InvalidStateException(int sdkErrorCode) : base(message, sdkErrorCode)
        {
        }
    }

}
