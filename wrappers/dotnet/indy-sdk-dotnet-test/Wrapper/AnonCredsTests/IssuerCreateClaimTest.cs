﻿using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.Agent;
using static Indy.Sdk.Dotnet.Wrapper.AgentObservers;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AnonCredsTests
{
    [TestClass]
    public class IssuerCreateClaimTest : AnonCredsIntegrationTestBase
    {
        [ClassCleanup]
        public static void CloseCommonWallet()
        {
            _commonWallet.CloseAsync().Wait();

        }

        [TestMethod]
        public void TestIssuerCreateAndStoreClaimDefWorks()
        {
            InitCommonWallet();

            var claimRequest = string.Format(_claimRequestTemplate, _issuerDid, 1);

            var claim = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\n" +
                    "               \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\n" +
                    "               \"height\":[\"175\",\"175\"],\n" +
                    "               \"age\":[\"28\",\"28\"]\n" +
                    "        }";

            var createClaimResult = AnonCreds.IssuerCreateClaimAsync(_commonWallet, claimRequest, claim, -1, -1).Result;
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
        public async Task TestIssuerCreateClaimWorksForClaimDoesNotCorrespondToClaimRequest()
        {
            InitCommonWallet();

            var claimRequest = string.Format(_claimRequestTemplate, _issuerDid, 1);

            var claim = "{\"status\":[\"partial\",\"51792877103171595686471452153480627530895\"],\n" +
                    "        \"period\":[\"8\",\"8\"]\n" +
                    "       }";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.IssuerCreateClaimAsync(_commonWallet, claimRequest, claim, -1, -1)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimReqWorksForInvalidClaim()
        {
            InitCommonWallet();

            String claimRequest = string.Format(_claimRequestTemplate, _issuerDid, 1);

            String claim = "{\"sex\":\"male\",\n" +
                    "        \"name\":\"Alex\",\n" +
                    "        \"height\":\"175\",\n" +
                    "        \"age\":\"28\"" +
                    "       }";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.IssuerCreateClaimAsync(_commonWallet, claimRequest, claim, -1, -1)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}
