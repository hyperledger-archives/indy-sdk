namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when a credential has been revoked.
    /// </summary>
    public class CredentialRevokedException : IndyException
    {
        const string message = "The credential has been revoked.";

        /// <summary>
        /// Initializes a new CredentialRevokedException.
        /// </summary>
        internal CredentialRevokedException() : base(message, (int)ErrorCode.AnoncredsCredentialRevoked)
        {

        }
    }

}
