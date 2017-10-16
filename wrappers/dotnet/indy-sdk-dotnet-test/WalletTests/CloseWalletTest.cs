using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class CloseWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestCloseWalletWorks()
        {
            await Wallet.CreateWalletAsync(POOL, WALLET, TYPE, null, null);
            var wallet = await Wallet.OpenWalletAsync(WALLET, null, null);

            Assert.IsNotNull(wallet);

            await wallet.CloseAsync();
        }

        [TestMethod]
        public async Task TestCloseWalletWorksForTwice()
        {
            await Wallet.CreateWalletAsync(POOL, WALLET, TYPE, null, null);
            var wallet = await Wallet.OpenWalletAsync(WALLET, null, null);

            Assert.IsNotNull(wallet);

            await wallet.CloseAsync();

            var ex = await Assert.ThrowsExceptionAsync<WalletClosedException>(() =>
                wallet.CloseAsync()
            );
        }

        [TestMethod]
        public async Task TestCloseWalletWorksForPlugged()
        {
            await Wallet.CreateWalletAsync(POOL, WALLET, "inmem", null, null);

            var wallet = await Wallet.OpenWalletAsync(WALLET, null, null);
            await wallet.CloseAsync();
        }
    }
}
