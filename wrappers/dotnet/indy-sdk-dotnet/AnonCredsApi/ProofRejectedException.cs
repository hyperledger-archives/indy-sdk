namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when a proof has been rejected.
    /// </summary>
    public class ProofRejectedException : IndyException
    {
        const string message = "The proof has been rejected.";

        /// <summary>
        /// Initializes a new ProofRejectedException.
        /// </summary>
        internal ProofRejectedException() : base(message, (int)ErrorCode.AnoncredsProofRejected)
        {

        }
    }

}
