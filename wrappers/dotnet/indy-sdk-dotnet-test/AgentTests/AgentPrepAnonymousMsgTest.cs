using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Newtonsoft.Json.Linq;
using System;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class AgentPrepAnonymousMsgTest : IndyIntegrationTestWithSingleWallet
    {
        private async Task CheckMessage(byte[] encryptedMsg)
        {
            await Wallet.CreateWalletAsync(POOL, "walletForCheck", TYPE, null, null);
            var localWallet = await Wallet.OpenWalletAsync("walletForCheck", null, null);

            var didJson = string.Format("{{\"seed\":\"{0}\",\"cid\":false}}", MY1_SEED);
            var result = await Signus.CreateAndStoreMyDidAsync(localWallet, didJson);
            var recipientDid = result.Did;

            var decryptedMessageBytes = await Signus.DecryptSealedAsync(localWallet, recipientDid, encryptedMsg);
            var decryptedMessageJson = Encoding.UTF8.GetString(decryptedMessageBytes);
            var decryptedMsg = JObject.Parse(decryptedMessageJson);


            Assert.IsFalse(decryptedMsg.Value<bool>("auth"));
            Assert.IsTrue(MESSAGE.SequenceEqual(Convert.FromBase64String(decryptedMsg.Value<string>("msg"))));

            await localWallet.CloseAsync();
            await Wallet.DeleteWalletAsync("walletForCheck", null);            
        }

        [TestMethod]
        public async Task TestPrepAnonymousMsgWorks()
        {
            var encryptedMsg = await Agent.PrepAnonymousMsgAsync(VERKEY_FOR_MY1_SEED, MESSAGE);
            await CheckMessage(encryptedMsg);
        }

        [TestMethod]
        public async Task TestPrepAnonymousMsgWorksForInvalidRecipientVk()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Agent.PrepAnonymousMsgAsync(INVALID_VERKEY, MESSAGE)
            );
        }
    }
}
