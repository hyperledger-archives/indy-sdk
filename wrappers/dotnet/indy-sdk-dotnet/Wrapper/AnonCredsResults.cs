namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Result of creating and storing a revocation registry.
    /// </summary>
    public sealed class IssuerCreateAndStoreRevocRegResult
    {
        /// <summary>
        /// Initializes a new IssuerCreateAndStoreRevocRegResult
        /// </summary>
        /// <param name="revocRegJson">The revocation registry JSON.</param>
        /// <param name="revocRegUuid">The revocation registry UUID.</param>
        public IssuerCreateAndStoreRevocRegResult(string revocRegJson, string revocRegUuid)
        {
            RevocRegJson = revocRegJson;
            RevocRegUuid = revocRegUuid;
        }

        /// <summary>
        /// Gets the revocation registry JSON.
        /// </summary>
        public string RevocRegJson { get; }

        /// <summary>
        /// Gets the revocation registry UUID.
        /// </summary>
        public string RevocRegUuid { get; }
    }

    /// <summary>
    /// Result of creating an issuer claim.
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
            RevocRegUpdateJson = revocRegUpdateJson;
            ClaimJson = claimJson;
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
