using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DidTests
{
    [TestClass]
    public class StoreTheirDidTest : IndyIntegrationTestWithSingleWallet
    {
        private const string _verkey = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";
             
        [TestMethod]
        public async Task TestStoreTheirDidWorks()
        {
            await Did.StoreTheirDidAsync(wallet, string.Format("{{\"did\":\"{0}\"}}", DID));
        }

        [TestMethod]
        public async Task TestCreateMyDidWorksForInvalidIdentityJson()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Did.StoreTheirDidAsync(wallet, "{\"field\":\"value\"}")
            );            
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksWithVerkey()
        {
            var json = string.Format(IDENTITY_JSON_TEMPLATE, DID, _verkey);

            await Did.StoreTheirDidAsync(wallet, json);
        }

        [TestMethod]
        public async Task TestStoreTheirDidWorksWithoutDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Did.StoreTheirDidAsync(wallet, string.Format("{{\"verkey\":\"{0}\"}}", _verkey))
            );
        }
    }
}
