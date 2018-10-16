using Hyperledger.Indy.PairwiseApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class PairwiseExistsTests : PairwiseIntegrationTestBase
    {
        [TestMethod]
        public async Task TestPairwiseExistsWorks()
        {
            await Pairwise.CreateAsync(wallet, theirDid, theirDid, null);

            Assert.IsTrue(await Pairwise.IsExistsAsync(wallet, theirDid));
        }

        [TestMethod]
        public async Task TestPairwiseExistsWorksForNotCreated()
        {
            Assert.IsFalse(await Pairwise.IsExistsAsync(wallet, theirDid));
        }
    }
}
