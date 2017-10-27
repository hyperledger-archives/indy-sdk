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
            await Wallet.CreateWalletAsync(POOL, WALLET, TYPE, null, null);
            wallet = await Wallet.OpenWalletAsync(WALLET, null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(WALLET, null);
        }
    }

}
