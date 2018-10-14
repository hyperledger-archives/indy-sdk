namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when attempting create a credential definition that already exists.
    /// </summary>
    public class CredentialDefinitionAlreadyExistsException : IndyException
    {
        const string message = "The specified credential definition already exists.";

        /// <summary>
        /// Initializes a new CredDefAlreadyExistsException.
        /// </summary>
        internal CredentialDefinitionAlreadyExistsException() : base(message, (int)ErrorCode.AnoncredsCredDefAlreadyExistsError)
        {

        }
    }

}
