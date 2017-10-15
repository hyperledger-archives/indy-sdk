namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when an anoncreds is not issued.
    /// </summary>
    public class AnoncredsNotIssuedException : IndyException
    {
        const string message = "The anoncreds is not issued.";

        /// <summary>
        /// Initializes a new AnoncredsNotIssuedException.
        /// </summary>
        internal AnoncredsNotIssuedException() : base(message, (int)ErrorCode.AnoncredsNotIssuedError)
        {

        }
    }

}
