using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class StoreTheirDidTest : IndyIntegrationTestWithSingleWallet
    {
        private const string _verkey = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";
             
        [TestMethod]
        public async Task TestStoreTheirDidWorks()
        {
            await Signus.StoreTheirDidAsync(wallet, string.Format("{{\"did\":\"{0}\"}}", DID1));
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForInvalidIdentityJson()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Signus.StoreTheirDidAsync(wallet, "{\"field\":\"value\"}")
            );            
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksWithVerkey()
        {
            var json = string.Format(IDENTITY_JSON_TEMPLATE, DID1, _verkey);

            await Signus.StoreTheirDidAsync(wallet, json);
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksWithoutDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Signus.StoreTheirDidAsync(wallet, string.Format("{{\"verkey\":\"{0}\"}}", _verkey))
            );
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksForCorrectCryptoType()
        {
            var json = string.Format("{{\"did\":\"{0}\", " +
                "\"verkey\":\"{1}\", " +
                "\"crypto_type\": \"ed25519\"}}", DID1, _verkey);

            await Signus.StoreTheirDidAsync(wallet, json);
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksForInvalidCryptoType()
        {
            var json = string.Format("{{\"did\":\"{0}\", " +
                "\"verkey\":\"{1}\", " +
                "\"crypto_type\": \"some_type\"}}", DID1, _verkey);

            var ex = await Assert.ThrowsExceptionAsync<UnknownCryptoException>(() =>
                Signus.StoreTheirDidAsync(wallet, json)
            );
        }


    }
}
