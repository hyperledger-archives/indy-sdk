using System;

namespace Hyperledger.Indy.DidApi
{
    /// <summary>
    /// Result of creating and storing my DID.
    /// </summary>
    public sealed class CreateAndStoreMyDidResult
    {
        /// <summary>
        /// Initializes a new CreateAndStoreMyDidResult.
        /// </summary>
        /// <param name="did">The DID created.</param>
        /// <param name="verKey">The verification key to use for verifying signatures.</param>
        internal CreateAndStoreMyDidResult(string did, string verKey)
        {
            Did = did ?? throw new ArgumentNullException("did"); 
            VerKey = verKey ?? throw new ArgumentNullException("verKey");
        }

        /// <summary>
        /// Gets the DID.
        /// </summary>
        public string Did { get; }

        /// <summary>
        /// Gets the verification key.
        /// </summary>
        public string VerKey { get; }
    }
}
