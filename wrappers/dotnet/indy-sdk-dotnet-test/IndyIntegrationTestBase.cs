using Hyperledger.Indy.PoolApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test
{
    public abstract class IndyIntegrationTestBase
    {
        protected const string TRUSTEE_SEED = "000000000000000000000000Trustee1";
        protected const string MY1_SEED = "00000000000000000000000000000My1";
        protected const string MY2_SEED = "00000000000000000000000000000My2";
        protected const string DID1 = "8wZcEriaNLNKtteJvx7f8i";
        protected const string IDENTITY_JSON_TEMPLATE = "{{\"did\":\"{0}\",\"verkey\":\"{1}\"}}";
        protected readonly static byte[] MESSAGE = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");
        protected const string SCHEMA_DATA = "{\"name\":\"gvt2\",\"version\":\"3.0\",\"attr_names\": [\"name\", \"male\"]}";
        protected const string POOL = "Pool1";
        protected const string WALLET = "Wallet1";
        protected const string TYPE = "default";
        protected readonly static string TRUSTEE_IDENTITY_JSON = string.Format("{{\"seed\":\"{0}\"}}", TRUSTEE_SEED);
        protected const string VERKEY = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
        protected const string VERKEY_FOR_MY1_SEED = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";
        protected const string VERKEY_FOR_MY2_SEED = "kqa2HyagzfMAq42H5f9u3UMwnSBPQx2QfrSyXbUPxMn";
        protected const string DID_FOR_MY1_SEED = "VsKV7grR1BUE29mG2Fm2kX";
        protected const string DID_FOR_MY2_SEED = "2PRyVHmkXQnQzJQKxHxnXC";
        protected const string METADATA = "some metadata";
        protected const string ENDPOINT = "127.0.0.1:9700";
        protected const string INVALID_VERKEY = "CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW";
        protected const string CRYPTO_TYPE = "ed25519";

        protected static string MY1_IDENTITY_JSON = string.Format("{{\"seed\":\"{0}\"}}", MY1_SEED);

        protected HashSet<Pool> openedPools = new HashSet<Pool>();

        [TestInitialize]
        public async Task SetUp()
        {
            await InitHelper.InitAsync();
            StorageUtils.CleanupStorage();
        }

        [TestCleanup]
        public async Task TearDown()
        {
            foreach (var pool in openedPools)
            {
                if (pool != null)
                    await pool.CloseAsync();
            }

            openedPools.Clear();
            StorageUtils.CleanupStorage();
        }
    }
}
