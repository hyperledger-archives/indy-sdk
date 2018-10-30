using Hyperledger.Indy.PairwiseApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class ListPairwiseTests : PairwiseIntegrationTestBase
    {
        [TestMethod]
        public async Task TestListPairwiseWorks()
        {
            await Pairwise.CreateAsync(wallet, theirDid, myDid, null);

            var listPairwise = await Pairwise.ListAsync(wallet);
            var listPairwiseArray = JArray.Parse(listPairwise);

            Assert.AreEqual(1, listPairwiseArray.Count);
            Assert.AreEqual(listPairwiseArray[0].ToString(), string.Format(PAIR_TEMPLATE, myDid, theirDid));
        }

        [TestMethod]
        public async Task TestListPairwiseWorksForEmptyResult()
        {
            var listPairwise = await Pairwise.ListAsync(wallet);
            var listPairwiseArray = JArray.Parse(listPairwise);
            Assert.AreEqual(0, listPairwiseArray.Count);
        }
    }
}
