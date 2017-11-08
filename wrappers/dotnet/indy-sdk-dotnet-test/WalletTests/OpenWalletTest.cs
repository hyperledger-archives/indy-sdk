using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class OpenWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestOpenWalletWorks()
        {
            var walletName = "openWalletWorks";

            await Wallet.CreateWalletAsync(POOL, walletName, TYPE, null, null);
            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);

            Assert.IsNotNull(wallet);
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForConfig()
        {
            var walletName = "openWalletWorksForConfig";

            await Wallet.CreateWalletAsync(POOL, walletName, TYPE, null, null);
            var wallet = await Wallet.OpenWalletAsync(walletName, "{\"freshness_time\":1000}", null);

            Assert.IsNotNull(wallet);
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForPlugged()
        {
            var walletName = "testOpenWalletWorksForPlugged";

            await Wallet.CreateWalletAsync(POOL, walletName, "inmem", null, null);
            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);
            Assert.IsNotNull(wallet);
        }
        
        [TestMethod]
        public async Task TestOpenWalletWorksForNotCreated()
        {
            var ex = await Assert.ThrowsExceptionAsync<IOException>(() =>
               Wallet.OpenWalletAsync("testOpenWalletWorksForNotCreated", null, null)
            );
        }
        
        [TestMethod]
        public async Task TestOpenWalletWorksForTwice()
        {
            var walletName = "openWalletWorksForTwice";

            await Wallet.CreateWalletAsync(POOL, walletName, TYPE, null, null);
            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);

            var ex = await Assert.ThrowsExceptionAsync<WalletAlreadyOpenedException>(() =>
               Wallet.OpenWalletAsync(walletName, null, null)
            );
        }

        
    }
}
