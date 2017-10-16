﻿using Hyperledger.Indy.PairwiseApi;
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
            await Pairwise.CreateAsync(wallet, theirDid, myDid, METADATA);

            var pairwiseInfoJson = await Pairwise.GetAsync(wallet, theirDid);
            var pairwiseInfo = JObject.Parse(pairwiseInfoJson);

            Assert.AreEqual(myDid, pairwiseInfo.Value<string>("my_did"));
            Assert.AreEqual(METADATA, pairwiseInfo.Value<string>("metadata"));
        }

        [TestMethod]
        public async Task TestGetPairwiseWorksWhenNoMetadataIsPresent()
        {
            await Pairwise.CreateAsync(wallet, theirDid, myDid, null);

            var pairwiseInfoJson = await Pairwise.GetAsync(wallet, theirDid);
            var pairwiseInfo = JObject.Parse(pairwiseInfoJson);

            Assert.AreEqual(myDid, pairwiseInfo.Value<string>("my_did"));
            Assert.IsNull(pairwiseInfo["metadata"]);
        }

        [TestMethod]
        public async Task TestGetPairwiseWorksForNotCreated()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Pairwise.GetAsync(wallet, theirDid)
            );          
        }
    }
}
