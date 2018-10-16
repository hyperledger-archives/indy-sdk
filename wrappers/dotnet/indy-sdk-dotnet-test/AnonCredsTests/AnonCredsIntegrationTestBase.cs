using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{

    public abstract class AnonCredsIntegrationTestBase 
    {
        private static bool _walletOpened = false;

        protected static Wallet wallet;
        protected static string gvtSchemaId;
        protected static string gvtSchema;
        protected static string xyzSchemaId;
        protected static string xyzSchema;
        protected static string issuer1gvtCredDefId;
        protected static string issuer1gvtCredDef;
        protected static string issuer1xyzCredDef;
        protected static string issuer1GvtCredOffer;
        protected static string issuer2GvtCredOffer;
        protected static string issuer1GvtCredReq;
        protected static string issuer1GvtCredReqMetadata;
        protected static string CREDENTIALS = "{\"key\":\"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY\", \"key_derivation_method\":\"RAW\"}";
        protected static string masterSecretId = "master_secret_name";
        protected static string issuerDid = "NcYxiDXkpYi6ov5FcYDi1e";
        protected static string proverDid = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
        protected static string defaultCredentialDefinitionConfig = "{\"support_revocation\":false}";
        protected static string tag = "tag1";
        protected static string gvtSchemaName = "gvt";
        protected static string schemaVersion = "1.0";
        protected static string gvtSchemaAttributes = "[\"name\", \"age\", \"sex\", \"height\"]";
        protected static string credentialId1 = "id1";
        protected static string credentialId2 = "id2";
        // note that encoding is not standardized by Indy except that 32-bit integers are encoded as themselves. IS-786
        protected static string gvtCredentialValuesJson = JObject.Parse("{\n" +
                "               \"sex\":{\"raw\":\"male\",\"encoded\":\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"},\n" +
                "               \"name\":{\"raw\":\"Alex\",\"encoded\":\"1139481716457488690172217916278103335\"},\n" +
                "               \"height\":{\"raw\":\"175\",\"encoded\":\"175\"},\n" +
                "               \"age\":{\"raw\":\"28\",\"encoded\":\"28\"}\n" +
                "        }").ToString();
        protected static string xyzCredentialValuesJson = JObject.Parse("{\n" +
                "               \"status\":{\"raw\":\"partial\",\"encoded\":\"51792877103171595686471452153480627530895\"},\n" +
                "               \"period\":{\"raw\":\"8\",\"encoded\":\"8\"}\n" +
                "        }").ToString();
        protected static string proofRequest = JObject.Parse("{\n" +
                "                   \"nonce\":\"123432421212\",\n" +
                "                   \"name\":\"proof_req_1\",\n" +
                "                   \"version\":\"0.1\", " +
                "                   \"requested_attributes\":{" +
                "                          \"attr1_referent\":{\"name\":\"name\"}" +
                "                    },\n" +
                "                    \"requested_predicates\":{" +
                "                          \"predicate1_referent\":{\"name\":\"age\",\"p_type\":\">=\",\"p_value\":18}" +
                "                    }" +
                "               }").ToString();




        [TestInitialize]
        public async Task SetUp() 
        {
            await InitHelper.InitAsync();
            await InitCommonWallet();
        }

        protected async Task InitCommonWallet()
        {
            if (_walletOpened)
                return;

            var walletConfig = JsonConvert.SerializeObject(new { id = Guid.NewGuid() });

            await Wallet.CreateWalletAsync(walletConfig, CREDENTIALS);
            wallet = await Wallet.OpenWalletAsync(walletConfig, CREDENTIALS);

            var createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid, gvtSchemaName, schemaVersion, gvtSchemaAttributes);
            gvtSchemaId = createSchemaResult.SchemaId;
            gvtSchema = createSchemaResult.SchemaJson;

            var xyzSchemaAttributes = "[\"status\", \"period\"]";
            var xyzSchemaName = "xyz";
            createSchemaResult = await AnonCreds.IssuerCreateSchemaAsync(issuerDid, xyzSchemaName, schemaVersion, xyzSchemaAttributes);
            xyzSchemaId = createSchemaResult.SchemaId;
            xyzSchema = createSchemaResult.SchemaJson;

            //Issue GVT issuer1GvtCredential by Issuer1
            var issuer1CreateGvtCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(wallet, issuerDid, gvtSchema, tag, null, defaultCredentialDefinitionConfig);
            issuer1gvtCredDefId = issuer1CreateGvtCredDefResult.CredDefId;
            issuer1gvtCredDef = issuer1CreateGvtCredDefResult.CredDefJson;

            //Issue XYZ issuer1GvtCredential by Issuer1
            var issuer1CreateXyzCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(wallet, issuerDid, xyzSchema, tag, null, defaultCredentialDefinitionConfig);
            var issuer1xyzCredDefId = issuer1CreateXyzCredDefResult.CredDefId;
            issuer1xyzCredDef = issuer1CreateXyzCredDefResult.CredDefJson;

            //Issue GVT issuer1GvtCredential by Issuer2
            var issuerDid2 = "VsKV7grR1BUE29mG2Fm2kX";
            var issuer2CreateGvtCredDefResult = await AnonCreds.IssuerCreateAndStoreCredentialDefAsync(wallet, issuerDid2, gvtSchema, tag, null, defaultCredentialDefinitionConfig);
            var issuer2gvtCredDefId = issuer2CreateGvtCredDefResult.CredDefId;
            var issuer2gvtCredDef = issuer2CreateGvtCredDefResult.CredDefJson;

            issuer1GvtCredOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(wallet, issuer1gvtCredDefId);
            var issuer1XyzCredOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(wallet, issuer1xyzCredDefId);
            issuer2GvtCredOffer = await AnonCreds.IssuerCreateCredentialOfferAsync(wallet, issuer2gvtCredDefId);

            await AnonCreds.ProverCreateMasterSecretAsync(wallet, masterSecretId);

            var createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(wallet, proverDid, issuer1GvtCredOffer, issuer1gvtCredDef, masterSecretId);
            issuer1GvtCredReq = createCredReqResult.CredentialRequestJson;
            issuer1GvtCredReqMetadata = createCredReqResult.CredentialRequestMetadataJson;

            var createCredResult = await AnonCreds.IssuerCreateCredentialAsync(wallet, issuer1GvtCredOffer, issuer1GvtCredReq, gvtCredentialValuesJson, null, null);
            var issuer1GvtCredential = createCredResult.CredentialJson;

            await AnonCreds.ProverStoreCredentialAsync(wallet, credentialId1, issuer1GvtCredReqMetadata, issuer1GvtCredential, issuer1gvtCredDef, null);

            createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(wallet, proverDid, issuer1XyzCredOffer, issuer1xyzCredDef, masterSecretId);
            var issuer1XyzCredReq = createCredReqResult.CredentialRequestJson;
            var issuer1XyzCredReqMetadata = createCredReqResult.CredentialRequestMetadataJson;

            createCredResult = await AnonCreds.IssuerCreateCredentialAsync(wallet, issuer1XyzCredOffer, issuer1XyzCredReq, xyzCredentialValuesJson, null, null);
            var issuer1XyzCredential = createCredResult.CredentialJson;

            await AnonCreds.ProverStoreCredentialAsync(wallet, credentialId2, issuer1XyzCredReqMetadata, issuer1XyzCredential, issuer1xyzCredDef, null);

            createCredReqResult = await AnonCreds.ProverCreateCredentialReqAsync(wallet, proverDid, issuer2GvtCredOffer, issuer2gvtCredDef, masterSecretId);
            var issuer2GvtCredReq = createCredReqResult.CredentialRequestJson;
            var issuer2GvtCredReqMetadata = createCredReqResult.CredentialRequestMetadataJson;

            var gvt2CredValues = "{" +
                    "           \"sex\":{\"raw\":\"male\",\"encoded\":\"2142657394558967239210949258394838228692050081607692519917028371144233115103\"},\n" +
                    "           \"name\":{\"raw\":\"Alexander\",\"encoded\":\"21332817548165488690172217217278169335\"},\n" +
                    "           \"height\":{\"raw\":\"170\",\"encoded\":\"170\"},\n" +
                    "           \"age\":{\"raw\":\"28\",\"encoded\":\"28\"}\n" +
                    "   }";

            createCredResult = await AnonCreds.IssuerCreateCredentialAsync(wallet, issuer2GvtCredOffer, issuer2GvtCredReq, gvt2CredValues, null, null);
            var issuer2GvtCredential = createCredResult.CredentialJson;

            var credentialId3 = "id3";
            await AnonCreds.ProverStoreCredentialAsync(wallet, credentialId3, issuer2GvtCredReqMetadata, issuer2GvtCredential, issuer2gvtCredDef, null);

            _walletOpened = true;
        }
    }
}
