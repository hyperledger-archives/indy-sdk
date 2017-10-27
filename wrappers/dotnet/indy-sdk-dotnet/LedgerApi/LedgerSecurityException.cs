namespace Hyperledger.Indy.LedgerApi
{
    /// <summary>
    /// Exception thrown when attempting to send a transaction without the necessary privileges.
    /// </summary>
    public class LedgerSecurityException : IndyException
    {
        const string message = "The transaction cannot be sent as the privileges for the current pool connection don't allow it.";

        /// <summary>
        /// Initializes a new LedgerSecurityException.
        /// </summary>
        internal LedgerSecurityException() : base(message, (int)ErrorCode.LedgerSecurityError)
        {

        }
    }

}
