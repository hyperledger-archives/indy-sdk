using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    [TestClass]
    public class CloseWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public void TestCloseWalletWorks()
        {
            var walletName = "closeWalletWorks";
            Wallet.CreateWalletAsync("default", walletName, "default", null, null).Wait();

            var wallet = Wallet.OpenWalletAsync(walletName, null, null).Result;

            Assert.IsNotNull(wallet);

            wallet.CloseAsync().Wait();
        }

        [TestMethod]
        public async Task TestCloseWalletWorksForTwice()
        {
            var walletName = "closeWalletWorksForTwice";

            Wallet.CreateWalletAsync("default", walletName, "default", null, null).Wait();

            var wallet = Wallet.OpenWalletAsync(walletName, null, null).Result;

            Assert.IsNotNull(wallet);

            wallet.CloseAsync().Wait();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                wallet.CloseAsync()
            );

            Assert.AreEqual(ErrorCode.WalletInvalidHandle, ex.ErrorCode);
        }

        [TestMethod]
        public void TestCloseWalletWorksForPlugged()
        {
            var type = "inmem";
            var walletName = "testCloseWalletWorksForPlugged";

            Wallet.RegisterWalletTypeAsync(type, new InMemWalletType(), false).Wait();
            Wallet.CreateWalletAsync("default", walletName, type, null, null).Wait();

            var wallet = Wallet.OpenWalletAsync(walletName, null, null).Result;
            wallet.CloseAsync().Wait();
        }
    }
}
