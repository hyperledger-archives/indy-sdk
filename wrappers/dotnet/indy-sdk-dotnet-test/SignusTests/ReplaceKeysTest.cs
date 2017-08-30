using Base58Check;
using Hyperledger.Indy.Sdk.SignUsApi;
using Hyperledger.Indy.Sdk.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Sdk.Test.SignUsTests
{
    [TestClass]
    public class ReplaceKeysTest : IndyIntegrationTestBase
    {
        private Wallet _wallet;
        private string _walletName = "signusWallet";
        private string _did;
        private string _verKey;

        [TestInitialize]
        public async Task CreateWalletWithDid()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);

            var result = await SignUs.CreateAndStoreMyDidAsync(_wallet, "{}");

            _did = result.Did;
            _verKey = result.VerKey;
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if(_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);
        }
        
        [TestMethod]
        public async Task TestReplaceKeysWorksForEmptyJson()
        {
            var result = await SignUs.ReplaceKeysAsync(_wallet, _did, "{}");

            Assert.IsNotNull(result);
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(result.VerKey).Length);
        }

        [TestMethod]
        public async Task TestReplaceKeysWorksForInvalidDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                SignUs.ReplaceKeysAsync(_wallet, "invalid_base58_string", "{}")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestReplaceKeysWorksForNotExistsDid()
        {
            var result = await SignUs.ReplaceKeysAsync(_wallet, "8wZcEriaNLNKtteJvx7f8i", "{}");

            Assert.IsNotNull(result);
        }

        [TestMethod]
        public async Task TestReplaceKeysWorksForSeed()
        {
            var result = await SignUs.ReplaceKeysAsync(_wallet, _did, "{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}");

            Assert.IsNotNull(result);
            Assert.AreEqual("CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", result.VerKey);
            Assert.AreNotEqual(_verKey, result.VerKey);
        }

    }
}
