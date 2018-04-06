using System;

namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Result of an issuer creating a claim.
    /// </summary>
    public sealed class IssuerCreateClaimResult
    {
        /// <summary>
        /// Initializes a new IssuerCreateClaimResult.
        /// </summary>
        /// <param name="revocRegUpdateJson">The revocation registry update JSON.</param>
        /// <param name="claimJson">The claim JSON.</param>
        public IssuerCreateClaimResult(string revocRegUpdateJson, string claimJson)
        {
            RevocRegUpdateJson = revocRegUpdateJson ?? throw new ArgumentNullException(nameof(revocRegUpdateJson));
            ClaimJson = claimJson ?? throw new ArgumentNullException(nameof(claimJson)); 
        }

        /// <summary>
        /// Gets the revocation registry update JSON.
        /// </summary>
        public string RevocRegUpdateJson { get; }

        /// <summary>
        /// Gets the claim JSON.
        /// </summary>
        public string ClaimJson { get; }
    }
}
