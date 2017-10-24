namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Exception thrown when an anoncreds accumulator is full.
    /// </summary>
    public class AnoncredsAccumulatorFullException : IndyException
    {
        const string message = "The anoncreds accumulator is full.";

        /// <summary>
        /// Initializes a new AnoncredsAccumulatorFullException.
        /// </summary>
        internal AnoncredsAccumulatorFullException() : base(message, (int)ErrorCode.AnoncredsAccumulatorIsFull)
        {

        }
    }

}
