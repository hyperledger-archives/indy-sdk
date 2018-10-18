using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.BlobStorageApi;
using Hyperledger.Indy.LedgerApi;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.LedgerTests
{
    public class LedgerIntegrationTestBase : IndyIntegrationTestWithPoolAndSingleWallet
    {
        private static bool entitiesPosted = false;

        protected static string schemaId;
        protected static string credDefId;
        protected static string revRegDefId;

        public async Task PostEntitiesAsync() 
        {
            if (entitiesPosted) {
                return;
            }

            var myDid = await CreateStoreAndPublishDidFromTrusteeAsync();

            // create and post credential schema
            var createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(myDid, GVT_SCHEMA_NAME, SCHEMA_VERSION, GVT_SCHEMA_ATTRIBUTES);
            var schema = createSchemaResult.SchemaJson;
            schemaId = createSchemaResult.SchemaId;

            var schemaRequest = await Ledger.BuildSchemaRequestAsync(myDid, schema);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, schemaRequest);

            var getSchemaRequest = await Ledger.BuildGetSchemaRequestAsync(myDid, schemaId);
            var getSchemaResponse = await PoolUtils.EnsurePreviousRequestAppliedAsync(pool, getSchemaRequest, response => {
                var getSchemaResponseObject = JObject.Parse(response);
			    return getSchemaResponseObject["result"]["seqNo"] != null;
            });

		    var parseSchemaResult = await Ledger.ParseGetSchemaResponseAsync(getSchemaResponse);

            // create and post credential definition
            var createCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(wallet, myDid, parseSchemaResult.ObjectJson, TAG, null, REV_CRED_DEF_CONFIG);
            var credDefJson = createCredDefResult.CredDefJson;
            credDefId = createCredDefResult.CredDefId;

            var credDefRequest = await Ledger.BuildCredDefRequestAsync(myDid, credDefJson);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, credDefRequest);

            // create and post revocation registry
            var tailsWriter = await BlobStorage.OpenWriterAsync("default", TAILS_WRITER_CONFIG);
            var revRegConfig = "{\"issuance_type\":null,\"max_cred_num\":5}";
            var createRevRegResult = await AnonCreds.IssuerCreateAndStoreRevocRegAsync(wallet, myDid, null, TAG, credDefId, revRegConfig, tailsWriter);
            revRegDefId = createRevRegResult.RevRegId;
		    var revRegDef = createRevRegResult.RevRegDefJson;
            var revRegEntry = createRevRegResult.RevRegEntryJson;

            var revRegDefRequest = await Ledger.BuildRevocRegDefRequestAsync(myDid, revRegDef);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, revRegDefRequest);

            var revRegEntryRequest = await Ledger.BuildRevocRegEntryRequestAsync(myDid, revRegDefId, "CL_ACCUM", revRegEntry);
            await Ledger.SignAndSubmitRequestAsync(pool, wallet, myDid, revRegEntryRequest);

            entitiesPosted = true;

	    }
    }
}
