using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.Test.Util.Base58Check;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class CreateKeyTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestCreateKeyWorksForSeed()
        {
            var senderVk = await Crypto.CreateKeyAsync(wallet, MY1_IDENTITY_KEY_JSON);
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(senderVk).Length);
        }

        [TestMethod]
        public async Task TestCreateKeyWorksWithoutSeed()
        {
            var senderVk = await Crypto.CreateKeyAsync(wallet, "{}");
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(senderVk).Length);
        }

        [TestMethod]
        public async Task TestCreateKeyWorksForInvalidSeed()
        {
            var paramJson = string.Format("{{\"seed\":\"{0}\"}}", "invalidSeedLength");

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
               Crypto.CreateKeyAsync(wallet, paramJson)
           );
        }
    }
}
