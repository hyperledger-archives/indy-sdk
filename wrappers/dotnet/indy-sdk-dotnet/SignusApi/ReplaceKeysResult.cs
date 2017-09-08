namespace Hyperledger.Indy.SignusApi
{
    /// <summary>
    /// Result of replacing keys.
    /// </summary>
    public class ReplaceKeysResult
    {
        /// <summary>
        /// Initializes a new ReplaceKeysResult.
        /// </summary>
        /// <param name="verKey">The verification key.</param>
        /// <param name="pk">The primary key.</param>
        internal ReplaceKeysResult(string verKey, string pk)
        {
            VerKey = verKey;
            Pk = pk;
        }

        /// <summary>
        /// Gets the verification key.
        /// </summary>
        public string VerKey { get; }

        /// <summary>
        /// Gets the primary key.
        /// </summary>
        public string Pk { get; }

    }
}
