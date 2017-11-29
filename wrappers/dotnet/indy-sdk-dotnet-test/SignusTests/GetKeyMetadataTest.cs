using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class GetKeyMetadataTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestGetKeyMetadataWorks()
        {
            await Signus.SetKeyMetadataAsync(wallet, VERKEY, METADATA);
            var receivedMetadata = await Signus.GetKeyMetadataAsync(wallet, VERKEY);
            Assert.AreEqual(METADATA, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetKeyMetadataWorksForEmptyString()
        {
            await Signus.SetKeyMetadataAsync(wallet, VERKEY, string.Empty);
            var receivedMetadata = await Signus.GetKeyMetadataAsync(wallet, VERKEY);
            Assert.AreEqual(string.Empty, receivedMetadata);
        }

        [TestMethod]
        public async Task TestGetKeyMetadataWorksForNoMetadata()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
               Signus.GetKeyMetadataAsync(wallet, VERKEY)
           );
        }
    }
}
