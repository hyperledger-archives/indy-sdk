namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when an anoncreds is not issued.
    /// </summary>
    public class AnoncredsNotIssuedException : IndyException
    {
        const string message = "The anoncreds is not issued.";

        internal AnoncredsNotIssuedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
