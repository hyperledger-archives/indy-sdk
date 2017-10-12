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
            await Pairwise.CreateAsync(_wallet, _theirDid, _myDid, METADATA);
        }

        [TestMethod]
        public async Task TestCreatePairwiseWorksForEmptyMetadata()
        {
            await Pairwise.CreateAsync(_wallet, _theirDid, _myDid, null);
        }

        [TestMethod]
        public async Task TestCreatePairwiseWorksForNotFoundMyDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Pairwise.CreateAsync(_wallet, _theirDid, DID1, null)
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestCreatePairwiseWorksForNotFoundTheirDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Pairwise.CreateAsync(_wallet, DID1, _myDid, null)
            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }
    }
}
