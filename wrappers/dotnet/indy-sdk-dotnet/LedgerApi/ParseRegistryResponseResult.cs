namespace Hyperledger.Indy.LedgerApi
{
    /// <summary>
    /// Parse registry response result.
    /// </summary>
    public class ParseRegistryResponseResult
    {
        /// <summary>
        /// Gets the identifier.
        /// </summary>
        /// <value>The identifier.</value>
        public string Id { get; private set; }

        /// <summary>
        /// Gets the object json.
        /// </summary>
        /// <value>The object json.</value>
        public string ObjectJson { get; private set; }

        /// <summary>
        /// Gets the timestamp.
        /// </summary>
        /// <value>The timestamp.</value>
        public long Timestamp { get; private set; }

        /// <summary>
        /// Initializes a new instance of the <see cref="T:Hyperledger.Indy.LedgerApi.ParseRegistryResponseResult"/> class.
        /// </summary>
        /// <param name="id">Identifier.</param>
        /// <param name="objectJson">Object json.</param>
        /// <param name="timestamp">Timestamp.</param>
        internal ParseRegistryResponseResult(string id, string objectJson, long timestamp)
        {
            Id = id;
            ObjectJson = objectJson;
            Timestamp = timestamp;
        }
    }
}