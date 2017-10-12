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
            var walletName = "closeWalletWorks";
            await Wallet.CreateWalletAsync("default", walletName, "default", null, null);

            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);

            Assert.IsNotNull(wallet);

            await wallet.CloseAsync();
        }

        [TestMethod]
        public async Task TestCloseWalletWorksForTwice()
        {
            var walletName = "closeWalletWorksForTwice";

            await Wallet.CreateWalletAsync("default", walletName, "default", null, null);

            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);

            Assert.IsNotNull(wallet);

            await wallet.CloseAsync();

            var ex = await Assert.ThrowsExceptionAsync<WalletClosedException>(() =>
                wallet.CloseAsync()
            );
        }

        [TestMethod]
        public async Task TestCloseWalletWorksForPlugged()
        {
            var type = "inmem";
            var walletName = "testCloseWalletWorksForPlugged";

            await Wallet.CreateWalletAsync("default", walletName, type, null, null);

            var wallet = await Wallet.OpenWalletAsync(walletName, null, null);
            await wallet.CloseAsync();
        }
    }
}
