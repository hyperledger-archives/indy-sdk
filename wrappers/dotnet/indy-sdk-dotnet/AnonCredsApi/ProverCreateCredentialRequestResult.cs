namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Prover create credential request result.
    /// </summary>
    public class ProverCreateCredentialRequestResult
    {
        /// <summary>
        /// Gets the credential request json.
        /// </summary>
        /// <value>The credential request json.</value>
        public string CredentialRequestJson { get; }
        /// <summary>
        /// 
        /// </summary>
        /// <value>The credential request metadata json.</value>
        public string CredentialRequestMetadataJson { get; }

        /// <summary>
        /// Initializes a new instance of the
        /// <see cref="T:Hyperledger.Indy.AnonCredsApi.ProverCreateCredentialRequestResult"/> class.
        /// </summary>
        /// <param name="credentialRequestJson">Credential request json.</param>
        /// <param name="credentialRequestMetadataJson">Credential request metadata json.</param>
        public ProverCreateCredentialRequestResult(string credentialRequestJson, string credentialRequestMetadataJson)
        {
            CredentialRequestMetadataJson = credentialRequestMetadataJson;
            CredentialRequestJson = credentialRequestJson;
        }
    }
}