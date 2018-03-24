using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.Test.Util.Base58Check;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class CreateMyDidTest : IndyIntegrationTestWithSingleWallet
    {        
        [TestMethod]
        public async Task TestCreateMyDidWorksForEmptyJson()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            Assert.IsNotNull(result);

            Assert.AreEqual(16, Base58CheckEncoding.DecodePlain(result.Did).Length);
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(result.VerKey).Length);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForSeed()
        {
            var result = await Did.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            Assert.IsNotNull(result);

            Assert.AreEqual(DID_MY1, result.Did);
            Assert.AreEqual(VERKEY_MY1, result.VerKey);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksAsCid()
        {
            var json = string.Format("{{\"seed\":\"{0}\",\"cid\":true}}", MY1_SEED);

            var result = await Did.CreateAndStoreMyDidAsync(wallet, json);
            Assert.IsNotNull(result);

            Assert.AreEqual(VERKEY_MY1, result.Did);
            Assert.AreEqual(VERKEY_MY1, result.VerKey);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForPassedDid()
        {
            var json = string.Format("{{\"did\":\"{0}\",\"cid\":false}}", DID1);

            var result = await Did.CreateAndStoreMyDidAsync(wallet, json);
            Assert.IsNotNull(result);

            Assert.AreEqual(DID1, result.Did);
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForCorrectCryptoType()
        {
            var json = string.Format("{{\"seed\":\"{0}\",\"crypto_type\":\"{1}\"}}", MY1_SEED, CRYPTO_TYPE);

            var result = await Did.CreateAndStoreMyDidAsync(wallet, json);


            Assert.AreEqual(DID_MY1, result.Did);
            Assert.AreEqual(VERKEY_MY1, result.VerKey); 
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForInvalidSeed()
        {
            var json = "{\"seed\":\"aaaaaaaaaaa\"}";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Did.CreateAndStoreMyDidAsync(wallet, json)
            );
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForInvalidCryptoType()
        {
            var json = string.Format("{{\"seed\":\"{0}\",\"crypto_type\":\"crypto_type\"}}", MY1_SEED);

            var ex = await Assert.ThrowsExceptionAsync<UnknownCryptoException>(() =>
                Did.CreateAndStoreMyDidAsync(wallet, json)
            );
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForAllParams()
        {
            var json = string.Format("{{\"did\":\"{0}\",\"seed\":\"{1}\",\"crypto_type\":\"{2}\",\"cid\":true}}", DID1, MY1_SEED, CRYPTO_TYPE);

            var result = await Did.CreateAndStoreMyDidAsync(wallet, json);
            Assert.IsNotNull(result);

            Assert.AreEqual(DID1, result.Did);
            Assert.AreEqual(VERKEY_MY1, result.VerKey);
        }

    }
}
