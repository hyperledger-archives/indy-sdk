using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class GenerateWalletKeyTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestGenerateWalletKeyWorks()
        {
            var key = await Wallet.GenerateWalletKeyAsync(null);
            var credentials = JsonConvert.SerializeObject(new { key = key, key_derivation_method = "RAW"});

            await Wallet.CreateWalletAsync(WALLET_CONFIG, credentials);
        }

        [TestMethod]
        public async Task TestGenerateWalletKeyWorksForSeed()
        {
            var config = JsonConvert.SerializeObject(new { seed = MY1_SEED });
            var key = await Wallet.GenerateWalletKeyAsync(config);

            Assert.AreEqual("CwMHrEQJnwvuE8q9zbR49jyYtVxVBHNTjCPEPk1aV3cP", key);

            var credentials = JsonConvert.SerializeObject(new { key = key, key_derivation_method = "RAW" });
            await Wallet.CreateWalletAsync(WALLET_CONFIG, credentials);
        }
    }
}
