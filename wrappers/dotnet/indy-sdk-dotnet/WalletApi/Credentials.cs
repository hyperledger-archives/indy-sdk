using System;
namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// Used by Wallet.Create/Wallet.Open
    /// </summary>
    public sealed class Credentials
    {
        /// <summary>
        /// string, Passphrase used to derive current wallet master key
        /// </summary>
        public string key = null;
        /// <summary>
        /// Optional&lt;string>, If present than wallet master key will be rotated to a new one
        ///                                  derived from this passphrase.
        /// </summary>
        public string rekey = null;
        /// <summary>
        /// optional&lt;object> Credentials for wallet storage. Storage type defines set of supported keys.
        ///                              Can be optional if storage supports default configuration.
        ///                              For 'default' storage type should be empty.
        /// </summary>
        public string storage_credentials = null;
    }
}
