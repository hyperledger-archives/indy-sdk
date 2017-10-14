namespace Hyperledger.Indy
{
    /// <summary>
    /// Exception indicating that a value being processed was not considered a valid value.
    /// </summary>
    public class InvalidStructureException : IndyException
    {
        const string message = "A value being processed is not valid.";

        internal InvalidStructureException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
