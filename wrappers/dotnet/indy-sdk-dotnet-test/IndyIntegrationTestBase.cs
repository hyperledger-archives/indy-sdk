using Hyperledger.Indy.PoolApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test
{
    public abstract class IndyIntegrationTestBase
    {
        protected static string TRUSTEE_SEED = "000000000000000000000000Trustee1";
        protected static string MY1_SEED = "00000000000000000000000000000My1";
        protected static string DID1 = "8wZcEriaNLNKtteJvx7f8i";
        protected static string IDENTITY_JSON_TEMPLATE = "{\"did\":\"%s\",\"verkey\":\"%s\"}";
        protected static byte[] MESSAGE = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");
        protected static string SCHEMA_DATA = "{\"name\":\"gvt2\",\"version\":\"3.0\",\"attr_names\": [\"name\", \"male\"]}";
        protected static string POOL = "Pool1";
        protected static string WALLET = "Wallet1";
        protected static string TYPE = "default";
        protected static string TRUSTEE_IDENTITY_JSON = string.Format("{{\"seed\":\"{0}\"}}", TRUSTEE_SEED);

        protected HashSet<Pool> _openedPools = new HashSet<Pool>();

        [TestInitialize]
        public async Task SetUp()
        {
            await InitHelper.InitAsync();
            StorageUtils.CleanupStorage();
        }

        [TestCleanup]
        public async Task TearDown()
        {
            foreach (var pool in _openedPools)
            {
                if (pool != null)
                    await pool.CloseAsync();
            }

            _openedPools.Clear();
            StorageUtils.CleanupStorage();
        }
    }
}
