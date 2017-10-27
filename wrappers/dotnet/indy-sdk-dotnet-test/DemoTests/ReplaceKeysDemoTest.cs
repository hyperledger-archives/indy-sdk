using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.SignusApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.DemoTests
{
    [TestClass]
    public class ReplaceKeysDemoTest : IndyIntegrationTestWithPoolAndSingleWallet
    {   
        [TestMethod]
        public async Task TestReplaceKeysDemoWorks()
        {
            // 1. Create My Did
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = result.Did;
            var myVerkey = result.VerKey;

            // 2. Create Their Did from Trustee1 seed
            var createTheirDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = createTheirDidResult.Did;

            // 3. Build and send Nym Request
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            // 4. Start replacing of keys
            var newKeys = await Signus.ReplaceKeysStartAsync(wallet, myDid, "{}");
            var newVerkey = newKeys.VerKey;

            // 5. Build and send Nym Request with new key
            nymRequest = await Ledger.BuildNymRequestAsync(myDid, myDid, newVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, nymRequest);

            // 6. Apply replacing of keys
            await Signus.ReplaceKeysApplyAsync(wallet, myDid);

            // 7. Send schema request
            var schemaRequest = await Ledger.BuildSchemaRequestAsync(myDid, SCHEMA_DATA);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, schemaRequest);
        }

        [TestMethod]
        public async Task TestReplaceKeysWithoutNymTransaction()
        {
            // 1. Create My Did
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");
            var myDid = result.Did;
            var myVerkey = result.VerKey;

            // 2. Create Their Did from Trustee1 seed
            var createTheirDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = createTheirDidResult.Did;

            // 3. Build and send Nym Request
            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, trusteeDid, nymRequest);

            // 4. Start replacing of keys
            await Signus.ReplaceKeysStartAsync(wallet, myDid, "{}");

            // 5. Apply replacing of keys
            await Signus.ReplaceKeysApplyAsync(wallet, myDid);

            // 6. Send schema request
            var schemaRequest = await Ledger.BuildSchemaRequestAsync(myDid, SCHEMA_DATA);

            var ex = await Assert.ThrowsExceptionAsync<InvalidLedgerTransactionException>(() =>
               Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, schemaRequest)
            );
        }
    }
}
