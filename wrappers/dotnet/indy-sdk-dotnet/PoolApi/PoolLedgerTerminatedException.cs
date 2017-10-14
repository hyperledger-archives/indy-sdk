namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when the pool ledger was terminated.
    /// </summary>
    public class PoolLedgerTerminatedException : IndyException
    {
        const string message = "The pool ledger was terminated.";

        internal PoolLedgerTerminatedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
