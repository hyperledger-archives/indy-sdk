using System;
using System.Collections.Generic;
using System.Text;

namespace Hyperledger.Indy.SignusApi
{
    /// <summary>
    /// Result of getting the endpoint for a DID.
    /// </summary>
    public class EndpointForDidResult
    {
        /// <summary>
        /// Initializes a new EndpointForDidResult.
        /// </summary>
        /// <param name="endpoint">The endpoint.</param>
        /// <param name="transportVk">The trasport verification key.</param>
        internal EndpointForDidResult(string endpoint, string transportVk)
        {
            Endpoint = endpoint;
            TransportVk = transportVk;
        }

        /// <summary>
        /// Gets the endpoint.
        /// </summary>
        public string Endpoint { get; private set; }

        /// <summary>
        /// Gets the transport verification key.
        /// </summary>
        public string TransportVk { get; private set; }
    }
}
