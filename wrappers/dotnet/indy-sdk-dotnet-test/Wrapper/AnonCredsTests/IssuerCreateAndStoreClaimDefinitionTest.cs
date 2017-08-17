﻿using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;


namespace Indy.Sdk.Dotnet.Test.Wrapper.AnonCredsTests
{
    [TestClass]
    public class IssuerCreateAndStoreClaimDefinitionTest : AnonCredsIntegrationTestBase
    {
        private  Wallet _wallet;
        private  string _walletName = "createAndStoreClaimDefWallet";
        private new string _issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
        private  string _gvtSchemaJson = "{\n" +
                "                    \"seqNo\":1,\n" +
                "                    \"data\": {\n" +
                "                        \"name\":\"gvt\",\n" +
                "                        \"version\":\"1.0\",\n" +
                "                        \"keys\":[\"age\",\"sex\",\"height\",\"name\"]\n" +
                "                    }\n" +
                "                 }";

        [TestInitialize]
        public async Task CreateWallet()
        {
            await Wallet.CreateWalletAsync("default", _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            if(_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);            
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorks()
        {
            var claimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, _issuerDid, _gvtSchemaJson, null, false);
            Assert.IsNotNull(claimDef);

            var claimDefObject = JObject.Parse(claimDef);
            var primary = claimDefObject["data"]["primary"];

            Assert.AreEqual(((JObject)primary["r"]).Count, 4);
            Assert.IsTrue(primary.Value<string>("n").Length > 0);
            Assert.IsTrue(primary.Value<string>("s").Length > 0);
            Assert.IsTrue(primary.Value<string>("z").Length > 0);
            Assert.IsTrue(primary.Value<string>("rms").Length > 0);
            Assert.IsTrue(primary.Value<string>("rctxt").Length > 0);
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForInvalidSchemaJson()
        {
            var schema = "{\"seqNo\":1, \"name\":\"name\",\"version\":\"1.0\", \"keys\":[\"name\"]}";
           
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, _issuerDid, schema, null, false)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForEmptyKeys()
        {
            var schema = "{\n" +
                "                    \"seqNo\":1,\n" +
                "                    \"data\": {\n" +
                "                        \"name\":\"gvt\",\n" +
                "                        \"version\":\"1.0\",\n" +
                "                        \"keys\":[]\n" +
                "                    }\n" +
                "                 }";

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, _issuerDid, schema, null, false)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForCorrectCryptoType()
        {
            var claimDef = await AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, _issuerDid, _gvtSchemaJson, "CL", false);
            Assert.IsNotNull(claimDef);

            var claimDefObject = JObject.Parse(claimDef);
            var primary = claimDefObject["data"]["primary"];

            Assert.AreEqual(((JObject)primary["r"]).Count, 4);
            Assert.IsTrue(primary.Value<string>("n").Length > 0);
            Assert.IsTrue(primary.Value<string>("s").Length > 0);
            Assert.IsTrue(primary.Value<string>("z").Length > 0);
            Assert.IsTrue(primary.Value<string>("rms").Length > 0);
            Assert.IsTrue(primary.Value<string>("rctxt").Length > 0);
        }

        [TestMethod]
        public async Task TestIssuerCreateAndStoreClaimDefWorksForInvalidCryptoType()
        {
            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                AnonCreds.IssuerCreateAndStoreClaimDefAsync(_wallet, _issuerDid, _gvtSchemaJson, "type", false)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }
    }
}
