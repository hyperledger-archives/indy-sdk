using Hyperledger.Indy.PairwiseApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class PairwiseExistsTests : PairwiseIntegrationTestBase
    {
        [TestMethod]
        public async Task TestPairwiseExistsWorks()
        {
            await Pairwise.CreateAsync(_wallet, _theirDid, _myDid, null);

            Assert.IsTrue(await Pairwise.IsExistsAsync(_wallet, _theirDid));
        }

        [TestMethod]
        public async Task TestPairwiseExistsWorksForNotCreated()
        {
            Assert.IsFalse(await Pairwise.IsExistsAsync(_wallet, _theirDid));
        }
    }
}
