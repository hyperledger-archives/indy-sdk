using Hyperledger.Indy.AgentApi;
using Hyperledger.Indy.LedgerApi;
using Hyperledger.Indy.SignusApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AgentTests
{
    [TestClass]
    public class AgentConnectTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAgentConnectWorksForRemoteData()
        {
            var endpoint = "127.0.0.1:9605";
            var listenerWalletName = "listenerWallet";
            var trusteeWalletName = "trusteeWallet";

            await Wallet.CreateWalletAsync(poolName, listenerWalletName, TYPE, null, null);
            var listenerWallet = await Wallet.OpenWalletAsync(listenerWalletName, null, null);

            await Wallet.CreateWalletAsync(poolName, trusteeWalletName, TYPE, null, null);
            var trusteeWallet = await Wallet.OpenWalletAsync(trusteeWalletName, null, null);
            var senderWallet = trusteeWallet;

            var createMyDidResult = await Signus.CreateAndStoreMyDidAsync(listenerWallet, "{}");
            var listenerDid = createMyDidResult.Did;
            var listenerVerkey = createMyDidResult.VerKey;
            var listenerPk = createMyDidResult.Pk;

            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(trusteeWallet, TRUSTEE_IDENTITY_JSON);
            var trusteeDid = trusteeDidResult.Did;
            var senderDid = trusteeDid;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, listenerDid, listenerVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(pool, trusteeWallet, trusteeDid, nymRequest);

            var attribRequest = await Ledger.BuildAttribRequestAsync(listenerDid, listenerDid, null,
                    string.Format("{{\"endpoint\":{{\"ha\":\"{0}\",\"verkey\":\"{1}\"}}}}", endpoint, listenerPk), null);
            await Ledger.SignAndSubmitRequestAsync(pool, listenerWallet, listenerDid, attribRequest);

            var activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(pool, listenerWallet, listenerDid);

            await AgentConnection.ConnectAsync(pool, senderWallet, senderDid, listenerDid);

            await listenerWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(listenerWalletName, null);

            await trusteeWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(trusteeWalletName, null);
        }

        [TestMethod]
        public async Task TestAgentConnectWorksForAllDataInWalletPresent()
        {
            var endpoint = "127.0.0.1:9606";

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(wallet, "{}");

            var identityJson = string.Format(AGENT_IDENTITY_JSON_TEMPLATE, myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            await Signus.StoreTheirDidAsync(wallet, identityJson);

            var activeListener = await AgentListener.ListenAsync(endpoint);
            await activeListener.AddIdentityAsync(pool, wallet, myDidResult.Did);

            await AgentConnection.ConnectAsync(pool, wallet, myDidResult.Did, myDidResult.Did);
        }
    }
}
