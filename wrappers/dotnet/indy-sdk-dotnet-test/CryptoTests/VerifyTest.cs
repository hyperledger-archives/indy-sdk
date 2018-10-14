using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.DidApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class VerifyTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestCryptoVerifyWorks()
        {
            var result = await Crypto.VerifyAsync(VERKEY_TRUSTEE, MESSAGE, SIGNATURE);
            Assert.IsTrue(result);
        }

        [TestMethod]
        public async Task TestCryptoVerifyWorksForVerkeyWithCorrectCryptoType()
        {
            var verkey = VERKEY_TRUSTEE + ":ed25519";
            var valid = await Crypto.VerifyAsync(verkey, MESSAGE, SIGNATURE);
            Assert.IsTrue(valid);            
        }

        [TestMethod]
        public async Task TestCryptoVerifyWorksForVerkeyWithInvalidCryptoType()
        {
            var verkey = VERKEY_TRUSTEE + ":unknown_crypto";

            var ex = await Assert.ThrowsExceptionAsync<UnknownCryptoTypeException>(() =>
               Crypto.VerifyAsync(verkey, MESSAGE, SIGNATURE)
           );
        }

        [TestMethod]
        public async Task TestCryptoVerifyWorksForOtherSigner()
        {
            var valid = await Crypto.VerifyAsync(VERKEY_MY2, MESSAGE, SIGNATURE);
            Assert.IsFalse(valid);
        }
    }
}
