using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Api
{
    /// <summary>
    /// A signed ledger request.
    /// </summary>
    public class SignedLedgerRequest
    {
        /// <summary>
        /// Gets the JSON of the request.
        /// </summary>
        public string Json { get; }

        /// <summary>
        /// Initializes a new SignedLedgerReqeust.
        /// </summary>
        /// <param name="unsignedJson">An unsigned ledger request Json .</param>
        internal SignedLedgerRequest(string unsignedJson)
        {
            Json = unsignedJson;
        }
    }
}
