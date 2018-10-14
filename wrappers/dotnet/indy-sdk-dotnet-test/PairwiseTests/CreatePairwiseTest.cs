using Hyperledger.Indy.PairwiseApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class CreatePairwiseTest : PairwiseIntegrationTestBase
    {
        [TestMethod]
        public async Task TestCreatePairwiseWorks()
        {
            await Pairwise.CreateAsync(wallet, theirDid, myDid, METADATA);
        }

        [TestMethod]
        public async Task TestCreatePairwiseWorksForEmptyMetadata()
        {
            await Pairwise.CreateAsync(wallet, theirDid, myDid, null);
        }

        [TestMethod]
        public async Task TestCreatePairwiseWorksForNotFoundMyDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                Pairwise.CreateAsync(wallet, theirDid, DID, null)
            );
        }

        [TestMethod]
        public async Task TestCreatePairwiseWorksForNotFoundTheirDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                Pairwise.CreateAsync(wallet, DID, myDid, null)
            );
        }
    }
}
