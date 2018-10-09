using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class OpenWalletTest : IndyIntegrationTestBase
    {
        private const string OPEN_WALLET_NAME = "OpenWallet";
        private Wallet _openWallet = null;

        [TestCleanup]
        public async Task CleanupWallet()
        {
            if (null == _openWallet) return;

            try 
            {
                _openWallet.CloseAsync();
                _openWallet.Dispose();

            }
            catch
            {
                // the point of cleanup is to make sure everything is cleaned up
                // if it fails, it means no clean up was needed, most likely due
                // to test failing.  so its not imperative to do anything
                // with exceptions during clean up
            }
        }

        [TestMethod]
        public async Task TestOpenWalletWorks()
        {
            WalletConfig config = new WalletConfig() { id = OPEN_WALLET_NAME };
            Credentials cred = new Credentials() { key = WALLET_KEY };

            await Wallet.CreateWalletAsync(config, cred);
            _openWallet = await Wallet.OpenWalletAsync(config, cred);

            Assert.IsNotNull(_openWallet);
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForNotCreated()
        {
            WalletConfig config = new WalletConfig() { id = OPEN_WALLET_NAME };
            Credentials cred = new Credentials() { key = WALLET_KEY };

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
               Wallet.OpenWalletAsync(config, cred)
            );
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForTwice()
        {
            WalletConfig config = new WalletConfig() { id = OPEN_WALLET_NAME };
            Credentials cred = new Credentials() { key = WALLET_KEY };

            await Wallet.CreateWalletAsync(config, cred);
            _openWallet = await Wallet.OpenWalletAsync(config, cred);

            var ex = await Assert.ThrowsExceptionAsync<WalletAlreadyOpenedException>(() =>
               Wallet.OpenWalletAsync(config, cred)
            );
        }


    }
}
