namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when attempting to open pool which does not yet have a created configuration.
    /// </summary>
    public class PoolConfigNotCreatedException : IndyException
    {
        const string message = "The requested pool cannot be opened because it does not have an existing configuration.";

        /// <summary>
        /// Initializes a new PoolConfigNotCreatedException.
        /// </summary>
        internal PoolConfigNotCreatedException() : base(message, (int)ErrorCode.PoolLedgerNotCreatedError)
        {

        }
    }

}
