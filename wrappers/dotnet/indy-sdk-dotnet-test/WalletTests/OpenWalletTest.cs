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

            await Wallet.CreateWalletAsync("default", walletName, "default", null, null);
            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);

            Assert.IsNotNull(wallet);
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForConfig()
        {
            var walletName = "openWalletWorksForConfig";

            await Wallet.CreateWalletAsync("default", walletName, "default", null, null);
            var wallet = await Wallet.OpenWalletAsync(walletName, "{\"freshness_time\":1000}", null);

            Assert.IsNotNull(wallet);
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForNotCreatedWallet()
        {
            var ex = await Assert.ThrowsExceptionAsync<IOException>(() =>
                Wallet.OpenWalletAsync("openWalletWorksForNotCreatedWallet", null, null)
            );
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForTwice()
        {
            var walletName = "openWalletWorksForTwice";

            await Wallet.CreateWalletAsync("default", walletName, "default", null, null);

            var wallet1 = Wallet.OpenWalletAsync(walletName, null, null);

            var ex = await Assert.ThrowsExceptionAsync<WalletAlreadyOpenedException>(() =>
               Wallet.OpenWalletAsync(walletName, null, null)
            );
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForNotCreated()
        {
            var ex = await Assert.ThrowsExceptionAsync<IOException>(() =>
               Wallet.OpenWalletAsync("testOpenWalletWorksForNotCreated", null, null)
            );
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForPlugged()
        {
            var type = "inmem";
            var poolName = "default";
            var walletName = "testOpenWalletWorksForPlugged";

            await Wallet.CreateWalletAsync(poolName, walletName, type, null, null);
            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);
            Assert.IsNotNull(wallet);
        }
    }
}
