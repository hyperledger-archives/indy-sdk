namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when an anoncreds accululator is full.
    /// </summary>
    public class AnoncredsAccumulatorFullException : IndyException
    {
        const string message = "The anoncreds accumulator is full.";

        internal AnoncredsAccumulatorFullException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
