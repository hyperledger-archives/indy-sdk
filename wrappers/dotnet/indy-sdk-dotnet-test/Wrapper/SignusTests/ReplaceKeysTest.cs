using Base58Check;
using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.SignusTests
{
    [TestClass]
    public class ReplaceKeysTest : IndyIntegrationTestBase
    {
        private Wallet _wallet;

        private string _did;
        private string _verKey;

        [TestInitialize]
        public void CreateWalletWithDid()
        {
            Wallet.CreateWalletAsync("default", "signusWallet", "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync("signusWallet", null, null).Result;

            var result = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;

            _did = result.Did;
            _verKey = result.VerKey;
        }

        [TestCleanup]
        public void DeleteWallet()
        {
            _wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("signusWallet", null).Wait();
        }
        
        [TestMethod]
        public void TestReplaceKeysWorksForEmptyJson()
        {
            var result = Signus.ReplaceKeysAsync(_wallet, _did, "{}").Result;

            Assert.IsNotNull(result);
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(result.VerKey).Length);
        }

        [TestMethod]
        public async Task TestReplaceKeysWorksForInvalidDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.ReplaceKeysAsync(_wallet, "invalid_base58_string", "{}")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public void TestReplaceKeysWorksForNotExistsDid()
        {
            var result = Signus.ReplaceKeysAsync(_wallet, "8wZcEriaNLNKtteJvx7f8i", "{}").Result;

            Assert.IsNotNull(result);
        }

        [TestMethod]
        public void TestReplaceKeysWorksForSeed()
        {
            var result = Signus.ReplaceKeysAsync(_wallet, _did, "{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}").Result;

            Assert.IsNotNull(result);
            Assert.AreEqual("CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", result.VerKey);
            Assert.AreNotEqual(_verKey, result.VerKey);
        }

    }
}
