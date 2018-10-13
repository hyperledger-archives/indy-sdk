using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using System;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DemoTests
{
    [TestClass]
    public class CryptoDemoTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestCryptoDemo()
        {
            //1. Create and Open Pool
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, "{}");

            //2. Create and Open My Wallet
            var myWalletConfig = JsonConvert.SerializeObject(new { id = "myWallet" });
            await Wallet.CreateWalletAsync(myWalletConfig, WALLET_CREDENTIALS);
            var myWallet = await Wallet.OpenWalletAsync(myWalletConfig, WALLET_CREDENTIALS);

            //3. Create and Open Their Wallet
            var theirWalletConfig = JsonConvert.SerializeObject(new { id = "theirWallet" });
            await Wallet.CreateWalletAsync(theirWalletConfig, WALLET_CREDENTIALS);
            var theirWallet = await Wallet.OpenWalletAsync(theirWalletConfig, WALLET_CREDENTIALS);

            //4. Create My Did
            var createMyDidResult = await Did.CreateAndStoreMyDidAsync(myWallet, "{}");
            Assert.IsNotNull(createMyDidResult);

            //5. Create Their Did
            var createTheirDidResult = await Did.CreateAndStoreMyDidAsync(theirWallet, "{}");
            Assert.IsNotNull(createTheirDidResult);
            var theirDid = createTheirDidResult.Did;
            var theirVerkey = createTheirDidResult.VerKey;

            // 6. Store Their DID
            var identityJson = JsonConvert.SerializeObject(new { did = theirDid, verkey = theirVerkey});
            await Did.StoreTheirDidAsync(myWallet, identityJson);

            // 7. Their sign message
            var msg = "{\n" +
                    "        \"reqId\":1495034346617224651,\n" +
                    "        \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\",\n" +
                    "        \"operation\":{\n" +
                    "            \"type\":\"1\",\n" +
                    "            \"dest\":\"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB\"\n" +
                    "        }\n" +
                    "    }";

            var msgBytes = Encoding.UTF8.GetBytes(msg);

            byte[] signature = await Crypto.SignAsync(theirWallet, theirVerkey, msgBytes);

            // 8. I verify message
            Boolean valid = await Crypto.VerifyAsync(theirVerkey, msgBytes, signature);
            Assert.IsTrue(valid);

            // 9. Close and delete My Wallet
            await myWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(myWalletConfig, WALLET_CREDENTIALS);

            // 10. Close and delete Their Wallet
            await theirWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(theirWalletConfig, WALLET_CREDENTIALS);

            //11. Close Pool
            await pool.CloseAsync();
        }
    }
}
