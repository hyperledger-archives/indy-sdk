using Hyperledger.Indy.PairwiseApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
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
        [Ignore] //Bug in SDK?
        public async Task TestSetPairwiseMetadataWorksWithNull()
        {
            await Pairwise.CreateAsync(_wallet, _theirDid, _myDid, METADATA);

            var pairwiseWithMetadata = await Pairwise.GetAsync(_wallet, _theirDid);

            await Pairwise.SetMetadataAsync(_wallet, _theirDid, null);
            var pairwiseWithoutMetadata = await Pairwise.GetAsync(_wallet, _theirDid);
            var pairwiseInfo = JObject.Parse(pairwiseWithoutMetadata);

            Assert.AreNotEqual(pairwiseWithoutMetadata, pairwiseWithMetadata);
            Assert.IsNull(pairwiseInfo["metadata"]);
        }

        [TestMethod]
        [Ignore] //Bug in SDK?
        public async Task TestSetPairwiseMetadataWorksWithEmptyString()
        {
            await Pairwise.CreateAsync(_wallet, _theirDid, _myDid, METADATA);

            var pairwiseWithMetadata = await Pairwise.GetAsync(_wallet, _theirDid);

            await Pairwise.SetMetadataAsync(_wallet, _theirDid, string.Empty);
            var pairwiseWithoutMetadata = await Pairwise.GetAsync(_wallet, _theirDid);
            var pairwiseInfo = JObject.Parse(pairwiseWithoutMetadata);

            Assert.AreNotEqual(pairwiseWithoutMetadata, pairwiseWithMetadata);
            Assert.AreEqual(string.Empty, pairwiseInfo.Value<string>("metadata"));
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
