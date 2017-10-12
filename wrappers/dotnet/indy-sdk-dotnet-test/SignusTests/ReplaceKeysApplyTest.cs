using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class ReplaceKeysApplyTest : IndyIntegrationTestBase
    {
        private Wallet _wallet;
        private string _did;
        private string _walletName = "signusWallet";

        [TestInitialize]
        public async Task CreateWalletWithDid()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);

            var result = await Signus.CreateAndStoreMyDidAsync(this._wallet, "{}");

            _did = result.Did;
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await _wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(_walletName, null);
        }

        [TestMethod]
        public async Task TestReplaceKeysApplyWorks()
        {
            await Signus.ReplaceKeysStartAsync(_wallet, _did, "{}");
            await Signus.ReplaceKeysApplyAsync(_wallet, _did);
        }

        [TestMethod]
        public async Task TestReplaceKeysApplyWorksWithoutCallingReplaceStart()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.ReplaceKeysApplyAsync(_wallet, _did)
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestReplaceKeysApplyWorksForNotFoundDid()
        {
            await Signus.ReplaceKeysStartAsync(_wallet, _did, "{}");

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.ReplaceKeysApplyAsync(_wallet, "unknowndid")
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }
    }
}
