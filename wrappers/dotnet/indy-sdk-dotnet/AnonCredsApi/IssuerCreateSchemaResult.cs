namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Issuer create schema result.
    /// </summary>
    public class IssuerCreateSchemaResult
    {
        /// <summary>
        /// Gets the schema identifier.
        /// </summary>
        /// <value>The schema identifier.</value>
        public string SchemaId { get; }

        /// <summary>
        /// Gets the schema json.
        /// </summary>
        /// <value>The schema json.</value>
        public string SchemaJson { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="T:Hyperledger.Indy.AnonCredsApi.IssuerCreateSchemaResult"/> class.
        /// </summary>
        /// <param name="schemaId">Schema identifier.</param>
        /// <param name="schemaJson">Schema json.</param>
        public IssuerCreateSchemaResult(string schemaId, string schemaJson)
        {
            SchemaJson = schemaJson;
            SchemaId = schemaId;
        }
    }
}