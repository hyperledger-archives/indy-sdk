using Hyperledger.Indy.PairwiseApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class SetPairwiseMetadataTests : PairwiseIntegrationTestBase
    {
        [TestMethod]
        public async Task TestSetPairwiseMetadataWorks()
        {
            await Pairwise.CreateAsync(wallet, theirDid, myDid, null);

            var pairwiseWithoutMetadata = await Pairwise.GetAsync(wallet, theirDid);

            await Pairwise.SetMetadataAsync(wallet, theirDid, METADATA);
            var pairwiseWithMetadata = await Pairwise.GetAsync(wallet, theirDid);

            Assert.AreNotEqual(pairwiseWithoutMetadata, pairwiseWithMetadata);
            Assert.AreEqual(string.Format(PAIRWISE_TEMPLATE, myDid, METADATA), pairwiseWithMetadata);
        }

        [TestMethod]
        [Ignore] //Bug in SDK?
        public async Task TestSetPairwiseMetadataWorksWithNull()
        {
            await Pairwise.CreateAsync(wallet, theirDid, myDid, METADATA);

            var pairwiseWithMetadata = await Pairwise.GetAsync(wallet, theirDid);

            await Pairwise.SetMetadataAsync(wallet, theirDid, null);
            var pairwiseWithoutMetadata = await Pairwise.GetAsync(wallet, theirDid);
            var pairwiseInfo = JObject.Parse(pairwiseWithoutMetadata);

            Assert.AreNotEqual(pairwiseWithoutMetadata, pairwiseWithMetadata);
            Assert.IsNull(pairwiseInfo["metadata"]);
        }

        [TestMethod]
        [Ignore] //Bug in SDK?
        public async Task TestSetPairwiseMetadataWorksWithEmptyString()
        {
            await Pairwise.CreateAsync(wallet, theirDid, myDid, METADATA);

            var pairwiseWithMetadata = await Pairwise.GetAsync(wallet, theirDid);

            await Pairwise.SetMetadataAsync(wallet, theirDid, string.Empty);
            var pairwiseWithoutMetadata = await Pairwise.GetAsync(wallet, theirDid);
            var pairwiseInfo = JObject.Parse(pairwiseWithoutMetadata);

            Assert.AreNotEqual(pairwiseWithoutMetadata, pairwiseWithMetadata);
            Assert.AreEqual(string.Empty, pairwiseInfo.Value<string>("metadata"));
        }

        [TestMethod]
        public async Task TestSetPairwiseMetadataWorksForNotCreatedPairwise()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Pairwise.SetMetadataAsync(wallet, theirDid, METADATA)
            );
        }

    }
}
