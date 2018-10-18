namespace Hyperledger.Indy.PoolApi
{
    /// <summary>
    /// Exception thrown when Pool Genesis Transactions are not compatible with Protocol version.
    /// </summary>
    public class PoolIncompatibleProtocolVersionException : IndyException
    {
        const string message = "The pool genesis transactions are not compatible with protocol version.";

        /// <summary>
        /// Initializes a new PoolIncompatibleProtocolVersionException.
        /// </summary>
        internal PoolIncompatibleProtocolVersionException() : base(message, (int)ErrorCode.PoolIncompatibleProtocolVersionError)
        {

        }
    }

}
