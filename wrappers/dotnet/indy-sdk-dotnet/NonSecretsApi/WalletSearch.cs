using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.NonSecretsApi
{
    /// <summary>
    /// Wallet search.
    /// </summary>
    public class WalletSearch : IDisposable
    {
        /// <summary>
        /// Gets the handle.
        /// </summary>
        /// <value>The handle.</value>
        public int Handle { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="T:Hyperledger.Indy.NonSecrets.WalletSearch"/> class.
        /// </summary>
        /// <param name="handle">Handle.</param>
        internal WalletSearch(int handle)
        {
            Handle = handle;
        }

        /// <summary>
        /// Fetch next records for wallet search.
        /// </summary>
        /// <returns>
        /// <code>
        /// {
        ///   totalCount: &lt;str>, // present only if retrieveTotalCount set to true
        ///   records: [{ // present only if retrieveRecords set to true
        ///       id: "Some id",
        ///       type: "Some type", // present only if retrieveType set to true
        ///       value: "Some value", // present only if retrieveValue set to true
        ///       tags: &lt;tags json>, // present only if retrieveTags set to true
        ///   }],
        /// }
        /// </code>
        /// </returns>
        /// <param name="wallet">Wallet.</param>
        /// <param name="count">Count of records to fetch.</param>
        public Task<string> NextAsync(Wallet wallet, int count)
        {
            return NonSecrets.FetchNextRecordsAsync(wallet, this, count);
        }

        #region IDisposable Support
        private bool disposedValue = false; // To detect redundant calls

        /// <summary>
        /// Dispose the specified disposing.
        /// </summary>
        /// <param name="disposing">If set to <c>true</c> disposing.</param>
        protected virtual void Dispose(bool disposing)
        {
            if (!disposedValue)
            {
                if (disposing)
                {
                }

                NativeMethods.indy_close_wallet_search(
                   -1,
                   Handle,
                   CallbackHelper.NoValueCallback
                );

                disposedValue = true;
            }
        }

        /// <summary>
        /// Releases unmanaged resources and performs other cleanup operations before the
        /// <see cref="T:Hyperledger.Indy.NonSecretsApi.WalletSearch"/> is reclaimed by garbage collection.
        /// </summary>
        ~WalletSearch()
        {
            Dispose(false);
        }

        /// <summary>
        /// Releases all resource used by the <see cref="T:Hyperledger.Indy.NonSecretsApi.WalletSearch"/> object.
        /// </summary>
        /// <remarks>Call <see cref="Dispose()"/> when you are finished using the
        /// <see cref="T:Hyperledger.Indy.NonSecretsApi.WalletSearch"/>. The <see cref="Dispose()"/> method leaves the
        /// <see cref="T:Hyperledger.Indy.NonSecretsApi.WalletSearch"/> in an unusable state. After calling
        /// <see cref="Dispose()"/>, you must release all references to the
        /// <see cref="T:Hyperledger.Indy.NonSecretsApi.WalletSearch"/> so the garbage collector can reclaim the memory
        /// that the <see cref="T:Hyperledger.Indy.NonSecretsApi.WalletSearch"/> was occupying.</remarks>
        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }
        #endregion
    }
}
