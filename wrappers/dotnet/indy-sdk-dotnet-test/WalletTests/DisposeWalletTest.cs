using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class DisposeWalletTest : IndyIntegrationTestBase
    {
        private const string DISPOSE_WALLET_NAME = "DisposeWallet";
        private Wallet _disposeWallet = null;
        private WalletConfig _config = new WalletConfig() { id = DISPOSE_WALLET_NAME };
        private Credentials _cred = new Credentials() { key = WALLET_KEY };

        [TestInitialize]
        public async Task CreateWallet()
        {       
            await Wallet.CreateWalletAsync(_config, _cred);
        }

        [TestCleanup]
        public async Task CleanupWallet()
        {
            if (null == _disposeWallet) return;

            try
            {
                _disposeWallet.Dispose();

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
        public async Task CanDisposeClosedWallet()
        {
            using (var _disposeWallet = await Wallet.OpenWalletAsync(_config, _cred))
            {
                await _disposeWallet.CloseAsync();
            }
        }

        [TestMethod]
        public async Task DisposeCanBeCalledRepeatedly()
        {
            _disposeWallet = await Wallet.OpenWalletAsync(_config, _cred);
            _disposeWallet.Dispose();
            _disposeWallet.Dispose();
        }

        [TestMethod]
        public async Task WalletCanBeReOpenedAfterDispose()
        {
            _disposeWallet = await Wallet.OpenWalletAsync(_config, _cred);
            _disposeWallet.Dispose();

            using (var newWallet = await Wallet.OpenWalletAsync(_config, _cred))
            {
            }
        }

        [TestMethod]
        public async Task ClosingDisposedWalletStillProvidesSDKError()
        {
            _disposeWallet = await Wallet.OpenWalletAsync(_config, _cred);
            _disposeWallet.Dispose();

            var ex = await Assert.ThrowsExceptionAsync<InvalidWalletException>(() =>
                _disposeWallet.CloseAsync()
            );
        }      
    }
}
