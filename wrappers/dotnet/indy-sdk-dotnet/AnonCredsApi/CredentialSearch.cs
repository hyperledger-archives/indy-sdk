using Hyperledger.Indy.Utils;
using Hyperledger.Indy.WalletApi;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Represents a credential search object
    /// </summary>
    public class CredentialSearch : IDisposable
    {
        /// <summary>
        /// Gets the handle.
        /// </summary>
        /// <value>The handle.</value>
        public int Handle { get; }

        /// <summary>
        /// Gets the total count of items.
        /// This field is <c>null</c> when search was created using <see cref="AnonCreds.ProverSearchCredentialsForProofRequestAsync(Wallet, string, string)"/>
        /// </summary>
        /// <value>The total count.</value>
        public int TotalCount { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="T:Hyperledger.Indy.AnonCredsApi.CredentialSearch"/> class.
        /// </summary>
        /// <param name="handle">Handle.</param>
        /// <param name="totalCount">Total count.</param>
        internal CredentialSearch(int handle, int totalCount)
        {
            TotalCount = totalCount;
            Handle = handle;
        }

        /// <summary>
        /// Fetch next credentials for search.
        /// </summary>
        /// <returns>The async.</returns>
        /// <param name="count">The item count to fetch.</param>
        public Task<string> NextAsync(int count)
        {
            return AnonCreds.ProverFetchCredentialsAsync(this, count);
        }

        #region IDisposable Support
        private bool _disposedValue; // To detect redundant calls

        /// <summary>
        /// Dispose the specified disposing.
        /// </summary>
        /// <param name="disposing">If set to <c>true</c> disposing.</param>
        protected virtual void Dispose(bool disposing)
        {
            if (!_disposedValue)
            {
                if (disposing)
                {
                }

                NativeMethods.indy_prover_close_credentials_search(
                    -1,
                    Handle,
                    CallbackHelper.NoValueCallback);

                _disposedValue = true;
            }
        }

        /// <summary>
        /// Releases unmanaged resources and performs other cleanup operations before the
        /// <see cref="T:Hyperledger.Indy.NonSecretsApi.WalletSearch"/> is reclaimed by garbage collection.
        /// </summary>
        ~CredentialSearch()
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
