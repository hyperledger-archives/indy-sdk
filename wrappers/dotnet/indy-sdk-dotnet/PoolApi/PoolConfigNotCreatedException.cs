namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when attempting to open pool which does not yet have a created configuration.
    /// </summary>
    public class PoolConfigNotCreatedException : IndyException
    {
        const string message = "The requested pool cannot be opened because it does not have an existing configuration.";

        internal PoolConfigNotCreatedException(int sdkErrorCode) : base(message, sdkErrorCode)
        {

        }
    }

}
