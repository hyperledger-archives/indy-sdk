using Hyperledger.Indy.Test.Util.Base58Check;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class CreateMyDidTest : IndyIntegrationTestBase
    {
        private Wallet _wallet;
        private string _walletName = "SignusWallet";
        private string _seed = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        private string _did = "8wZcEriaNLNKtteJvx7f8i";
        private string _expectedVerkey = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
        private string _existsCryptoType = "ed25519";

        [TestInitialize]
        public async Task CreateWallet()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if(_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);
        }
        
        [TestMethod]
        public async Task TestCreateMyDidWorksForEmptyJson()
        {
            var json = "{}";

            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, json);
            Assert.IsNotNull(result);

            Assert.AreEqual(16, Base58CheckEncoding.DecodePlain(result.Did).Length);
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(result.VerKey).Length);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForSeed()
        {
            var json = string.Format("{{\"seed\":\"{0}\"}}", _seed);

            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, json);
            Assert.IsNotNull(result);

            var expectedDid = "NcYxiDXkpYi6ov5FcYDi1e";

            Assert.AreEqual(expectedDid, result.Did);
            Assert.AreEqual(_expectedVerkey, result.VerKey);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksAsCid()
        {
            var json = string.Format("{{\"seed\":\"{0}\",\"cid\":true}}", _seed);

            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, json);
            Assert.IsNotNull(result);

            Assert.AreEqual(_expectedVerkey, result.Did);
            Assert.AreEqual(_expectedVerkey, result.VerKey);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForPassedDid()
        {
            var json = string.Format("{{\"did\":\"{0}\",\"cid\":false}}", _did);

            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, json);
            Assert.IsNotNull(result);

            Assert.AreEqual(_did, result.Did);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForCorrectCryptoType()
        {
            var json = string.Format("{{\"crypto_type\":\"{0}\"}}", _existsCryptoType);

            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, json);
            Assert.IsNotNull(result);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForInvalidSeed()
        {
            var json = "{\"seed\":\"aaaaaaaaaaa\"}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.CreateAndStoreMyDidAsync(_wallet, json)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForInvalidCryptoType()
        {
            var json = "{\"crypto_type\":\"crypto_type\"}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Signus.CreateAndStoreMyDidAsync(_wallet, json)
            );

            Assert.AreEqual(ErrorCode.SignusUnknownCryptoError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForAllParams()
        {
            var json = string.Format("{{\"did\":\"{0}\",\"seed\":\"{1}\",\"crypto_type\":\"{2}\",\"cid\":true}}", _did, _seed, _existsCryptoType);

            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, json);
            Assert.IsNotNull(result);

            Assert.AreEqual(_did, result.Did);
            Assert.AreEqual(_expectedVerkey, result.VerKey);
        }

    }
}
