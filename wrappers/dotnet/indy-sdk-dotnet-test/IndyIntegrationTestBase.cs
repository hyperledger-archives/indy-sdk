using Hyperledger.Indy.PoolApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
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
        protected const string VERKEY = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
        protected const string VERKEY_MY1 = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";
        protected const string VERKEY_MY2 = "kqa2HyagzfMAq42H5f9u3UMwnSBPQx2QfrSyXbUPxMn";
        protected const string VERKEY_TRUSTEE = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";
        protected const string INVALID_VERKEY = "CnEDk___MnmiHXEV1WFgbV___eYnPqs___TdcZaNhFVW";
        protected const string DID1 = "8wZcEriaNLNKtteJvx7f8i";
        protected const string DID_MY1 = "VsKV7grR1BUE29mG2Fm2kX";
        protected const string DID_MY2 = "2PRyVHmkXQnQzJQKxHxnXC";
        protected const string DID_TRUSTEE = "V4SGRU86Z58d6TV7PBUe6f";
        protected const string INVALID_DID = "invalid_base58string";
        protected const string IDENTITY_JSON_TEMPLATE = "{{\"did\":\"{0}\",\"verkey\":\"{1}\"}}";
        protected readonly static byte[] MESSAGE = Encoding.UTF8.GetBytes("{\"reqId\":1496822211362017764}");
        protected const string SCHEMA_DATA = "{\"name\":\"gvt2\",\"version\":\"3.0\",\"attr_names\": [\"name\", \"male\"]}";
        protected const string POOL = "Pool1";
        protected const string WALLET = "Wallet1";
        protected const string TYPE = "default";
        protected const string METADATA = "some metadata";
        protected const string ENDPOINT = "127.0.0.1:9700";
        protected const string CRYPTO_TYPE = "ed25519";
        protected readonly static byte[] SIGNATURE = (byte[])(Array)new sbyte[] { 20, -65, 100, -43, 101, 12, -59, -58, -53, 49, 89, -36, -51, -64, -32, -35, 97, 77, -36, -66, 90, 60, -114, 23, 16, -16, -67, -127, 45, -108, -11, 8, 102, 95, 95, -7, 100, 89, 41, -29, -43, 25, 100, 1, -24, -68, -11, -21, -70, 21, 52, -80, -20, 11, 99, 70, -101, -97, 89, -41, -59, -17, -118, 5 };
        protected readonly static byte[] ENCRYPTED_MESSAGE = (byte[])(Array)new sbyte[] { -105, 30, 89, 75, 76, 28, -59, -45, 105, -46, 20, 124, -85, -13, 109, 29, -88, -82, -8, -6, -50, -84, -53, -48, -49, 56, 124, 114, 82, 126, 74, 99, -72, -78, -117, 96, 60, 119, 50, -40, 121, 21, 57, -68, 89 };
        protected readonly static byte[] NONCE = (byte[])(Array)new sbyte[] { -14, 102, -41, -57, 1, 4, 75, -46, -91, 87, 14, 41, -39, 48, 42, -126, -121, 84, -58, 59, -27, 51, -32, -23 };


        protected readonly static string TRUSTEE_IDENTITY_JSON = string.Format("{{\"seed\":\"{0}\"}}", TRUSTEE_SEED);
        protected readonly static string MY1_IDENTITY_JSON = string.Format("{{\"seed\":\"{0}\"}}", MY1_SEED);
        protected readonly static string MY1_IDENTITY_KEY_JSON = string.Format("{{\"seed\":\"{0}\"}}", MY1_SEED);

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
