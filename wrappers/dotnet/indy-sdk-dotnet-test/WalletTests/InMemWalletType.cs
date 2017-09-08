using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Threading;

namespace Hyperledger.Indy.Test.WalletTests
{
    public class InMemWalletType : WalletType
    {
        private IDictionary<int, InMemWallet> _openWallets = new ConcurrentDictionary<int, InMemWallet>();
        private IDictionary<string, InMemWallet> _configuredWallets = new ConcurrentDictionary<string, InMemWallet>();

        /// <summary>
        /// The next command handle to use.
        /// </summary>
        private static int _nextWalletHandle = 0;

        public IDictionary<string, InMemWallet> ConfiguredWallets
        {
            get { return ConfiguredWallets; }
        }

        /// <summary>
        /// Gets the next command handle.
        /// </summary>
        /// <returns>The next command handle.</returns>
        protected static int GetNextWalletHandle()
        {
            return Interlocked.Increment(ref _nextWalletHandle);
        }

        public override ErrorCode Create(string name, string config, string credentials)
        {
            if (_configuredWallets.ContainsKey(name))
                return ErrorCode.WalletAlreadyExistsError;

            var freshnessDuration = TimeSpan.FromSeconds(1000);

            if (!string.IsNullOrEmpty(config))
            {
                var configObj = JObject.Parse(config);
                var configuredFreshness = configObj.Value<double?>("freshness_time");

                if (configuredFreshness != null)
                    freshnessDuration = TimeSpan.FromSeconds(configuredFreshness.Value);
            }

            _configuredWallets.Add(name, new InMemWallet(freshnessDuration));
            return ErrorCode.Success;
        }

        public override ErrorCode Delete(string name, string config, string credentials)
        {
            if (!_configuredWallets.ContainsKey(name))
                return ErrorCode.CommonInvalidState;

            var wallet = _configuredWallets[name];

            if (wallet.IsOpen)
                return ErrorCode.CommonInvalidState;

            _configuredWallets.Remove(name);

            return ErrorCode.Success;
        }

        public override ErrorCode Open(string name, string config, string runtimeConfig, string credentials, out int walletHandle)
        {
            walletHandle = -1;

            if (!_configuredWallets.ContainsKey(name))
                return ErrorCode.CommonInvalidState;

            var wallet = _configuredWallets[name];

            if (wallet.IsOpen)
                return ErrorCode.WalletAlreadyOpenedError;

            wallet.IsOpen = true;

            walletHandle = GetNextWalletHandle();
            _openWallets.Add(walletHandle, wallet);

            return ErrorCode.Success;
        }

        public override ErrorCode Close(int walletHandle)
        {
            InMemWallet wallet;

            try
            {
                wallet = (InMemWallet)GetWalletByHandle(walletHandle);
            }
            catch(Exception)
            {
                return ErrorCode.WalletInvalidHandle;
            }
            
            wallet.IsOpen = false;
            _openWallets.Remove(walletHandle);

            return ErrorCode.Success;
        }

        protected override ICustomWallet GetWalletByHandle(int handle)
        {
            return _openWallets[handle];
        }

    }
}
