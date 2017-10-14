namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when a proof has been rejected.
    /// </summary>
    public class ProofRejectedException : IndyException
    {
        const string message = "The proof has been rejected.";

        internal ProofRejectedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
