namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when attempting to use a pool that has been closed.
    /// </summary>
    public class PoolClosedException : IndyException
    {
        const string message = "The pool is closed and cannot be used.";

        /// <summary>
        /// Initializes a new PoolClosedException.
        /// </summary>
        internal PoolClosedException() : base(message, (int)ErrorCode.PoolLedgerInvalidPoolHandle)
        {

        }
    }

}
