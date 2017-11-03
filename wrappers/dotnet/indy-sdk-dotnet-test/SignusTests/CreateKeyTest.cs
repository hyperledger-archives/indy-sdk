using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.Test.Util.Base58Check;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.SignusTests
{
    [TestClass]
    public class CreateKeyTest : IndyIntegrationTestWithSingleWallet
    {
        [TestMethod]
        public async Task TestCreateKeyWorksForSeed()
        {
            var paramJson = string.Format("{{\"seed\":\"{0}\"}}", MY1_SEED);
            var senderVk = await Signus.CreateKeyAsync(wallet, paramJson);
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(senderVk).Length);
        }

        [TestMethod]
        public async Task TestCreateKeyWorksWithoutSeed()
        {
            var senderVk = await Signus.CreateKeyAsync(wallet, "{}");
            Assert.AreEqual(32, Base58CheckEncoding.DecodePlain(senderVk).Length);
        }

        [TestMethod]
        public async Task TestCreateKeyWorksForInvalidSeed()
        {
            var paramJson = string.Format("{{\"seed\":\"{0}\"}}", "invalidSeedLength");

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
               Signus.CreateKeyAsync(wallet, paramJson)
           );
        }
    }
}
