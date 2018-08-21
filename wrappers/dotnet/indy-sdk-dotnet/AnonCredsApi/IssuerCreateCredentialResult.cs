namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Result from calling IssuerCreateCredentialAsync.
    /// </summary>
    public sealed class IssuerCreateCredentialResult
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="T:Hyperledger.Indy.AnonCredsApi.IssuerCreateCredentialResult"/> class.
        /// </summary>
        /// <param name="credentialJson">Credential json.</param>
        /// <param name="revocId">Revoc identifier.</param>
        /// <param name="revocRegDeltaJson">Revoc reg delta json.</param>
        public IssuerCreateCredentialResult(string credentialJson, string revocId, string revocRegDeltaJson)
        {
            RevocId = revocId;
            RevocRegDeltaJson = revocRegDeltaJson;
            CredentialJson = credentialJson; 

        }

        /// <summary>
        /// Gets the revocation registry update JSON.
        /// </summary>
        public string RevocRegDeltaJson { get; }

        /// <summary>
        /// Gets the credential JSON.
        /// </summary>
        public string CredentialJson { get; }

        /// <summary>
        /// Gets the revocation registration delta JSON.
        /// </summary>
        /// <value>The revoc identifier.</value>
        public string RevocId { get; }
    }
}
