namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when attempting to use a pool that has been closed or is invalid.
    /// </summary>
    public class InvalidPoolException : IndyException
    {
        const string message = "The pool is closed or invalid and cannot be used.";

        /// <summary>
        /// Initializes a new PoolClosedException.
        /// </summary>
        internal InvalidPoolException() : base(message, (int)ErrorCode.PoolLedgerInvalidPoolHandle)
        {

        }
    }

}
