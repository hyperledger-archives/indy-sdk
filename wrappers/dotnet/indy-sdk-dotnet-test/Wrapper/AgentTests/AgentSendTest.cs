using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.Agent;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentSendTest : AgentIntegrationTestBase
    {
        private static TaskCompletionSource<Connection> serverToClientConnectionFuture = new TaskCompletionSource<Connection>();
        private static TaskCompletionSource<string> serverToClientMsgFuture = new TaskCompletionSource<string>();
        private static TaskCompletionSource<string> clientToServerMsgFuture = new TaskCompletionSource<string>();

        private new static MessageReceivedHandler _messageObserver = (Connection connection, string message) =>
            {
                Console.WriteLine("Received message '" + message + "' on connection " + connection);

                serverToClientMsgFuture.SetResult(message);
            };

        private new static MessageReceivedHandler _messageObserverForIncoming = (Connection connection, string message) =>
            {
                Console.WriteLine("Received message '" + message + "' on incoming connection " + connection);

                clientToServerMsgFuture.SetResult(message);
            };

        private new static ConnectionOpenedHandler _incomingConnectionObserver = (Listener listener, Connection connection, string senderDid, string receiverDid) =>
            {
                Console.WriteLine("New connection " + connection);

                serverToClientConnectionFuture.SetResult(connection);

                return _messageObserverForIncoming;
            };

        [TestMethod]
        public async Task TestAgentSendWorks()
        {
            var endpoint = "127.0.0.1:9609";

            var myDidResult = await Signus.CreateAndStoreMyDidAsync(_wallet, "{}");

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDidResult.Did, myDidResult.Pk, myDidResult.VerKey, endpoint);
            await Signus.StoreTheirDidAsync(_wallet, identityJson);

            var activeListener = await Agent.AgentListenAsync(endpoint, _incomingConnectionObserver);

            await activeListener.AddIdentityAsync(_pool, _wallet, myDidResult.Did);

            var clientToServerConnection = await Agent.AgentConnectAsync(_pool, _wallet, myDidResult.Did, myDidResult.Did, _messageObserver);

            var clientToServerMessage = "msg_from_client";
            var serverToClientMessage = "msg_from_server";

            await clientToServerConnection.SendAsync(clientToServerMessage);

            Assert.AreEqual(clientToServerMessage, await clientToServerMsgFuture.Task);

            var serverToClientConnection = await serverToClientConnectionFuture.Task;
            await serverToClientConnection.SendAsync(serverToClientMessage);

            Assert.AreEqual(serverToClientMessage, await serverToClientMsgFuture.Task);
        }
    }
}
