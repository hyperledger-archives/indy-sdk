using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    [TestClass]
    public class OpenWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public void TestOpenWalletWorks()
        {
            var walletName = "openWalletWorks";

            Wallet.CreateWalletAsync("default", walletName, "default", null, null).Wait();
            Wallet wallet = Wallet.OpenWalletAsync(walletName, null, null).Result;

            Assert.IsNotNull(wallet);
        }

        [TestMethod]
        public void TestOpenWalletWorksForConfig()
        {
            var walletName = "openWalletWorksForConfig";

            Wallet.CreateWalletAsync("default", walletName, "default", null, null).Wait();
            Wallet wallet = Wallet.OpenWalletAsync(walletName, "{\"freshness_time\":1000}", null).Result;

            Assert.IsNotNull(wallet);
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForNotCreatedWallet()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Wallet.OpenWalletAsync("openWalletWorksForNotCreatedWallet", null, null)
            );

            Assert.AreEqual(ErrorCode.CommonIOError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForTwice()
        {
            var walletName = "openWalletWorksForTwice";

            Wallet.CreateWalletAsync("default", walletName, "default", null, null).Wait();

            var wallet1 = Wallet.OpenWalletAsync(walletName, null, null).Result;

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
               Wallet.OpenWalletAsync(walletName, null, null)
            );

            Assert.AreEqual(ErrorCode.WalletAlreadyOpenedError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForNotCreated()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
               Wallet.OpenWalletAsync("testOpenWalletWorksForNotCreated", null, null)
            );

            Assert.AreEqual(ErrorCode.CommonIOError, ex.ErrorCode);
        }

        [TestMethod]
        public void TestOpenWalletWorksForPlugged()
        {
            var type = "inmem";
            var poolName = "default";
            var walletName = "testOpenWalletWorksForPlugged";

            Wallet.CreateWalletAsync(poolName, walletName, type, null, null).Wait();
            var wallet = Wallet.OpenWalletAsync(walletName, null, null).Result;
            Assert.IsNotNull(wallet);
        }
    }
}
