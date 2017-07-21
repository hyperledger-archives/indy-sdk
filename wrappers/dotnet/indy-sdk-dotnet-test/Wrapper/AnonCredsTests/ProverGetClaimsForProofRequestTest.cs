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
    public class ProverGetClaimsForProofRequestTest : AnonCredsIntegrationTestBase
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
        public void TestProverGetClaimsForProofRequestWorksForRevealedAttribute()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;                

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["attrs"]["attr1_uuid"];
            Assert.AreEqual(claimsForAttribute1.Count, 1);
        }

        [TestMethod]
        public void TestProverGetClaimsForProofRequestWorksForNotFoundAttribute()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"attribute\"}},\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["attrs"]["attr1_uuid"];
            Assert.AreEqual(claimsForAttribute1.Count, 0);
        }

        [TestMethod]
        public void TestProverGetClaimsForProofRequestWorksForSatisfyPredicate()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"attribute\"}},\n" +
                "              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                "             }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["predicates"]["predicate1_uuid"];
            Assert.AreEqual(claimsForAttribute1.Count, 1);
        }

        [TestMethod]
        public void TestProverGetClaimsForProofRequestWorksForNotSatisfyPredicate()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{},\n" +
                "              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":58}}\n" +
                "             }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["predicates"]["predicate1_uuid"];
            Assert.AreEqual(claimsForAttribute1.Count, 0);
        }

        [TestMethod]
        public void TestProverGetClaimsForProofRequestWorksForMultipleAttributesAndPredicates()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "               \"name\":\"proof_req_1\",\n" +
                "               \"version\":\"0.1\",\n" +
                "               \"requested_attrs\":{\n" +
                "                     \"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"},\n" +
                "                     \"attr2_uuid\":{\"schema_seq_no\":1, \"name\":\"sex\"}\n" +
                "               },\n" +
                "               \"requested_predicates\":{\n" +
                "                     \"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18},\n" +
                "                     \"predicate2_uuid\":{\"attr_name\":\"height\",\"p_type\":\"GE\",\"value\":160}\n" +
                "               }}";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["attrs"]["attr1_uuid"];
            Assert.AreEqual(claimsForAttribute1.Count, 1);

            var claimsForAttribute2 = (JArray)claims["attrs"]["attr2_uuid"];
            Assert.AreEqual(claimsForAttribute2.Count, 1);

            var claimsForPredicate1 = (JArray)claims["predicates"]["predicate1_uuid"];
            Assert.AreEqual(claimsForPredicate1.Count, 1);

            var claimsForPredicate2 = (JArray)claims["predicates"]["predicate2_uuid"];
            Assert.AreEqual(claimsForPredicate2.Count, 1);
        }

        [TestMethod]
        public void TestProverGetClaimsForProofRequestWorksForEmptyRequest()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{},\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            Assert.AreEqual(((JObject)claims["attrs"]).Count, 0);
            Assert.AreEqual(((JObject)claims["predicates"]).Count, 0);
        }

        [TestMethod]
        public void TestProverGetClaimsForProofRequestWorksForRevealedAttributeWithOtherSchema()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":2, \"name\":\"name\"}},\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["attrs"]["attr1_uuid"];
            Assert.AreEqual(claimsForAttribute1.Count, 0);
        }

        [TestMethod]
        public void TestProverGetClaimsForProofRequestWorksForRevealedAttributeBySpecificIssuer()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{\"attr1_uuid\":{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"name\":\"name\"}},\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["attrs"]["attr1_uuid"];
            Assert.AreEqual(claimsForAttribute1.Count, 1);
        }

        [TestMethod]
        public void TestProverGetClaimsForProofRequestWorksForSatisfyPredicateByIssuerAndSchema()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{},\n" +
                "              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\"}}\n" +
                "             }";

            var claimsJson = AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest).Result;

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["predicates"]["predicate1_uuid"];
            Assert.AreEqual(claimsForAttribute1.Count, 1);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForInvalidProofRequest()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForInvalidPredicateType()
        {
            InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{},\n" +
                "              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"LE\",\"value\":18}}\n" +
                "             }";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}
