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
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            var wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            await wallet.CloseAsync();
        }

        [TestMethod]
        public async Task TestCloseWalletWorksForTwice()
        {
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            var wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            await wallet.CloseAsync();

            var ex = await Assert.ThrowsExceptionAsync<InvalidWalletException>(() =>
                wallet.CloseAsync()
            );
        }

        [TestMethod]
        public async Task TestAutoCloseWorks()
        {
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            using (var a_wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS)) {
                Assert.IsNotNull(a_wallet);
            }

            var wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            await wallet.CloseAsync();
        }
    }
}
