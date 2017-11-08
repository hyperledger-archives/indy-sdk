using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.CryptoApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class AgentParseMsgTest : IndyIntegrationTestWithPoolAndSingleWallet
    {
        const string _keyJsonTemplate = "{{\"seed\":\"{0}\"}}";

        [TestMethod]
        public async Task TestParseMsgWorksForAuthenticatedMessage()
        {
            var paramJson = string.Format(_keyJsonTemplate, MY1_SEED);
            var senderVk = await Crypto.CreateKeyAsync(wallet, paramJson);

            paramJson = string.Format(_keyJsonTemplate, MY2_SEED);
            var recipientVk = await Crypto.CreateKeyAsync(wallet, paramJson);

            var encryptedMsg = await Agent.PrepMsgAsync(wallet, senderVk, recipientVk, MESSAGE);
            var parseResult = await Agent.ParseMsgAsync(wallet, recipientVk, encryptedMsg);

            Assert.AreEqual(senderVk, parseResult.SenderKey);
            Assert.IsTrue(MESSAGE.SequenceEqual(parseResult.Msg));
        }

        [TestMethod]
        public async Task TestParseMsgWorksForAnonymousMessage()
        {
            var paramJson = string.Format(_keyJsonTemplate, MY2_SEED);
            var recipientVk = await Crypto.CreateKeyAsync(wallet, paramJson);

            var encryptedMsg = await Agent.PrepAnonymousMsgAsync(recipientVk, MESSAGE);
            var parseResult = await Agent.ParseMsgAsync(wallet, recipientVk, encryptedMsg);

            Assert.IsNull(parseResult.SenderKey);
            Assert.IsTrue(MESSAGE.SequenceEqual(parseResult.Msg));
        }

        [TestMethod]
        public async Task TestParseMsgWorksForInvalidAuthenticatedMessage()
        {
            var result = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");
            var recipientDid = result.Did;
            var recipientVk = result.VerKey;

            var identityJson = string.Format(IDENTITY_JSON_TEMPLATE, recipientDid, recipientVk);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var msg = string.Format("{{\"auth\":true,\"nonсe\":\"Th7MpTaRZVRYnPiabds81Y12\",\"sender\":\"{0}\",\"msg\":\"unencrypted message\"}}", VERKEY);
            var encryptedMsg = await Signus.EncryptSealedAsync(wallet, pool, recipientDid, Encoding.UTF8.GetBytes(msg));

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Agent.ParseMsgAsync(wallet, recipientVk, encryptedMsg)
            );
        }

        [TestMethod]
        public async Task TestParseMsgWorksForInvalidAnonymousMessage()
        {
            var recipientVk = await Crypto.CreateKeyAsync(wallet, "{}");

            var msg = "unencrypted message";
            var encryptedMsg = Encoding.UTF8.GetBytes(msg);

            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Agent.ParseMsgAsync(wallet, recipientVk, encryptedMsg)
            );
        }

        [TestMethod]
        public async Task TestParseMsgWorksForUnknownRecipientVk()
        {
            var encryptedMsg = await Agent.PrepAnonymousMsgAsync(VERKEY, MESSAGE);

            var ex = await Assert.ThrowsExceptionAsync<WalletValueNotFoundException>(() =>
                Agent.ParseMsgAsync(wallet, VERKEY, encryptedMsg)
            );
        }
    }
}
