namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when a claim has been revoked.
    /// </summary>
    public class ClaimRevokedException : IndyException
    {
        const string message = "The claim has been revoked.";

        /// <summary>
        /// Initializes a new ClaimRevokedException.
        /// </summary>
        internal ClaimRevokedException() : base(message, (int)ErrorCode.AnoncredsCredentialRevoked)
        {

        }
    }

}
