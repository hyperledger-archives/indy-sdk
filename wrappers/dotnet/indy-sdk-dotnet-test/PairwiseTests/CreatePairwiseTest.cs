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
            await Pairwise.CreateAsync(wallet, theirVerkey, myDid, METADATA);
        }

        [TestMethod]
        public async Task TestCreatePairwiseWorksForEmptyMetadata()
        {
            await Pairwise.CreateAsync(wallet, theirVerkey, myDid, null);
        }

        [TestMethod]
        public async Task TestCreatePairwiseWalletItemNotFoundExceptionForNotFoundMyDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                Pairwise.CreateAsync(wallet, theirVerkey, DID1, null)
            );
        }

        [TestMethod]
        public async Task TestCreatePairwiseWalletItemNotFoundExceptionForNotFoundTheirDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletItemNotFoundException>(() =>
                Pairwise.CreateAsync(wallet, DID1, myDid, null)
            );
        }
    }
}
