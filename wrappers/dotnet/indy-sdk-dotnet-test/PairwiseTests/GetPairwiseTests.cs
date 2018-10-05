using Hyperledger.Indy.PairwiseApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class GetPairwiseTests : PairwiseIntegrationTestBase
    {
        [TestMethod]
        public async Task TestGetPairwiseWorks()
        {
            await Pairwise.CreateAsync(wallet, theirVerkey, myDid, METADATA);

            var pairwiseInfoJson = await Pairwise.GetAsync(wallet, theirVerkey);
            var pairwiseInfo = JObject.Parse(pairwiseInfoJson);

            Assert.AreEqual(myDid, pairwiseInfo.Value<string>("my_did"));
            Assert.AreEqual(METADATA, pairwiseInfo.Value<string>("metadata"));
        }

        [TestMethod]
        public async Task TestGetPairwiseWorksWhenNoMetadataIsPresent()
        {
            await Pairwise.CreateAsync(wallet, theirVerkey, myDid, null);

            var pairwiseInfoJson = await Pairwise.GetAsync(wallet, theirVerkey);
            var pairwiseInfo = JObject.Parse(pairwiseInfoJson);

            Assert.AreEqual(myDid, pairwiseInfo.Value<string>("my_did"));
            Assert.IsNull(pairwiseInfo["metadata"]);
        }

        [TestMethod]
        public async Task TestGetPairwiseWalletItemNotFoundExceptionForNotCreated()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                Pairwise.GetAsync(wallet, theirVerkey)
            );          
        }
    }
}
