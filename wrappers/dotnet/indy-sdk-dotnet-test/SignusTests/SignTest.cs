using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class SignTest : IndyIntegrationTestWithSingleWallet
    {       
        [TestMethod]
        public async Task TestSignWorks()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, MY1_IDENTITY_JSON);
            var did = result.Did;
            
            var expectedSignature = (byte[])(Array)new sbyte[] {- 87, - 41, 8, - 31, 7, 107, 110, 9, - 63, - 94, - 54, - 42, - 94, 66, - 18, - 45, 63, - 47, 12, - 60, 8, - 45, 55, 27, 120, 94,
                - 52, - 109, 53, 104, 103, 61, 60, - 7, - 19, 127, 103, 46, - 36, - 33, 10, 95, 75, 53, - 11, - 46, - 15, - 105, - 65, 41, 48, 30, 9, 16, 78, - 4,
                - 99, - 50, - 46, - 111, 125, - 123, 109, 11};

            var signature = await Signus.SignAsync(wallet, result.Did, MESSAGE);

            Assert.IsTrue(expectedSignature.SequenceEqual(signature));
        }

        [TestMethod]
        public async Task TestSignWorksForUnknownDid()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Signus.SignAsync(wallet, DID1, MESSAGE)
            );
        }       
    }
}
