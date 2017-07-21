using Indy.Sdk.Dotnet.Wrapper;
using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    public class InMemWalletType : WalletType
    {
        private IDictionary<int, InMemWallet> _walletsByHandle = new ConcurrentDictionary<int, InMemWallet>();
        private IDictionary<string, InMemWallet> _walletsByName = new ConcurrentDictionary<string, InMemWallet>();

        /// <summary>
        /// The next command handle to use.
        /// </summary>
        private static int _nextWalletHandle = 0;

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
            if (_walletsByName.ContainsKey(name))
                return ErrorCode.WalletAlreadyExistsError;

            _walletsByName.Add(name, new InMemWallet());
            return ErrorCode.Success;
        }

        public override ErrorCode Delete(string name, string config, string credentials)
        {
            if (!_walletsByName.ContainsKey(name))
                return ErrorCode.CommonInvalidState;

            var wallet = _walletsByName[name];

            if (wallet.IsOpen)
                return ErrorCode.CommonInvalidState;

            _walletsByName.Remove(name);

            return ErrorCode.Success;
        }

        public override ErrorCode Open(string name, string config, string runtimeConfig, string credentials, out int walletHandle)
        {
            walletHandle = -1;

            if (!_walletsByName.ContainsKey(name))
                return ErrorCode.CommonInvalidState;

            var wallet = _walletsByName[name];

            if (wallet.IsOpen)
                return ErrorCode.WalletAlreadyOpenedError;

            wallet.IsOpen = true;

            walletHandle = GetNextWalletHandle();
            _walletsByHandle.Add(walletHandle, wallet);

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
            _walletsByHandle.Remove(walletHandle);

            return ErrorCode.Success;
        }

        protected override WalletBase GetWalletByHandle(int handle)
        {
            return _walletsByHandle[handle];
        }

    }
}
