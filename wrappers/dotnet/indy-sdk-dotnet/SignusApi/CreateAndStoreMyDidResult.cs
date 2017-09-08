namespace Hyperledger.Indy.SignusApi
{
    /// <summary>
    /// Result of creating and storing my DID.
    /// </summary>
    public class CreateAndStoreMyDidResult
    {
        /// <summary>
        /// Initializes a new CreateAndStoreMyDidResult.
        /// </summary>
        /// <param name="did">The DID created.</param>
        /// <param name="verKey">The verification key to use for verifying signatures.</param>
        /// <param name="pk">The primary key to use for encryption</param>
        internal CreateAndStoreMyDidResult(string did, string verKey, string pk)
        {
            Did = did;
            VerKey = verKey;
            Pk = pk;
        }

        /// <summary>
        /// Gets the DID.
        /// </summary>
        public string Did { get; }

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
