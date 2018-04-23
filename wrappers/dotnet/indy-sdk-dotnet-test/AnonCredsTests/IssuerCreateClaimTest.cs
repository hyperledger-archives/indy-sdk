using Hyperledger.Indy.AnonCredsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class IssuerCreateClaimTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimWorks()
        {
            await InitCommonWallet();

            var claimRequest = string.Format(claimRequestTemplate, issuerDid, 1);

            var claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = await AnonCreds.IssuerCreateCredentialAsync(commonWallet, claimRequest, claim, -1);
            Assert.IsNotNull(createClaimResult);
            var claimJson = createClaimResult.ClaimJson;

            var claimObj = JObject.Parse(claimJson);
            var primaryClaim = claimObj["signature"]["primary_claim"];

            Assert.IsTrue(primaryClaim.Value<string>("a").Length > 0);
            Assert.IsTrue(primaryClaim.Value<string>("m2").Length > 0);
            Assert.IsTrue(primaryClaim.Value<string>("e").Length > 0);
            Assert.IsTrue(primaryClaim.Value<string>("v").Length > 0);
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimWorksForClaimDoesNotCorrespondToClaimRequest()
        {
            await InitCommonWallet();

            var claimRequest = string.Format(claimRequestTemplate, issuerDid, 1);

            var claim = "{\"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
                    "        \"period\":[\"8\",\"8\"]\n" +
                    "       }";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.IssuerCreateCredentialAsync(commonWallet, claimRequest, claim, -1)
            );
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimReqWorksForInvalidClaim()
        {
            await InitCommonWallet();

            String claimRequest = string.Format(claimRequestTemplate, issuerDid, 1);

            String claim = "{\"sex\":\"male\",\n" +
                    "        \"name\":\"Alex\",\n" +
                    "        \"height\":\"175\",\n" +
                    "        \"age\":\"28\"" +
                    "       }";

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                AnonCreds.IssuerCreateCredentialAsync(commonWallet, claimRequest, claim, -1)
            );
        }
    }
}
