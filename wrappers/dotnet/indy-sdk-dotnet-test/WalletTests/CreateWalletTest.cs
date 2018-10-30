using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class CreateWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestCreateWalletWorks()
        {
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
        }
        
        [TestMethod]
        public async Task TestCreateWalletWorksForEmptyType()
        {
            var config = JsonConvert.SerializeObject(new { id = WalletUtils.GetWalletId() });
            await Wallet.CreateWalletAsync(config, WALLET_CREDENTIALS);
        }


        [TestMethod]
        public async Task TestCreateWalletWorksForUnknownType()
        {
            var config = JsonConvert.SerializeObject(new {
                id = WalletUtils.GetWalletId(),
                storage_type = "unknown_type"
            });

            var ex = await Assert.ThrowsExceptionAsync<UnknownWalletTypeException>(() =>
                Wallet.CreateWalletAsync(config, WALLET_CREDENTIALS)
            );
        }

        [TestMethod]
        public async Task TestCreateWalletWorksForEmptyName()
        {
            var config = JsonConvert.SerializeObject(new { id = string.Empty });

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Wallet.CreateWalletAsync(config, WALLET_CREDENTIALS)
            );
        }

        [TestMethod]
        public async Task TestCreateWalletFailsForDuplicate()
        {
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            var ex = await Assert.ThrowsExceptionAsync<WalletExistsException>(() =>
               Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS)
            );
        }
    }
}
