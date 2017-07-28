using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;
using System;

namespace Indy.Sdk.Dotnet.Test.Wrapper.DemoTests
{
    [TestClass]
    public class SignusDemoTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public void TestSignusDemo()
        {
            //1. Create and Open Pool
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var pool = Pool.OpenPoolLedgerAsync(poolName, "{}").Result;

            //2. Create and Open My Wallet
            Wallet.CreateWalletAsync(poolName, "myWallet", "default", null, null).Wait();
            var myWallet = Wallet.OpenWalletAsync("myWallet", null, null).Result;

            // 3. Create and Open Trustee Wallet
            Wallet.CreateWalletAsync(poolName, "theirWallet", "default", null, null).Wait();
            var theirWallet = Wallet.OpenWalletAsync("theirWallet", null, null).Result;

            //4. Create My Did
            var createMyDidResult = Signus.CreateAndStoreMyDidAsync(myWallet, "{}").Result;
            Assert.IsNotNull(createMyDidResult);

            //5. Create Their Did
            var createTheirDidResult = Signus.CreateAndStoreMyDidAsync(theirWallet, "{}").Result;
            Assert.IsNotNull(createTheirDidResult);
            var theirDid = createTheirDidResult.Did;
            var theirVerkey = createTheirDidResult.VerKey;

            // 6. Store Their DID
            var identityJson = string.Format("{{\"did\":\"{0}\", \"verkey\":\"{1}\"}}", theirDid, theirVerkey);
            Signus.StoreTheirDidAsync(myWallet, identityJson).Wait();

            // 7. Their sign message
            var msg = "{\n" +
                    "        \"reqId\":1495034346617224651,\n" +
                    "        \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
                    "        \"operation\":{\n" +
                    "            \"type\":\"1\",\n" +
                    "            \"dest\":\"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB\"\n" +
                    "        }\n" +
                    "    }";

            var signedMessage = Signus.SignAsync(theirWallet, theirDid, msg).Result;
            Assert.IsNotNull(signedMessage);

            // 8. I verify message
            Boolean valid = Signus.VerifySignatureAsync(myWallet, pool, theirDid, signedMessage).Result;
            Assert.IsTrue(valid);

            // 9. Close and delete My Wallet
            myWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("myWallet", null).Wait();

            // 10. Close and delete Their Wallet
            theirWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync("theirWallet", null).Wait();

            //11. Close Pool
            pool.CloseAsync().Wait();
        }
       
    }
}
