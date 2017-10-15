namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when an attempt to create a master-secret with the same name as an existing master-secret.
    /// </summary>
    public class DuplicateMasterSecretNameException : IndyException
    {
        const string message = "Another master-secret with the specified name already exists.";

        /// <summary>
        /// Initializes a new DuplicateMasterSecretNameException.
        /// </summary>
        internal DuplicateMasterSecretNameException() : base(message, (int)ErrorCode.AnoncredsMasterSecretDuplicateNameError)
        {

        }
    }

}
