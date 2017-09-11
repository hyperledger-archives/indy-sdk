using Hyperledger.Indy.AnonCredsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System.Threading.Tasks;


namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverGetClaimsForProofRequestTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForRevealedAttribute()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"name\"}},\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);                

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["attrs"]["attr1_uuid"];
            Assert.AreEqual(1, claimsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForNotFoundAttribute()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"attribute\"}},\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["attrs"]["attr1_uuid"];
            Assert.AreEqual(0, claimsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForSatisfyPredicate()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":1, \"name\":\"attribute\"}},\n" +
                "              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\n" +
                "             }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["predicates"]["predicate1_uuid"];
            Assert.AreEqual(1, claimsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForNotSatisfyPredicate()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{},\n" +
                "              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":58}}\n" +
                "             }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["predicates"]["predicate1_uuid"];
            Assert.AreEqual(0, claimsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForMultipleAttributesAndPredicates()
        {
            await InitCommonWallet();

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

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["attrs"]["attr1_uuid"];
            Assert.AreEqual(1, claimsForAttribute1.Count);

            var claimsForAttribute2 = (JArray)claims["attrs"]["attr2_uuid"];
            Assert.AreEqual(1, claimsForAttribute2.Count);

            var claimsForPredicate1 = (JArray)claims["predicates"]["predicate1_uuid"];
            Assert.AreEqual(1, claimsForPredicate1.Count);

            var claimsForPredicate2 = (JArray)claims["predicates"]["predicate2_uuid"];
            Assert.AreEqual(1, claimsForPredicate2.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForEmptyRequest()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{},\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            Assert.AreEqual(0, ((JObject)claims["attrs"]).Count);
            Assert.AreEqual(0, ((JObject)claims["predicates"]).Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForRevealedAttributeWithOtherSchema()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{\"attr1_uuid\":{\"schema_seq_no\":2, \"name\":\"name\"}},\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["attrs"]["attr1_uuid"];
            Assert.AreEqual(0, claimsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForRevealedAttributeBySpecificIssuer()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{\"attr1_uuid\":{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"name\":\"name\"}},\n" +
                "              \"requested_predicates\":{}\n" +
                "             }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["attrs"]["attr1_uuid"];
            Assert.AreEqual(1, claimsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForSatisfyPredicateByIssuerAndSchema()
        {
            await InitCommonWallet();

            var proofRequest = "{\"nonce\":\"123432421212\",\n" +
                "              \"name\":\"proof_req_1\",\n" +
                "              \"version\":\"0.1\",\n" +
                "              \"requested_attrs\":{},\n" +
                "              \"requested_predicates\":{\"predicate1_uuid\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\"}}\n" +
                "             }";

            var claimsJson = await AnonCreds.ProverGetClaimsForProofReqAsync(_commonWallet, proofRequest);

            var claims = JObject.Parse(claimsJson);

            var claimsForAttribute1 = (JArray)claims["predicates"]["predicate1_uuid"];
            Assert.AreEqual(1, claimsForAttribute1.Count);
        }

        [TestMethod]
        public async Task TestProverGetClaimsForProofRequestWorksForInvalidProofRequest()
        {
            await InitCommonWallet();

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
            await InitCommonWallet();

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
