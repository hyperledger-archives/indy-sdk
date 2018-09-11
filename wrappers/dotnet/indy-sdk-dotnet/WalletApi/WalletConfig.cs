using System;
namespace Hyperledger.Indy.WalletApi
{
    /// <summary>
    /// used by Wallet.Create and Wallet.Open
    /// </summary>
    public sealed class WalletConfig
    {
        /// <summary>
        /// string, Identifier of the wallet.
        ///         Configured storage uses this identifier to lookup exact wallet data placement.
        /// </summary>
        public string id;
        /// <summary>
        /// optional&lt;string>, Type of the wallet storage.Defaults to 'default'.
        ///                  'Default' storage type allows to store wallet data in the local file.
        ///                  Custom storage types can be registered with indy_register_wallet_storage call.        /// </summary>
        public string storage_type = null;
        /// <summary>
        /// Storage configuration json. Storage type defines set of supported keys.
        ///                     Can be optional if storage supports default configuration.
        ///                     For 'default' storage type configuration is:
        /// </summary>
        public string storage_config = null;
        /// <summary>
        /// optional&lt;string>, Path to the directory with wallet files.
        ///             Defaults to $HOME/.indy_client/wallets.
        ///             Wallet will be stored in the file {path}/{id}/sqlite.db
        /// </summary>
        public string path = null;
    }
}
