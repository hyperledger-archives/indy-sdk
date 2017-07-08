using Indy.Sdk.Dotnet.Wrapper;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Api
{
    /// <summary>
    /// High level API for interacting with wallets.
    /// </summary>
    public class Wallet
    {       
        /// <summary>
        /// Creates a new wallet.
        /// </summary>
        /// <param name="poolName">The name of the pool the wallet will be associated with.</param>
        /// <param name="name">The name of the wallet.</param>
        /// <param name="type">The wallet type.</param>
        /// <param name="config">The wallet configuration.</param>
        /// <param name="credentials">The credentials for the wallet.</param>
        /// <returns>An asynchronous Task that returns no value.</returns>
        public static Task CreateAsync(string poolName, string name, string type, string config, string credentials)
        {
            return Wrapper.Wallet.CreateWalletAsync(poolName, name, type, config, credentials);
        }

        /// <summary>
        /// Deletes a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to delete.</param>
        /// <param name="credentials">The credentials for the wallet.</param>
        /// <returns>An asynchronous Task that returns no value.</returns>
        public static Task DeleteAsync(string name, string credentials)
        {
            return Wrapper.Wallet.DeleteWalletAsync(name, credentials);
        }

        /// <summary>
        /// Opens a wallet.
        /// </summary>
        /// <param name="name">The name of the wallet to open.</param>
        /// <param name="runtimeConfig">The runtime configuration of for the wallet.</param>
        /// <param name="credentials">The credentials for the wallet.</param>
        /// <returns>An asynchronous Task that returns a Wallet instance.</returns>
        public static async Task<Wallet> OpenAsync(string name, string runtimeConfig, string credentials)
        {
            var walletHandle = await Wrapper.Wallet.OpenWalletAsync(name, runtimeConfig, credentials);
            return new Wallet(walletHandle);
        }

        /// <summary>
        /// Gets the low level API wrapper for the wallet.
        /// </summary>
        internal Wrapper.Wallet WalletWrapper { get; }

        /// <summary>
        /// Initializes a new Wallet instance.
        /// </summary>
        /// <param name="walletWrapper">The low level API wallet wrapper.</param>
        private Wallet(Wrapper.Wallet walletWrapper)
        {
            WalletWrapper = walletWrapper;
        }

        /// <summary>
        /// Closes the wallet.
        /// </summary>
        /// <returns></returns>
        public Task CloseAsync()
        {
            return WalletWrapper.CloseAsync();
        }

        /// <summary>
        /// Sets the sequence number for the specified value.
        /// </summary>
        /// <param name="walletKey">The wallet key to set the sequence number for.</param>
        /// <returns>An asynchronous Task that returns no value.</returns>
        public Task SetSeqNoForValueAsync(string walletKey)
        {
            return WalletWrapper.SetSeqNoForValueAsync(walletKey);
        }

        /// <summary>
        /// Creates and stores the local party's DID in this wallet.
        /// </summary>
        /// <param name="didJson">The DID Json to store.</param>
        /// <returns>An asynchronous Task that returns a CreateAndStoreMyDidResult instance.</returns>
        public Task<CreateAndStoreMyDidResult> CreateAndStoreMyDidAsync(string didJson)
        {
            return Signus.CreateAndStoreMyDidAsync(WalletWrapper, didJson);
        }

        /// <summary>
        /// Signs the provided message with the specified DID.
        /// </summary>
        /// <param name="did">The DID to sign the message with.</param>
        /// <param name="msg">The message to sign.</param>
        /// <returns>An asynchronous Task that returns the signed message.</returns>
        public Task<string> SignAsync(string did, string msg)
        {
            return Signus.SignAsync(WalletWrapper, did, msg);
        }

    }
}
