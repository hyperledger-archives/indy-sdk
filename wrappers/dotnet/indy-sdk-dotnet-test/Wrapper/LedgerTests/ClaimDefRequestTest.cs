using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.LedgerTests
{
    [TestClass]
    public class ClaimDefRequestTest : IndyIntegrationTestBase
    {
        private Pool _pool;
        private Wallet _wallet;
        private string _walletName = "ledgerWallet";

        [TestInitialize]
        public void OpenPool()
        {
            string poolName = PoolUtils.CreatePoolLedgerConfig();
            _pool = Pool.OpenPoolLedgerAsync(poolName, null).Result;

            Wallet.CreateWalletAsync(poolName, _walletName, "default", null, null).Wait();
            _wallet = Wallet.OpenWalletAsync(_walletName, null, null).Result;
        }

        [TestCleanup]
        public void ClosePool()
        {
            _pool.CloseAsync().Wait();
            _wallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(_walletName, null).Wait();
        }

        [TestMethod]
        public void TestBuildClaimDefRequestWorks()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var signature_type = "CL";
            var schema_seq_no = 1;
            var data = "{\"primary\":{\"n\":\"1\",\"s\":\"2\",\"rms\":\"3\",\"r\":{\"name\":\"1\"},\"rctxt\":\"1\",\"z\":\"1\"}}";

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"ref\":{1}," +
                    "\"data\":\"{2}\"," +
                    "\"type\":\"102\"," +
                    "\"signature_type\":\"{3}\"" +
                    "}}", identifier, schema_seq_no, data, signature_type);

            var claimDefRequest = Ledger.BuildClaimDefTxnAsync(identifier, schema_seq_no, signature_type, data).Result;


            Assert.IsTrue(claimDefRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public void TestBuildGetClaimDefRequestWorks()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var origin = "Th7MpTaRZVRYnPiabds81Y";
            var signature_type = "CL";
            var reference = 1;

            var expectedResult = string.Format("\"identifier\":\"{0}\"," +
                    "\"operation\":{{" +
                    "\"type\":\"108\"," +
                    "\"ref\":{1}," +
                    "\"signature_type\":\"{2}\"," +
                    "\"origin\":\"{3}\"" +
                    "}}", identifier, reference, signature_type, origin);

            var getClaimDefRequest = Ledger.BuildGetClaimDefTxnAsync(identifier, reference, signature_type, origin).Result;


            Assert.IsTrue(getClaimDefRequest.Replace("\\", "").Contains(expectedResult));
        }

        [TestMethod]
        public async Task TestBuildClaimDefRequestWorksForInvalidJson()
        {
            var identifier = "Th7MpTaRZVRYnPiabds81Y";
            var signature_type = "CL";
            var schema_seq_no = 1;
            var data = "{\"primary\":{\"n\":\"1\",\"s\":\"2\",\"rms\":\"3\",\"r\":{\"name\":\"1\"}}}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.BuildClaimDefTxnAsync(identifier, schema_seq_no, signature_type, data)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod] //Seems to fail unreliably; schema members do not maintain their order.  Test problem or underlying SDK isssue?
        public void TestClaimDefRequestWorks()
        {
            var trusteeJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, trusteeJson).Result;
            var trusteeDid = trusteeDidResult.Did;

            var myJson = "{}";

            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, myJson).Result;
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest).Wait();

            var schemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\",\"keys\": [\"name\", \"male\"]}";

            var schemaRequest = Ledger.BuildSchemaRequestAsync(myDid, schemaData).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, schemaRequest).Wait();

            var getSchemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\"}";
            var getSchemaRequest = Ledger.BuildGetSchemaRequestAsync(myDid, myDid, getSchemaData).Result;
            var getSchemaResponse = Ledger.SubmitRequestAsync(_pool, getSchemaRequest).Result;

            var schemaObj = JObject.Parse(getSchemaResponse);

            var schemaSeqNo = (int)schemaObj["result"]["seqNo"];
            var schemaJson = string.Format("{{\"seqNo\":{0},\"data\":{1}}}", schemaSeqNo, schemaData);

            var claimDef = AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, myDid, schemaJson, null, false).Result;

            var claimDefObj = JObject.Parse(claimDef);

            var claimDefJson = claimDefObj["data"];
            var signatureType = (string)claimDefObj["signature_type"];

            var claimDefRequest = Ledger.BuildClaimDefTxnAsync(myDid, schemaSeqNo, signatureType, claimDefObj["data"].ToString()).Result;
            var claimDefResponse = Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, claimDefRequest).Result;

            var getClaimDefRequest = Ledger.BuildGetClaimDefTxnAsync(myDid, schemaSeqNo, signatureType, claimDefObj["origin"].ToString()).Result;
            var getClaimDefResponse = Ledger.SubmitRequestAsync(_pool, getClaimDefRequest).Result;

            var getClaimDefResponseObj = JObject.Parse(getClaimDefResponse);

            var expectedClaimDef = claimDefObj["data"]["primary"];
            var actualClaimDef = getClaimDefResponseObj["result"]["data"]["primary"];

            Assert.AreEqual(expectedClaimDef["n"], actualClaimDef["n"]);
            Assert.AreEqual(expectedClaimDef["rms"], actualClaimDef["rms"]);
            Assert.AreEqual(expectedClaimDef["rctxt"], actualClaimDef["rctxt"]);
            Assert.AreEqual(expectedClaimDef["z"], actualClaimDef["z"]);
            Assert.AreEqual(expectedClaimDef["n"], actualClaimDef["n"]);
            Assert.AreEqual(expectedClaimDef["r"].ToString(), actualClaimDef["r"].ToString());
        }

        [TestMethod]
        public async Task TestClaimDefRequestWorksWithoutSignature()
        {
            var trusteeJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, trusteeJson).Result;
            var trusteeDid = trusteeDidResult.Did;

            var myJson = "{}";

            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, myJson).Result;
            var myDid = myDidResult.Did;
            var myVerkey = myDidResult.VerKey;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, myDid, myVerkey, null, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, _wallet, trusteeDid, nymRequest).Wait();

            var schemaData = "{\"name\":\"gvt2\",\"version\":\"2.0\",\"keys\": [\"name\", \"male\"]}";

            var schemaRequest = Ledger.BuildSchemaRequestAsync(myDid, schemaData).Result;
            var schemaResponse = Ledger.SignAndSubmitRequestAsync(_pool, _wallet, myDid, schemaRequest).Result;

            var schemaObj = JObject.Parse(schemaResponse);

            int schemaSeqNo = (int)schemaObj["result"]["seqNo"];
            var schemaJson = string.Format("{{\"seqNo\":{0},\"data\":{1}}}", schemaSeqNo, schemaData);

            var claimDef = AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, myDid, schemaJson, null, false).Result;

            var claimDefObj = JObject.Parse(claimDef);

            var claimDefJson = claimDefObj["data"].ToString();

            var claimDefRequest = Ledger.BuildClaimDefTxnAsync(myDid, schemaSeqNo, (string)claimDefObj["signature_type"], claimDefJson).Result;

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Ledger.SubmitRequestAsync(_pool, claimDefRequest)
            );

            Assert.AreEqual(ErrorCode.LedgerInvalidTransaction, ex.ErrorCode);

        }
    }
}
