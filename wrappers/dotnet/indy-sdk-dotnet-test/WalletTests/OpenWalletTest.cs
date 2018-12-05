using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class OpenWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestOpenWalletWorks()
        {
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            var wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            Assert.IsNotNull(wallet);
            await wallet.CloseAsync();
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForForInvalidCredentials()
        {
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            var ex = await Assert.ThrowsExceptionAsync<WalletAccessFailedException>(() =>
                Wallet.OpenWalletAsync(WALLET_CONFIG, "{\"key\": \"other_key\"}")
            );
        }

        [TestMethod]
        public async Task TestOpenWalletWorksForChangingCredentials()
        {
            await Wallet.CreateWalletAsync(WALLET_CONFIG, "{\"key\": \"key\"}");
            
            var wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, "{\"key\": \"key\", \"rekey\": \"other_key\"}");
            await wallet.CloseAsync();

            wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, "{\"key\": \"other_key\"}");
            await wallet.CloseAsync();
        }
        
        [TestMethod]
        public async Task TestOpenWalletWorksForNotCreated()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletNotFoundException>(() =>
               Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS)
            );
        }
        
        [TestMethod]
        public async Task TestOpenWalletWorksForTwice()
        {
            var config = JsonConvert.SerializeObject(new { id = "openWalletWorksForTwice" });

            await Wallet.CreateWalletAsync(config, WALLET_CREDENTIALS);
            var wallet = await Wallet.OpenWalletAsync(config, WALLET_CREDENTIALS);

            var ex = await Assert.ThrowsExceptionAsync<WalletAlreadyOpenedException>(() =>
               Wallet.OpenWalletAsync(config, WALLET_CREDENTIALS)
            );

            await wallet.CloseAsync();
        }        
    }
}
