using Hyperledger.Indy.PairwiseApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class SetPairwiseMetadataTests : PairwiseIntegrationTestBase
    {
        [TestMethod]
        public async Task TestSetPairwiseMetadataWorks()
        {
            await Pairwise.CreateAsync(_wallet, _theirDid, _myDid, null);

            var pairwiseWithoutMetadata = await Pairwise.GetAsync(_wallet, _theirDid);

            await Pairwise.SetMetadataAsync(_wallet, _theirDid, METADATA);
            var pairwiseWithMetadata = await Pairwise.GetAsync(_wallet, _theirDid);

            Assert.AreNotEqual(pairwiseWithoutMetadata, pairwiseWithMetadata);
            Assert.AreEqual(string.Format(PAIRWISE_TEMPLATE, _myDid, METADATA), pairwiseWithMetadata);
        }

        [TestMethod]
        public async Task TestSetPairwiseMetadataWorksForNotCreatedPairwise()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Pairwise.SetMetadataAsync(_wallet, _theirDid, METADATA)
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }
    }
}
