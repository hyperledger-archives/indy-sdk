using Hyperledger.Indy.CryptoApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class VerifyTest : CryptoIntegrationTestBase
    {
        [TestMethod]
        public async Task TestVerifyWorksWhenAllMatch()
        {
            var result = await Crypto.VerifyAsync(senderVerKey, MESSAGE, SIGNATURE);
            Assert.IsTrue(result);
        }

        [TestMethod]
        public async Task TestVerifyWorksKeyDoesNotMatchSignature()
        {
            var result = await Crypto.VerifyAsync(recipientVerKey, MESSAGE, SIGNATURE);
            Assert.IsFalse(result);            
        }

        [TestMethod]
        public async Task TestVerifyWorksWhenSignatureDoesNotMatchMessage()
        {
            var otherMessage = new byte[] { 1, 2 };

            var result = await Crypto.VerifyAsync(senderVerKey, otherMessage, SIGNATURE);
            Assert.IsFalse(result);
        }

    }
}
