namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating that an IO error occurred.
    /// </summary>
    public class IOException : IndyException
    {
        const string message = "An IO error occurred.";

        internal IOException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
