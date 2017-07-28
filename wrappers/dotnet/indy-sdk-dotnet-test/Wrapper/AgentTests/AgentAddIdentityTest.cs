﻿using Indy.Sdk.Dotnet.Test.Wrapper.AgentTests;
using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace Indy.Sdk.Dotnet.Test.Wrapper.LedgerTests
{
    [TestClass]
    public class AgentAddIdentityTest : AgentIntegrationTestBase
    {
        [TestMethod]
        public void TestAgentAddIdentityWorks()
        {
            var endpoint = "127.0.0.1:9901";

            var myDidResult = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;

            var activeListener = Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Result;

            activeListener.AddIdentityAsync(_pool, _wallet, myDidResult.Did).Wait();
        }

        [TestMethod]
        public void TestAgentAddIdentityWorksForMultiplyKeys()
        {
            var endpoint = "127.0.0.1:9902";

            var myDid1 = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;
            var myDid2 = Signus.CreateAndStoreMyDidAsync(_wallet, "{}").Result;

            CreateAndStoreMyDidResult[] didResults = { myDid1, myDid2 };

            var activeListener = Agent.AgentListenAsync(endpoint, _incomingConnectionObserver).Result;

            foreach (var didResult in didResults)
            {
                activeListener.AddIdentityAsync(_pool, _wallet, didResult.Did).Wait();
            }
            }
    }
}
