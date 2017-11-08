using Hyperledger.Indy.CryptoApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    public abstract class CryptoIntegrationTestBase : IndyIntegrationTestWithSingleWallet
    {
        public readonly byte[] SIGNATURE = new byte[] { 169, 215, 8, 225, 7, 107, 110, 9, 193, 162, 202, 214, 162, 66, 238, 211, 63, 209, 12, 196, 8, 211, 55, 27, 120, 94, 204, 147, 53, 104, 103, 61, 60, 249, 237, 127, 103, 46, 220, 223, 10, 95, 75, 53, 245, 210, 241, 151, 191, 41, 48, 30, 9, 16, 78, 252, 157, 206, 210, 145, 125, 133, 109, 11 };
        public const string INVALID_BASE58_DID = "invalid_base58string";
        public const string KEY_NOT_IN_WALLET = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4vob";

        protected string senderVerKey;
        protected string recipientVerKey;

        [TestInitialize]
        public async Task CreateKeys()
        {
            var myKeyJson = string.Format("{{\"seed\":\"{0}\"}}", MY1_SEED);
            senderVerKey = await Crypto.CreateKeyAsync(wallet, myKeyJson);

            var theirKeyJson = string.Format("{{\"seed\":\"{0}\"}}", MY2_SEED);
            recipientVerKey = await Crypto.CreateKeyAsync(wallet, theirKeyJson);
        }
    }
}
