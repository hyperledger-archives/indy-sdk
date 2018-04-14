namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Issuer create and store credential def result.
    /// </summary>
    public class IssuerCreateAndStoreCredentialDefResult
    {
        /// <summary>
        /// Gets the cred def identifier.
        /// </summary>
        /// <value>The cred def identifier.</value>
        public string CredDefId { get; }

        /// <summary>
        /// Gets the cred def json.
        /// </summary>
        /// <value>The cred def json.</value>
        public string CredDefJson { get; }

        /// <summary>
        /// Initializes a new instance of the
        /// <see cref="T:Hyperledger.Indy.AnonCredsApi.IssuerCreateAndStoreCredentialDefResult"/> class.
        /// </summary>
        /// <param name="credDefId">Cred def identifier.</param>
        /// <param name="credDefJson">Cred def json.</param>
        public IssuerCreateAndStoreCredentialDefResult(string credDefId, string credDefJson)
        {
            CredDefJson = credDefJson;
            CredDefId = credDefId;
        }
    }
}