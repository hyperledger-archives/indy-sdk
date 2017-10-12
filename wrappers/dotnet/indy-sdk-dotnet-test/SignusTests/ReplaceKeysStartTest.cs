using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.Test.Util.Base58Check;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class ReplaceKeysStartTest : IndyIntegrationTestBase
    {
        private Wallet _wallet;
        private string _did;
        private string _verkey;
        private string _walletName = "signusWallet";

        [TestInitialize]
        public async Task CreateWalletWithDid()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);

            var result = await Signus.CreateAndStoreMyDidAsync(this._wallet, "{}");

            _did = result.Did;
            _verkey = result.VerKey;
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await _wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(_walletName, null);
        }

        [TestMethod]
        public async Task TestReplaceKeysStartWorksForEmptyJson()
        {
            var result = await Signus.ReplaceKeysStartAsync(_wallet, _did, "{}");

            Assert.IsNotNull(result);
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(result.VerKey).Length);
        }

        [TestMethod]
        public async Task TestReplaceKeysStartWorksForNotExistsDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Signus.ReplaceKeysStartAsync(this._wallet, "unknowndid", "{}")
            );
        }

        [TestMethod]
        public async Task TestReplaceKeysStartWorksForSeed()
        {
            var result = await Signus.ReplaceKeysStartAsync(this._wallet, this._did, "{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}");
            string verkey = result.VerKey;

            Assert.AreEqual("CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", verkey);
            Assert.AreNotEqual(this._verkey, verkey);
        }
    }
}
