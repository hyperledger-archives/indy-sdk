using Indy.Sdk.Dotnet.Wrapper;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Api
{
    /// <summary>
    /// High level API for interacting with ledgers.
    /// </summary>
    public class Ledger
    {
        //*******************
        // pool related
        //*******************
        
        /// <summary>
        /// Creates a new pool config for the ledger.
        /// </summary>
        /// <param name="configName">The name of the configuration to create.</param>
        /// <param name="config">The configuration.</param>
        /// <returns>An aysnchronous Task that returns no value.</returns>
        public static Task CreatePoolConfigAsync(string configName, string config)
        {
            return Wrapper.Pool.CreatePoolLedgerConfigAsync(configName, config);
        }

        /// <summary>
        /// Deletes an existing pool config for the ledger.
        /// </summary>
        /// <param name="configName">The name of th configuration.</param>
        /// <returns>An aysnchronous Task that returns no value.</returns>
        public static Task DeletePoolConfigAsync(string configName)
        {
            return Wrapper.Pool.DeletePoolLedgerConfigAsync(configName);
        }

        /// <summary>
        /// Opens a ledger instance connected to the specified pool configuration.
        /// </summary>
        /// <param name="configName">The name of the pool configuration to use.</param>
        /// <param name="config">Runtime configuration.</param>
        /// <returns>An aysnchronous Task that returns a Ledger instance.</returns>
        public static async Task<Ledger> OpenAsync(string configName, string config)
        {
            var poolWrapper = await Wrapper.Pool.OpenPoolLedgerAsync(configName, config);
            return new Ledger(poolWrapper);
        }

        /// <summary>
        /// Gets the pool wrapper instance used to communicate with the ledger pool.
        /// </summary>
        internal Wrapper.Pool PoolWrapper { get; }

        /// <summary>
        /// Initializes a new Ledger.
        /// </summary>
        /// <param name="poolWrapper">The pool instance to use when communicating with the ledger pool.</param>
        private Ledger(Wrapper.Pool poolWrapper)
        {
            PoolWrapper = poolWrapper;
        }

        /// <summary>
        /// Refreshes the ledger from the pool.
        /// </summary>
        /// <returns>An aysnchronous Task that returns no value.</returns>
        public Task RefreshAsync()
        {
            return PoolWrapper.RefreshAsync();
        }

        /// <summary>
        /// Closes the ledger.
        /// </summary>
        /// <returns>An aysnchronous Task that returns no value.</returns>
        public Task CloseAsync()
        {
            return PoolWrapper.CloseAsync();
        }

        //*******************
        // ledger related
        //*******************

        /// <summary>
        /// Signs and submits a message to the ledger.
        /// </summary>
        /// <param name="wallet">The wallet containing the submitter DID.</param>
        /// <param name="submitterDid">The submitter DID.</param>
        /// <param name="requstJson">The request JSON to sign and send.</param>
        /// <returns>An aysnchronous Task that returns the submit result.</returns>
        public Task<string> SignAndSubmitRequestAsync(Wallet wallet, string submitterDid, string requstJson)
        {
            return Wrapper.Ledger.SignAndSubmitRequestAsync(PoolWrapper, wallet.WalletWrapper, submitterDid, requstJson);
        }

        /// <summary>
        /// Signs and submits a message to the ledger.
        /// </summary>
        /// <param name="wallet">The wallet containing the submitter DID.</param>
        /// <param name="submitterDid">The submitter DID.</param>
        /// <param name="request">The request to sign and send.</param>
        /// <returns>An aysnchronous Task that returns the submit result.</returns>
        public Task<string> SignAndSubmitRequestAsync(Wallet wallet, string submitterDid, LedgerRequest request)
        {
            return Wrapper.Ledger.SignAndSubmitRequestAsync(PoolWrapper, wallet.WalletWrapper, submitterDid, request.Json);
        }


        /// <summary>
        /// Submits a pre-signed message to the ledger.
        /// </summary>
        /// <param name="request">The request to sign and send.</param>
        /// <returns>An aysnchronous Task that returns the submit result.</returns>
        public Task<string> SubmitRequestAsync(SignedLedgerRequest request)
        {
            return Wrapper.Ledger.SubmitRequestAsync(PoolWrapper, request.Json);
        }


        /// <summary>
        /// Submits a pre-signed message to the ledger.
        /// </summary>
        /// <param name="requestJson">The request to sign and send.</param>
        /// <returns>An aysnchronous Task that returns the submit result.</returns>
        public Task<string> SubmitRequestAsync(string requestJson)
        {
            return Wrapper.Ledger.SubmitRequestAsync(PoolWrapper, requestJson);
        }

        /// <summary>
        /// Builds a request to get a DDO.
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter of the request.</param>
        /// <param name="targetDid">The DID of the target to the the DDO for.</param>
        /// <returns>An aysnchronous Task that returns the DDO.</returns>
        public static Task<string> BuildGetDdoRequestAsync(string submitterDid, string targetDid)
        {
            return Wrapper.Ledger.BuildGetDdoRequestAsync(submitterDid, targetDid);
        }
        
        /// <summary>
        /// Builds a request to store a NYM
        /// </summary>
        /// <param name="submitterDid">The DID of the submitter.</param>
        /// <param name="targetDid">The DID the NYM belongs to.</param>
        /// <param name="verKey">The verification key.</param>
        /// <param name="alias">The alias.</param>
        /// <param name="role">The role.</param>
        /// <returns>An asynchonous Task that returns the request.</returns>
        public static Task<string> BuildNymRequestAsync(string submitterDid, string targetDid, string verKey, string alias, string role)
        {
            return Wrapper.Ledger.BuildNymRequestAsync(submitterDid, targetDid, verKey, alias, role);
        }

        //TODO: Add other ledger build commands.
    }
}
