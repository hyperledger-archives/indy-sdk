namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Issuer create and store revoc reg result.
    /// </summary>
    public class IssuerCreateAndStoreRevocRegResult
    {
        /// <summary>
        /// Gets the rev reg identifier.
        /// </summary>
        /// <value>The rev reg identifier.</value>
        public string RevRegId { get; }

        /// <summary>
        /// Gets the rev reg def json.
        /// </summary>
        /// <value>The rev reg def json.</value>
        public string RevRegDefJson { get; }

        /// <summary>
        /// Gets the rev reg entry json.
        /// </summary>
        /// <value>The rev reg entry json.</value>
        public string RevRegEntryJson { get; }

        /// <summary>
        /// Initializes a new instance of the
        /// <see cref="T:Hyperledger.Indy.AnonCredsApi.IssuerCreateAndStoreRevocRegResult"/> class.
        /// </summary>
        /// <param name="revRegId">Rev reg identifier.</param>
        /// <param name="revRegDefJson">Rev reg def json.</param>
        /// <param name="revRegEntryJson">Rev reg entry json.</param>
        public IssuerCreateAndStoreRevocRegResult(string revRegId, string revRegDefJson, string revRegEntryJson)
        {
            RevRegEntryJson = revRegEntryJson;
            RevRegDefJson = revRegDefJson;
            RevRegId = revRegId;
        }
    }
}