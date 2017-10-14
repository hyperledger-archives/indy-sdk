namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when attempting to use a full revocation registry.
    /// </summary>
    public class RevocationRegistryFullException : IndyException
    {
        const string message = "The specified revocation registry is full.  Another revocation registry must be created.";

        internal RevocationRegistryFullException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
