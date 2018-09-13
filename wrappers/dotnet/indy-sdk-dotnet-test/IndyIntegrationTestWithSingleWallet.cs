using Hyperledger.Indy.Test.Util;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test
{
    public abstract class IndyIntegrationTestWithSingleWallet : IndyIntegrationTestBase
    {
        protected Wallet wallet;


        [TestInitialize]
        public async Task CreateWallet()
        {
            await WalletUtils.CreateWallet(WALLET, WALLET_KEY);
            wallet = await WalletUtils.OpenWallet(WALLET, WALLET_KEY);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await wallet.CloseAsync();
            await WalletUtils.DeleteWallet(WALLET, WALLET_KEY);
        }
    }

}
