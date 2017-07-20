using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentConnectTest : AgentIntegrationTest
    {
        [TestMethod]
        public void TestAgentConnectWorksForRemoteData()
        {
            var endpoint = "127.0.0.1:9905";
            var listenerWalletName = "listenerWallet";
            var trusteeWalletName = "trusteeWallet";

            Wallet.CreateWalletAsync(_poolName, listenerWalletName, "default", null, null).Wait();
            var listenerWallet = Wallet.OpenWalletAsync(listenerWalletName, null, null).Result;

            Wallet.CreateWalletAsync(_poolName, trusteeWalletName, "default", null, null).Wait();
            var trusteeWallet = Wallet.OpenWalletAsync(trusteeWalletName, null, null).Result;
            var senderWallet = trusteeWallet;

            var createMyDidResult = Signus.CreateAndStoreMyDidAsync(listenerWallet, "{}").Result;
            var listenerDid = createMyDidResult.Did;
            var listenerVerkey = createMyDidResult.VerKey;
            var listenerPk = createMyDidResult.Pk;

            var trusteeDidJson = "{\"seed\":\"000000000000000000000000Trustee1\"}";

            var trusteeDidResult = Signus.CreateAndStoreMyDidAsync(trusteeWallet, trusteeDidJson).Result;
            var trusteeDid = trusteeDidResult.Did;
            var senderDid = trusteeDid;

            var nymRequest = Ledger.BuildNymRequestAsync(trusteeDid, listenerDid, listenerVerkey, null, null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, trusteeWallet, trusteeDid, nymRequest).Wait();

            var attribRequest = Ledger.BuildAttribRequestAsync(listenerDid, listenerDid, null,
                    string.Format("{{\"endpoint\":{{\"ha\":\"{0}\",\"verkey\":\"{1}\"}}}}", endpoint, listenerPk), null).Result;
            Ledger.SignAndSubmitRequestAsync(_pool, listenerWallet, listenerDid, attribRequest).Wait();

            var activeListener = Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Result;

            activeListener.AddIdentityAsync(_pool, listenerWallet, listenerDid).Wait();

            Agent.AgentConnectAsync(_pool, senderWallet, senderDid, listenerDid, _messageObserver).Wait();

            listenerWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(listenerWalletName, null).Wait();

            trusteeWallet.CloseAsync().Wait();
            Wallet.DeleteWalletAsync(trusteeWalletName, null).Wait();
        }

        [TestMethod]
        public void TestAgentConnectWorksForAllDataInWalletPresent()
        {
            var endpoint = "127.0.0.1:9906";

            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            Signus.StoreTheirDidAsync(_wallet, identityJson).Wait();

            var activeListener = Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Result;

            activeListener.AddIdentityAsync(_pool, _wallet, myDidResult.Did).Wait();

            Agent.AgentConnectAsync(_pool, _wallet, myDidResult.Did, myDidResult.Did, _messageObserver).Wait();
        }
    }
}
