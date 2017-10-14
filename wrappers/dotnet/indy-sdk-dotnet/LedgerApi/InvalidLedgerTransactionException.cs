namespace Hyperledger.Indy.LedgerApi
{
    /// <summary>
    /// Exception thrown when attempting to send an unknown or incomplete ledger message.
    /// </summary>
    public class InvalidLedgerTransactionException : IndyException
    {
        const string message = "The ledger message is unknown or malformed.";

        internal InvalidLedgerTransactionException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
