using System;
using System.Threading.Tasks;
using Hyperledger.Indy.Utils;

namespace Hyperledger.Indy.AnonCredsApi
{
    /// <summary>
    /// Represents a credential search for proof request object
    /// </summary>
    /// <seealso cref="System.IDisposable" />
    public class CredentialSearchForProofRequest : IDisposable
    {
        /// <summary>
        /// Gets the handle.
        /// </summary>
        /// <value>The handle.</value>
        public int Handle { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="CredentialSearchForProofRequest"/> class.
        /// </summary>
        /// <param name="handle">The handle.</param>
        internal CredentialSearchForProofRequest(int handle)
        {
            Handle = handle;
        }

        /// <summary>
        /// Fetch next credentials for search.
        /// </summary>
        /// <param name="itemReferent">The item referent.</param>
        /// <param name="count">The item count to fetch.</param>
        /// <returns>
        /// The credential search json data
        /// </returns>
        public Task<string> NextAsync(string itemReferent, int count)
        {
            return AnonCreds.ProverFetchCredentialsForProofRequestAsync(this, itemReferent, count);
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

                NativeMethods.indy_prover_close_credentials_search_for_proof_req(
                    -1,
                    Handle,
                    CallbackHelper.NoValueCallback);

                _disposedValue = true;
            }
        }

        /// <inheritdoc />
        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        /// <summary>
        /// Finalizes an instance of the <see cref="CredentialSearchForProofRequest"/> class.
        /// </summary>
        ~CredentialSearchForProofRequest()
        {
            Dispose(false);
        }
        #endregion
    }
}