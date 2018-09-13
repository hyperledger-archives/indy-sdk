using Hyperledger.Indy.Test.Util;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class CloseWalletTest : IndyIntegrationTestBase
    {
        private const string CLOSE_WALLET_NAME = "CloseWallet";
        private Wallet _closeWallet = null;

        [TestInitialize]
        public async Task CreateWallet()
        {
            WalletConfig config = new WalletConfig() { id = CLOSE_WALLET_NAME };
            Credentials cred = new Credentials() { key = WALLET_KEY };

            await Wallet.CreateWalletAsync(config, cred);
            _closeWallet = await Wallet.OpenWalletAsync(config, cred);
        }

        [TestCleanup]
        public async Task CleanupWallet()
        {
            if (null == _closeWallet) return;

            try
            {
                _closeWallet.Dispose();

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
        public async Task TestCloseWalletWorks()
        {
            Assert.IsNotNull(_closeWallet);

            await _closeWallet.CloseAsync();
        }

        [TestMethod]
        public async Task TestCloseWalletWorksForTwice()
        {

            Assert.IsNotNull(_closeWallet);

            await _closeWallet.CloseAsync();

            var ex = await Assert.ThrowsExceptionAsync<InvalidWalletException>(() =>
                _closeWallet.CloseAsync()
            );
        }
    }
}
