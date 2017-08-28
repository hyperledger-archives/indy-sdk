using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
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

            await Wallet.CreateWalletAsync(_poolName, listenerWalletName, "default", null, null);
            var listenerWallet = await Wallet.OpenWalletAsync(listenerWalletName, null, null);

            await Wallet.CreateWalletAsync(_poolName, trusteeWalletName, "default", null, null);
            var trusteeWallet = await Wallet.OpenWalletAsync(trusteeWalletName, null, null);
            var senderWallet = trusteeWallet;

            var createMyDidResult = await Signus.CreateAndStoreMyDidAsync(listenerWallet, "{}");
            var listenerDid = createMyDidResult.Did;
            var listenerVerkey = createMyDidResult.VerKey;
            var listenerPk = createMyDidResult.Pk;

            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeDidResult = await Signus.CreateAndStoreMyDidAsync(trusteeWallet, trusteeDidJson);
            var trusteeDid = trusteeDidResult.Did;
            var senderDid = trusteeDid;

            var nymRequest = await Ledger.BuildNymRequestAsync(trusteeDid, listenerDid, listenerVerkey, null, null);
            await Ledger.SignAndSubmitRequestAsync(_pool, trusteeWallet, trusteeDid, nymRequest);

            var attribRequest = await Ledger.BuildAttribRequestAsync(listenerDid, listenerDid, null,
                    string.Format("{{\"endpoint\":{{\"ha\":\"{0}\",\"verkey\":\"{1}\"}}}}", endpoint, listenerPk), null);
            await Ledger.SignAndSubmitRequestAsync(_pool, listenerWallet, listenerDid, attribRequest);

            var activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(_pool, listenerWallet, listenerDid);

            await AgentConnection.ConnectAsync(_pool, senderWallet, senderDid, listenerDid);

            await listenerWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(listenerWalletName, null);

            await trusteeWallet.CloseAsync();
            await Wallet.DeleteWalletAsync(trusteeWalletName, null);
        }

        [TestMethod]
        public async Task TestAgentConnectWorksForAllDataInWalletPresent()
        {
            var endpoint = "127.0.0.1:9606";

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var activeListener = await AgentListener.ListenAsync(endpoint);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDidResult.Did);

            await AgentConnection.ConnectAsync(_pool, _wallet, myDidResult.Did, myDidResult.Did);
        }
    }
}
