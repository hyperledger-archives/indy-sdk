namespace Hyperledger.Indy.LedgerApi
{
    /// <summary>
    /// Parse response result.
    /// </summary>
    public class ParseResponseResult
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
        /// Initializes a new instance of the <see cref="T:Hyperledger.Indy.LedgerApi.ParseResponseResult"/> class.
        /// </summary>
        /// <param name="id">Identifier.</param>
        /// <param name="objectJson">Object json.</param>
        internal ParseResponseResult(string id, string objectJson)
		{
		    Id = id;
			ObjectJson = objectJson;
		}
	}
}