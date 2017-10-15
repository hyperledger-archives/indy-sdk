namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating that an IO error occurred.
    /// </summary>
    public class IOException : IndyException
    {
        const string message = "An IO error occurred.";

        /// <summary>
        /// Initializes a new IOException.
        /// </summary>
        internal IOException() : base(message, (int)ErrorCode.CommonIOError)
        {

        }
    }

}
