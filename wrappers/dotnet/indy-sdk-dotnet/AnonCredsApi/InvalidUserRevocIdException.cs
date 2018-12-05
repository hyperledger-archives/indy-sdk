namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when an invalid user revocation registry id is used.
    /// </summary>
    public class InvalidUserRevocIdException : IndyException
    {
        const string message = "The user revocation registry id specified is invalid.";

        /// <summary>
        /// Initializes a new InvalidUserRevocIdException.
        /// </summary>
        internal InvalidUserRevocIdException() : base(message, (int)ErrorCode.AnoncredsInvalidUserRevocId)
        {

        }
    }

}
