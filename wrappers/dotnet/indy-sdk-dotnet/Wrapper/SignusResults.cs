namespace Indy.Sdk.Dotnet.Wrapper
{
    /// <summary>
    /// Result of creating and storing my DID.
    /// </summary>
    public class CreateAndStoreMyDidResult
    {
        /// <summary>
        /// Initalizes a new CreateAndStoreMyDidResult.
        /// </summary>
        /// <param name="did">The DID created.</param>
        /// <param name="verKey">The verification key for the signature.</param>
        /// <param name="pk">The primary key for decrpytion</param>
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

    /// <summary>
    /// The result of encryption.
    /// </summary>
    public class EncryptResult
    {
        /// <summary>
        /// Initializes a new EncryptionResult.
        /// </summary>
        /// <param name="encryptedMsg">The encrypted message.</param>
        /// <param name="nonce">The nonce.</param>
        internal EncryptResult(string encryptedMsg, string nonce)
        {
            EncryptedMsg = encryptedMsg;
            Nonce = nonce;
        }

        /// <summary>
        /// Gets the encrypted message.
        /// </summary>
        public string EncryptedMsg { get; }

        /// <summary>
        /// Gets the nonce.
        /// </summary>
        public string Nonce { get; }

    }
}
