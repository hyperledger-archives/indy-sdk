using Hyperledger.Indy.PairwiseApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class SetPairwiseMetadataTests : PairwiseIntegrationTestBase
    {
        private const string PAIRWISE_TEMPLATE_WITH_META = "{{\"my_did\":\"{0}\",\"metadata\":\"{1}\"}}";
        private const string PAIRWISE_TEMPLATE_WITHOUT_META = "{{\"my_did\":\"{0}\"}}";

        [TestMethod]
        public async Task TestSetPairwiseMetadataWorks()
        {
            await Pairwise.CreateAsync(wallet, theirDid, myDid, null);

            var pairwiseWithoutMetadata = await Pairwise.GetAsync(wallet, theirDid);
            Assert.AreEqual(string.Format(PAIRWISE_TEMPLATE_WITHOUT_META, myDid), pairwiseWithoutMetadata);

            await Pairwise.SetMetadataAsync(wallet, theirDid, METADATA);
            var pairwiseWithMetadata = await Pairwise.GetAsync(wallet, theirDid);

            Assert.AreNotEqual(pairwiseWithoutMetadata, pairwiseWithMetadata);
            Assert.AreEqual(string.Format(PAIRWISE_TEMPLATE_WITH_META, myDid, METADATA), pairwiseWithMetadata);
        }

        [TestMethod]
        public async Task TestSetPairwiseMetadataWorksForReset()
        {
            await Pairwise.CreateAsync(wallet, theirDid, myDid, METADATA);

            var pairwiseWithMetadata = await Pairwise.GetAsync(wallet, theirDid);

            Assert.AreEqual(string.Format(PAIRWISE_TEMPLATE_WITH_META, myDid, METADATA), pairwiseWithMetadata);

            await Pairwise.SetMetadataAsync(wallet, theirDid, null);
            var pairwiseWithoutMetadata = await Pairwise.GetAsync(wallet, theirDid);

            Assert.AreNotEqual(pairwiseWithMetadata, pairwiseWithoutMetadata);
            Assert.AreEqual(string.Format(PAIRWISE_TEMPLATE_WITHOUT_META, myDid), pairwiseWithoutMetadata);
        }

        [TestMethod]
        public async Task TestSetPairwiseMetadataWorksForNotCreatedPairwise()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                Pairwise.SetMetadataAsync(wallet, theirDid, METADATA)
            );
        }

    }
}
