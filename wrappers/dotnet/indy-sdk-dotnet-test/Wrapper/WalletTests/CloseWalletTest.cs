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
            Wallet.CreateWalletAsync("default", "CloseAsyncWorks", "default", null, null).Wait();

            var wallet = Wallet.OpenWalletAsync("CloseAsyncWorks", null, null).Result;

            Assert.IsNotNull(wallet);

            wallet.CloseAsync().Wait();
        }

        [TestMethod]
        public async Task TestCloseWalletWorksForTwice()
        {
            Wallet.CreateWalletAsync("default", "CloseAsyncWorksForTwice", "default", null, null).Wait();

            var wallet = Wallet.OpenWalletAsync("CloseAsyncWorksForTwice", null, null).Result;

            Assert.IsNotNull(wallet);

            wallet.CloseAsync().Wait();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                wallet.CloseAsync()
            );

            Assert.AreEqual(ErrorCode.WalletInvalidHandle, ex.ErrorCode);
        }
    }
}
