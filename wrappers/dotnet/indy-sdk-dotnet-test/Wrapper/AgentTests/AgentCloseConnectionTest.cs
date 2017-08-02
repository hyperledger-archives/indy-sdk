using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Threading.Tasks;
using static Indy.Sdk.Dotnet.Wrapper.Agent;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    [TestClass]
    public class AgentCloseConnectionTest : AgentIntegrationTestBase
    {
        private static TaskCompletionSource<Connection> _serverToClientConnectionTaskCompletionSource;

        private new static ConnectionOpenedHandler _incomingConnectionObserver = (listener, connection, senderDid, receiverDid) =>
        {
            Console.WriteLine("New connection " + connection);

            _serverToClientConnectionTaskCompletionSource.SetResult(connection);

            return _messageObserverForIncoming;
        };

        [TestInitialize]
        public void Initialize()
        {
            _serverToClientConnectionTaskCompletionSource = new TaskCompletionSource<Connection>();
        }

        [TestMethod]
        public async Task TestAgentCloseConnectionWorksForOutgoing()
        {
            var endpoint = "127.0.0.1:9903";

            var myDid = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDid.Did, myDid.Pk, myDid.VerKey, endpoint);

            Signus.StoreTheirDidAsync(_wallet, identityJson).Wait();

            var activeListener = Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Result;

            activeListener.AddIdentityAsync(_pool, _wallet, myDid.Did).Wait();

            var connection = Agent.AgentConnectAsync(_pool, _wallet, myDid.Did, myDid.Did, _messageObserver).Result;

            connection.CloseAsync().Wait();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                connection.SendAsync("msg")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

        [TestMethod]
        public async Task TestAgentCloseConnectionWorksForIncoming()
        {
            var endpoint = "127.0.0.1:9913";

            var myDid = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;

            var identityJson = string.Format("{{\"did\":\"{0}\", \"pk\":\"{1}\", \"verkey\":\"{2}\", \"endpoint\":\"{3}\"}}",
                    myDid.Did, myDid.Pk, myDid.VerKey, endpoint);

            Signus.StoreTheirDidAsync(_wallet, identityJson).Wait();

            var activeListener = Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Result;

            activeListener.AddIdentityAsync(_pool, _wallet, myDid.Did).Wait();

            var connection = Agent.AgentConnectAsync(_pool, _wallet, myDid.Did, myDid.Did, _messageObserver).Result;

            var serverToClientConnection = _serverToClientConnectionTaskCompletionSource.Task.Result;
            serverToClientConnection.CloseAsync().Wait();


            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                serverToClientConnection.SendAsync("msg")
            );

            Assert.AreEqual(ErrorCode.CommonInvalidStructure, ex.ErrorCode);
        }

    }
}
