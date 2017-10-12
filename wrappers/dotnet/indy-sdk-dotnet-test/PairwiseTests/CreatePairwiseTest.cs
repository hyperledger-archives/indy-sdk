using Hyperledger.Indy.PairwiseApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class CreatePairwiseTest : PairwiseIntegrationTestBase
    {
        [TestMethod]
        public async Task TestCreatePairwiseWorks()
        {
            await Pairwise.CreateAsync(wallet, _theirDid, _myDid, METADATA);
        }

        [TestMethod]
        public async Task TestCreatePairwiseWorksForEmptyMetadata()
        {
            await Pairwise.CreateAsync(wallet, _theirDid, _myDid, null);
        }

        [TestMethod]
        public async Task TestCreatePairwiseWorksForNotFoundMyDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Pairwise.CreateAsync(wallet, _theirDid, DID1, null)
            );
        }

        [TestMethod]
        public async Task TestCreatePairwiseWorksForNotFoundTheirDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Pairwise.CreateAsync(wallet, DID1, _myDid, null)
            );
        }
    }
}
