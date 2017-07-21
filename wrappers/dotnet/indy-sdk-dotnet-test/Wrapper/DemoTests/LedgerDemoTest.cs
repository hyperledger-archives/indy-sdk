using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;
using System;

namespace Indy.Sdk.Dotnet.Test.Wrapper.LedgerTests
{
    [TestClass]
    public class LedgerDemoTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public void TestLedgerDemo()
        {
            // 1. Create ledger config from genesis txn file
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var pool = Pool.OpenPoolLedgerAsync(poolName, "{}").Result;

            // 2. Create and Open My Wallet
            Wallet.CreateWalletAsync(poolName, "myWallet", "default", null, null).Wait();
            var myWallet = Wallet.OpenWalletAsync("myWallet", null, null).Result;

            // 3. Create and Open Trustee Wallet
            Wallet.CreateWalletAsync(poolName, "theirWallet", "default", null, null).Wait();
            var trusteeWallet = Wallet.OpenWalletAsync("theirWallet", null, null).Result;

            // 4. Create My Did
            var createMyDidResult = Signus.CreateAndStoreMyDidAsync(myWallet, "{}").Result;
            Assert.IsNotNull(createMyDidResult);
            var myDid = createMyDidResult.Did;
            var myVerkey = createMyDidResult.VerKey;

            // 5. Create Did from Trustee1 seed
            var theirDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}"; 

            var createTheirDidResult = Signus.CreateAndStoreMyDidAsync(trusteeWallet, theirDidJson).Result;
            Assert.IsNotNull(createTheirDidResult);
            var trusteeDid = createTheirDidResult.Did;

            // 6. Build Nym Request
            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null).Result;
            Assert.IsNotNull(nymRequest);

            // 7. Trustee Sign Nym Request
            var nymResponseJson = Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest).Result;
            Assert.IsNotNull(nymResponseJson);

            var nymResponse = JObject.Parse(nymResponseJson);

            Assert.AreEqual(myDid, nymResponse["result"].Value<string>("dest"));
            Assert.AreEqual(myVerkey, nymResponse["result"].Value<string>("verkey"));

            // 8. Close and delete My Wallet
            myWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("myWallet", null).Wait();

            // 9. Close and delete Their Wallet
            trusteeWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("theirWallet", null).Wait();

            // 10. Close Pool
            pool.CloseAsync().Wait();
        }
       
    }
}
