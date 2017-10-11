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
    public class GetPairwiseTests : PairwiseIntegrationTestBase
    {
        [TestMethod]
        public async Task TestGetPairwiseWorks()
        {
            await Pairwise.CreateAsync(_wallet, _theirDid, _myDid, METADATA);

            var pairwiseInfoJson = await Pairwise.GetAsync(_wallet, _theirDid);
            var pairwiseInfo = JObject.Parse(pairwiseInfoJson);

            Assert.AreEqual(_myDid, pairwiseInfo.Value<string>("my_did"));
            Assert.AreEqual(METADATA, pairwiseInfo.Value<string>("metadata"));
        }

        [TestMethod]
        public async Task TestGetPairwiseWorksForNotCreated()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Pairwise.GetAsync(_wallet, _theirDid)
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);            
        }
    }
}
