namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when the pool ledger was terminated.
    /// </summary>
    public class PoolLedgerTerminatedException : IndyException
    {
        const string message = "The pool ledger was terminated.";

        /// <summary>
        /// Initializes a new PoolLedgerTerminatedException.
        /// </summary>
        internal PoolLedgerTerminatedException() : base(message, (int)ErrorCode.PoolLedgerTerminated)
        {

        }
    }

}
