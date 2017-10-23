namespace Hyperledger.Indy.LedgerApi
{
    /// <summary>
    /// Exception thrown when the no consensus was reached during a ledger operation.
    /// </summary>
    public class LedgerConsensusException : IndyException
    {
        const string message = "No consensus was reached during the ledger operation";

        /// <summary>
        /// Initializes a new LedgerConsensusException.
        /// </summary>
        internal LedgerConsensusException() : base(message, (int)ErrorCode.LedgerNoConsensusError)
        {

        }
    }

}
