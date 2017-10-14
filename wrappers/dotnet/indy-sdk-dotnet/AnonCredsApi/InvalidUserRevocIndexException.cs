namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when an invalid user revocation registry index is used.
    /// </summary>
    public class InvalidUserRevocIndexException : IndyException
    {
        const string message = "The user revocation registry index specified is invalid.";

        internal InvalidUserRevocIndexException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
