using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.Test.Util.Base58Check;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class ReplaceKeysStartTest : IndyIntegrationTestWithSingleWallet
    {
        private string _did;
        private string _verkey;

        [TestInitialize]
        public async Task CreateWalletWithDid()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            _did = result.Did;
            _verkey = result.VerKey;
        }

        [TestMethod]
        public async Task TestReplaceKeysStartWorksForEmptyJson()
        {
            var result = await Signus.ReplaceKeysStartAsync(wallet, _did, "{}");

            Assert.IsNotNull(result);
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(result.VerKey).Length);
        }

        [TestMethod]
        public async Task TestReplaceKeysStartWorksForNotExistsDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Signus.ReplaceKeysStartAsync(this.wallet, DID1, "{}")
            );
        }

        [TestMethod]
        public async Task TestReplaceKeysStartWorksForSeed()
        {
            var result = await Signus.ReplaceKeysStartAsync(wallet, _did, "{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}");
            string verkey = result.VerKey;

            Assert.AreEqual("CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", verkey);
            Assert.AreNotEqual(this._verkey, verkey);
        }
    }
}
