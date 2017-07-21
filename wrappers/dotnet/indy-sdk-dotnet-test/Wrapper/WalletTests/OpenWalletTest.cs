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
            Wallet.CreateWalletAsync("default", "openWalletWorks", "default", null, null).Wait();
            Wallet wallet = Wallet.OpenWalletAsync("openWalletWorks", null, null).Result;

            Assert.IsNotNull(wallet);
        }

        [TestMethod]
        public void TestOpenWalletWorksForConfig()
        {
            Wallet.CreateWalletAsync("default", "openWalletWorksForConfig", "default", null, null).Wait();
            Wallet wallet = Wallet.OpenWalletAsync("openWalletWorksForConfig", "{\"freshness_time\":1000}", null).Result;

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
            Wallet.CreateWalletAsync("default", "openWalletWorksForTwice", "default", null, null).Wait();

            var wallet1 = Wallet.OpenWalletAsync("openWalletWorksForTwice", null, null).Result;

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
               Wallet.OpenWalletAsync("openWalletWorksForTwice", null, null)
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
    }
}
