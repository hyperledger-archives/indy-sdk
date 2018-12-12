namespace Hyperledger.Indy.DidApi
{
    /// <summary>
    /// Result of getting the endpoint for a DID.
    /// </summary>
    public class EndpointForDidResult
    {
        /// <summary>
        /// Initializes a new EndpointForDidResult.
        /// </summary>
        /// <param name="address">The address.</param>
        /// <param name="transportKey">The transport verification key.</param>
        internal EndpointForDidResult(string address, string transportKey)
        {
            Address = address;
            TransportKey = transportKey;
        }

        /// <summary>
        /// Gets the address.
        /// </summary>
        public string Address { get; private set; }

        /// <summary>
        /// Gets the transport verification key.
        /// </summary>
        public string TransportKey { get; private set; }
    }
}
