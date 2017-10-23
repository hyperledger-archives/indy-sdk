namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when attempting to create a pool ledger config with same name as an existing pool ledger config.
    /// </summary>
    public class PoolLedgerConfigExistsException : IndyException
    {
        const string message = "A pool ledger configuration already exists with the specified name.";

        /// <summary>
        /// Initializes a new PoolLedgerConfigExistsException.
        /// </summary>
        internal PoolLedgerConfigExistsException() : base(message, (int)ErrorCode.PoolLedgerConfigAlreadyExistsError)
        {

        }
    }

}
