namespace Hyperledger.Indy.LedgerApi
{
    /// <summary>
    /// Exception thrown when the no consensus was reached during a ledger operation.
    /// </summary>
    public class LedgerConsensusException : IndyException
    {
        const string message = "No consensus was reached during the ledger operation";

        internal LedgerConsensusException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
