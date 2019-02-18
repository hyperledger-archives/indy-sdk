namespace Hyperledger.Indy.NonSecretsApi
{
    /// <summary>
    /// No records exception.
    /// </summary>
    public class NoRecordsException : IndyException
    {
        const string message = "No records found.";

        internal NoRecordsException(string message, int sdkErrorCode) : base(message, sdkErrorCode)
        {
        }
    }
}
