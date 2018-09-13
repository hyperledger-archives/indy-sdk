using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;
using Hyperledger.Indy.Test.Util;

namespace Hyperledger.Indy.Test.DemoTests
{
    [TestClass]
    public class LedgerDemoTest : IndyIntegrationTestBase
    {
        private const string WALLET_NAME = "commonWallet";
        private const string TRUSTEE_WALLET_NAME = "trusteeWallet";
        private const string TRUSTEE_WALLET_KEY = "trusteeWalletKey";

        [TestMethod]
        public async Task TestLedgerDemo()
        {
            // 1. Create ledger config from genesis txn file
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            var pool = await Pool.OpenPoolLedgerAsync(poolName, "{}");

            // 2. Create and Open My Wallet
            await WalletUtils.CreateWallet(WALLET_NAME, WALLET_KEY);
            var myWallet = await WalletUtils.OpenWallet(WALLET_NAME, WALLET_KEY);

            // 3. Create and Open Trustee Wallet
            await WalletUtils.CreateWallet(TRUSTEE_WALLET_NAME, TRUSTEE_WALLET_KEY);
            var trusteeWallet = await WalletUtils.OpenWallet(TRUSTEE_WALLET_NAME, TRUSTEE_WALLET_KEY);

            // 4. Create My Did
            var createMyDidResult = await Did.CreateAndStoreMyDidAsync(myWallet, "{}");
            Assert.IsNotNull(createMyDidResult);
            var myDid = createMyDidResult.Did;
            var myVerkey = createMyDidResult.VerKey;

            // 5. Create Did from Trustee1 seed
            var createTheirDidResult = await Did.CreateAndStoreMyDidAsync(trusteeWallet, TRUSTEE_IDENTITY_JSON);
            Assert.IsNotNull(createTheirDidResult);
            var trusteeDid = createTheirDidResult.Did;

            // 6. Build Nym Request
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            Assert.IsNotNull(nymRequest);

            // 7. Trustee Sign Nym Request
            var nymResponseJson = await Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest);
            Assert.IsNotNull(nymResponseJson);

            var nymResponse = JObject.Parse(nymResponseJson);

            Assert.AreEqual(myDid, nymResponse["result"].Value<string>("dest"));
            Assert.AreEqual(myVerkey, nymResponse["result"].Value<string>("verkey"));

            // 8. Close and delete My Wallet
            await myWallet.CloseAsync();
            await WalletUtils.DeleteWallet(WALLET_NAME, WALLET_KEY);

            // 9. Close and delete Their Wallet
            await trusteeWallet.CloseAsync();
            await WalletUtils.DeleteWallet(TRUSTEE_WALLET_NAME, TRUSTEE_WALLET_KEY);

            // 10. Close Pool
            await pool.CloseAsync();
        }
       
    }
}
