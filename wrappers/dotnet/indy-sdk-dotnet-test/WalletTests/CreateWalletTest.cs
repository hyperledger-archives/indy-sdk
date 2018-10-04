using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.WalletTests
{
    [TestClass]
    public class CreateWalletTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestCreateWalletWorks()
        {
            WalletConfig config = new WalletConfig() { id = WALLET };
            Credentials cred = new Credentials() { key = WALLET_KEY };

            await Wallet.CreateWalletAsync(config, cred);
        }

        [TestMethod]
        public async Task TestCreateWalletFailsForDuplicateName()
        {
            WalletConfig config = new WalletConfig() { id = WALLET };
            Credentials cred = new Credentials() { key = WALLET_KEY };

            await Wallet.CreateWalletAsync(config, cred);

            var ex = await Assert.ThrowsExceptionAsync<WalletExistsException>(async () => 
                await Wallet.CreateWalletAsync(config, cred)
            );
        }
    }
}
