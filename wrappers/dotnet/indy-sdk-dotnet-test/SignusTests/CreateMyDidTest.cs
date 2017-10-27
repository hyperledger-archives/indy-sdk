using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.Test.Util.Base58Check;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class CreateMyDidTest : IndyIntegrationTestWithSingleWallet
    {
        private const string _expectedDid = "VsKV7grR1BUE29mG2Fm2kX";
        private const string _expectedVerkey = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";
        private const string _existsCryptoType = "ed25519";
        
        [TestMethod]
        public async Task TestCreateMyDidWorksForEmptyJson()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");
            Assert.IsNotNull(result);

            Assert.AreEqual(16, Base58CheckEncoding.DecodePlain(result.Did).Length);
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(result.VerKey).Length);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForSeed()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            Assert.IsNotNull(result);

            Assert.AreEqual(_expectedDid, result.Did);
            Assert.AreEqual(_expectedVerkey, result.VerKey);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksAsCid()
        {
            var json = string.Format("{{\"seed\":\"{0}\",\"cid\":true}}", MY1_SEED);

            var result = await Signus.CreateAndStoreMyDidAsync(wallet, json);
            Assert.IsNotNull(result);

            Assert.AreEqual(_expectedVerkey, result.Did);
            Assert.AreEqual(_expectedVerkey, result.VerKey);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForPassedDid()
        {
            var json = string.Format("{{\"did\":\"{0}\",\"cid\":false}}", DID1);

            var result = await Signus.CreateAndStoreMyDidAsync(wallet, json);
            Assert.IsNotNull(result);

            Assert.AreEqual(DID1, result.Did);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForCorrectCryptoType()
        {
            var json = string.Format("{{\"seed\":\"{0}\",\"crypto_type\":\"{1}\"}}", MY1_SEED, _existsCryptoType);

            var result = await Signus.CreateAndStoreMyDidAsync(wallet, json);
            Assert.IsNotNull(result);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForInvalidSeed()
        {
            var json = "{\"seed\":\"aaaaaaaaaaa\"}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Signus.CreateAndStoreMyDidAsync(wallet, json)
            );
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForInvalidCryptoType()
        {
            var json = string.Format("{{\"seed\":\"{0}\",\"crypto_type\":\"crypto_type\"}}", MY1_SEED);

            var ex = await Assert.ThrowsExceptionAsync<UnknownCryptoException>(() =>
                Signus.CreateAndStoreMyDidAsync(wallet, json)
            );
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForAllParams()
        {
            var json = string.Format("{{\"did\":\"{0}\",\"seed\":\"{1}\",\"crypto_type\":\"{2}\",\"cid\":true}}", DID1, MY1_SEED, _existsCryptoType);

            var result = await Signus.CreateAndStoreMyDidAsync(wallet, json);
            Assert.IsNotNull(result);

            Assert.AreEqual(DID1, result.Did);
            Assert.AreEqual(_expectedVerkey, result.VerKey);
        }

    }
}
