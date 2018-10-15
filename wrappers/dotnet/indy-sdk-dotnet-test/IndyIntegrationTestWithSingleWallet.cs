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
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
        }
    }

}
