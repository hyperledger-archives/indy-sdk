using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Api
{
    /// <summary>
    /// A request that can be submitted to a ledger.
    /// </summary>
    public class LedgerRequest
    {
        /// <summary>
        /// Initializes a new ledger request.
        /// </summary>
        /// <param name="json"></param>
        public LedgerRequest(string json)
        {
            Json = json;
        }

        /// <summary>
        /// The JSON content of the request.
        /// </summary>
        public string Json { get; }
    }
}
