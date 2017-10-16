using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class DisposeWalletTest : IndyIntegrationTestBase
    {
        [TestInitialize]
        public async Task CreateWallet()
        {
            await Wallet.CreateWalletAsync(POOL, WALLET, TYPE, null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await Wallet.DeleteWalletAsync(WALLET, null);
        }

        [TestMethod]
        public async Task CanDisposeClosedWallet()
        {
            using (var wallet = await Wallet.OpenWalletAsync(WALLET, null, null))
            {
                await wallet.CloseAsync();
            }
        }

        [TestMethod]
        public async Task DisposeCanBeCalledRepeatedly()
        {
            var wallet = await Wallet.OpenWalletAsync(WALLET, null, null);
            wallet.Dispose();
            wallet.Dispose();
        }

        [TestMethod]
        public async Task WalletCanBeReOpenedAfterDispose()
        {
            var wallet = await Wallet.OpenWalletAsync(WALLET, null, null);
            wallet.Dispose();

            using (var newWallet = await Wallet.OpenWalletAsync(WALLET, null, null))
            {
            }
        }

        [TestMethod]
        public async Task ClosingDisposedWalletStillProvidesSDKError()
        {
            var wallet = await Wallet.OpenWalletAsync(WALLET, null, null);
            wallet.Dispose();

            var ex = await Assert.ThrowsExceptionAsync<WalletClosedException>(() =>
                wallet.CloseAsync()
            );
        }      
    }
}
