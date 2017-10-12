using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test
{

    public abstract class IndyIntegrationTestWithSingleWallet : IndyIntegrationTestBase
    {
        protected Wallet _wallet;

        [TestInitialize]
        public async Task CreateWallet()
        {
            await Wallet.CreateWalletAsync(POOL, WALLET, TYPE, null, null);
            _wallet = await Wallet.OpenWalletAsync(WALLET, null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await _wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(WALLET, null);
        }
    }

}
