using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class DeleteWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestDeleteWalletWorks()
        {
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            await Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            await Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForClosed()
        {
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            var wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            Assert.IsNotNull(wallet);

            await wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            await Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForOpened()
        {
            var config = JsonConvert.SerializeObject(new { id = "deleteWalletWorksForOpened" });

            await Wallet.CreateWalletAsync(config, WALLET_CREDENTIALS);
            var wallet = await Wallet.OpenWalletAsync(config, WALLET_CREDENTIALS);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStateException>(() =>
                Wallet.DeleteWalletAsync(config, WALLET_CREDENTIALS)
            );

            await wallet.CloseAsync();
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForTwice()
        {
            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            var wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            await wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);

            var ex = await Assert.ThrowsExceptionAsync<WalletNotFoundException>(() =>
                 Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS)
            );        
        }

        [TestMethod]
        public async Task TestDeleteWalletWorksForNotCreated()
        {
            var ex = await Assert.ThrowsExceptionAsync<WalletNotFoundException>(() =>
                Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS)
            );
        }
    }
}
