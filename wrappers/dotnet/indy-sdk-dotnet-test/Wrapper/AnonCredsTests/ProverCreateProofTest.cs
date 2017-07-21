using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.Agent;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AnonCredsTests
{
    [TestClass]
    public class ProverCreateProofTest : AnonCredsIntegrationTestBase
    {
        [ClassCleanup]
        public static void CloseCommonWallet()
        {
            try
            {
                _commonWallet.CloseAsync().Wait();
            }
            catch (Exception)
            { }

        }

        [TestMethod]
        public void TestProverCreateProofWorks()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                    "                                \"name\":\"proof_req_1\",\n" +
                    "                                \"version\":\"0.1\",\n" +
                    "                                \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
                    "                                \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                    "                              }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;                

            var claims = JObject.Parse(claimsJson);

            var claimForAttribute = claims["attrs"]["attr1_uuid"][0];

            var claimUuid = claimForAttribute.Value<string>("claim_uuid");

            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{1}\"}}\n" +
                    "                                        }}", claimUuid, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDef);
            var revocRegsJson = "{}";

            var proofJson = AnonCreds.ProverCreateProofAsync(_commonWallet, proofRequest, requestedClaimsJson, schemasJson,
                    _masterSecretName, claimDefsJson, revocRegsJson).Result;
            Assert.IsNotNull(proofJson);
        }


        [TestMethod]
        public async Task TestProverCreateProofWorksForUsingNotSatisfyClaim()
        {
            InitCommonWallet();

            var claimsJson = AnonCreds.ProverGetClaimsAsync(_commonWallet, "{}").Result;

            var claims = JArray.Parse(claimsJson);

            var claimUuid = claims[0].Value<string>("claim_uuid");

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                    "               \"name\":\"proof_req_1\",\n" +
                    "               \"version\":\"0.1\",\n" +
                    "               \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"some_attr\"}},\n" +
                    "               \"requested_predicates\":{}\n" +
                    "              }";

            var requestedClaimsJson = string.Format("{{\"self_attested_attributes\":{{}},\n" +
                    "                                    \"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}},\n" +
                    "                                    \"requested_predicates\":{{}}\n" +
                    "                                   }}", claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDef);
            var revocRegsJson = "{}";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateProofAsync(_commonWallet, proofRequest, requestedClaimsJson, schemasJson,
                    _masterSecretName, claimDefsJson, revocRegsJson)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateProofWorksForInvalidMasterSecret()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "                                \"name\":\"proof_req_1\",\n" +
                "                                \"version\":\"0.1\",\n" +
                "                                \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
                "                                \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                "                              }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            var claimForAttribute = claims["attrs"]["attr1_uuid"][0];

            var claimUuid = claimForAttribute.Value<string>("claim_uuid");

            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{1}\"}}\n" +
                    "                                        }}", claimUuid, claimUuid);

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDef);
            var revocRegsJson = "{}";

            
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateProofAsync(_commonWallet, proofRequest, requestedClaimsJson, schemasJson, "wrong_master_secret", claimDefsJson, revocRegsJson)

            );

            Assert.AreEqual(ErrorCode.WalletNotFoundError, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateProofWorksForInvalidSchemas()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "                                \"name\":\"proof_req_1\",\n" +
                "                                \"version\":\"0.1\",\n" +
                "                                \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
                "                                \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                "                              }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            var claimForAttribute = claims["attrs"]["attr1_uuid"][0];

            var claimUuid = claimForAttribute.Value<string>("claim_uuid");

            var requestedClaimsJson = string.Format("{{\n" +
                    "                                          \"self_attested_attributes\":{{}},\n" +
                    "                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{0}\", true]}},\n" +
                    "                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{1}\"}}\n" +
                    "                                        }}", claimUuid, claimUuid);

            var schemasJson = "{}";
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDef);
            var revocRegsJson = "{}";


            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateProofAsync(_commonWallet, proofRequest, requestedClaimsJson, schemasJson, _masterSecretName, claimDefsJson, revocRegsJson)

            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverCreateProofWorksForInvalidRequestedClaimsJson()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "                                \"name\":\"proof_req_1\",\n" +
                "                                \"version\":\"0.1\",\n" +
                "                                \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
                "                                \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                "                              }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            var claimForAttribute = claims["attrs"]["attr1_uuid"][0];

            var claimUuid = claimForAttribute.Value<string>("claim_uuid");

            String requestedClaimsJson = "{\"self_attested_attributes\":{},\n" +
                "                      \"requested_predicates\":{}\n" +
                "                    }";

            var schemasJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _schema);
            var claimDefsJson = string.Format("{{\"{0}\":{1}}}", claimUuid, _claimDef);
            var revocRegsJson = "{}";


            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverCreateProofAsync(_commonWallet, proofRequest, requestedClaimsJson, schemasJson, "wrong_master_secret", claimDefsJson, revocRegsJson)

            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}
