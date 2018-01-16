using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DemoTests
{
    [TestClass]
    public class SignusDemoTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestSignusDemo()
        {
            //1. Create and Open Pool
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, "{}");

            //2. Create and Open My Wallet
            await Wallet.CreateWalletAsync(poolName, "myWallet", TYPE, null, null);
            var myWallet = await Wallet.OpenWalletAsync("myWallet", null, null);

            // 3. Create and Open Trustee Wallet
            await Wallet.CreateWalletAsync(poolName, "theirWallet", TYPE, null, null);
            var theirWallet = await Wallet.OpenWalletAsync("theirWallet", null, null);

            //4. Create My Did
            var createMyDidResult = await Signus.CreateAndStoreMyDidAsync(myWallet, "{}");
            Assert.IsNotNull(createMyDidResult);

            //5. Create Their Did
            var createTheirDidResult = await Signus.CreateAndStoreMyDidAsync(theirWallet, "{}");
            Assert.IsNotNull(createTheirDidResult);
            var theirDid = createTheirDidResult.Did;
            var theirVerkey = createTheirDidResult.VerKey;

            // 6. Store Their DID
            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, theirDid, theirVerkey);
            await Signus.StoreTheirDidAsync(myWallet, identityJson);

            // 7. Their sign message
            var msgBytes = Encoding.UTF8.GetBytes("{\n" +
                    "        \"reqId\":1495034346617224651,\n" +
                    "        \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
                    "        \"operation\":{\n" +
                    "            \"type\":\"1\",\n" +
                    "            \"dest\":\"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB\"\n" +
                    "        }\n" +
                    "    }");

            var signatureBytes = await Signus.SignAsync(theirWallet, theirDid, msgBytes);
            Assert.IsNotNull(signatureBytes);

            // 8. I verify message
            Boolean valid = await Signus.VerifySignatureAsync(myWallet, pool, theirDid, msgBytes, signatureBytes);
            Assert.IsTrue(valid);

            // 9. Close and delete My Wallet
            await myWallet.CloseAsync();
            await Wallet.DeleteWalletAsync("myWallet", null);

            // 10. Close and delete Their Wallet
            await theirWallet.CloseAsync();
            await Wallet.DeleteWalletAsync("theirWallet", null);

            //11. Close Pool
            await pool.CloseAsync();
        }
       
    }
}
