using Hyperledger.Indy.DidApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test
{
    public abstract class IndyIntegrationTestWithPoolAndSingleWallet : IndyIntegrationTestBase
    {
        protected Pool pool;
        protected Wallet wallet;
        protected string poolName;

        [TestInitialize]
        public async Task CreatePoolAndWallet()
        {
            poolName = PoolUtils.CreatePoolLedgerConfig();
            pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
        }

        [TestCleanup]
        public async Task DeletePoolAndWallet()
        {
            await pool.CloseAsync();
            await wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
        }

        protected void CheckResponseType(string response, string expectedType)
        {
            Assert.IsTrue(CompareResponseType(response, expectedType));
        }

        protected bool CompareResponseType(string response, string expectedType)
        {
            var res = JObject.Parse(response);
            return expectedType == res["op"].ToString();
        }

        protected async Task<string> CreateStoreAndPublishDidFromTrusteeAsync()
        {
            var trusteeDidResult = await Did.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;

            var myDidResult = await Did.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, "TRUSTEE");
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            return myDid;
        }
    }
}
