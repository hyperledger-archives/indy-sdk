using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DemoTests
{
    [TestClass]
    public class ReplaceKeysDemoTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "signusWallet";
        private string _schemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\",\"attr_names\": [\"name\", \"male\"]}";


        [TestInitialize]
        public async Task CreateWalletWithDid()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();
            _pool = await Pool.OpenPoolLedgerAsync(poolName, "{}");

            await Wallet.CreateWalletAsync(poolName, _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await _wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(_walletName, null);
            await _pool.CloseAsync();
        }


        [TestMethod]
        public async Task TestReplaceKeysDemoWorks()
        {
            // 1. Create My Did
            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");
            var myDid = result.Did;
            var myVerkey = result.VerKey;

            // 2. Create Their Did from Trustee1 seed
            var theirDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var createTheirDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, theirDidJson);
            var trusteeDid = createTheirDidResult.Did;

            // 3. Build and send Nym Request
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest);

            // 4. Start replacing of keys
            var newKeys = await Signus.ReplaceKeysStartAsync(_wallet, myDid, "{}");
            var newVerkey = newKeys.VerKey;

            // 5. Build and send Nym Request with new key
            nymRequest = await Ledger.BuildNymRequestAsync(myDid, myDid, newVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, nymRequest);

            // 6. Apply replacing of keys
            await Signus.ReplaceKeysApplyAsync(_wallet, myDid);

            // 7. Send schema request
            var schemaRequest = await Ledger.BuildSchemaRequestAsync(myDid, _schemaData);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, schemaRequest);
        }

        [TestMethod]
        public async Task TestReplaceKeysWithoutNymTransaction()
        {
            // 1. Create My Did
            var result = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");
            var myDid = result.Did;
            var myVerkey = result.VerKey;

            // 2. Create Their Did from Trustee1 seed
            var theirDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var createTheirDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, theirDidJson);
            var trusteeDid = createTheirDidResult.Did;

            // 3. Build and send Nym Request
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest);

            // 4. Start replacing of keys
            await Signus.ReplaceKeysStartAsync(_wallet, myDid, "{}");

            // 5. Apply replacing of keys
            await Signus.ReplaceKeysApplyAsync(_wallet, myDid);

            // 6. Send schema request
            var schemaRequest = await Ledger.BuildSchemaRequestAsync(myDid, _schemaData);

            var ex = await Assert.ThrowsExceptionAsync<InvalidLedgerTransactionException>(() =>
               Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, schemaRequest)
            );
        }
    }
}
