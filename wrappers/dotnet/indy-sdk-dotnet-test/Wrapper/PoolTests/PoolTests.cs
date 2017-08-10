using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.PoolTests
{
    [TestClass]
    public class PoolTests : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task CanDisposeClosedPool()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            using (var pool = await Pool.OpenPoolLedgerAsync(poolName, null))
            {
                await pool.CloseAsync();
            }
        }

        [TestMethod]        
        public async Task DisposeCanBeCalledRepeatedly()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            pool.Dispose();
            pool.Dispose();
        }

        [TestMethod]
        public async Task PoolWithSameNameCanBeOpenedAfterDispose()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            pool.Dispose();

            using (var newPool = await Pool.OpenPoolLedgerAsync(poolName, null))
            {
            }
        }

        [TestMethod]
        public async Task ClosingDisposedPoolStillProvidesSDKError()
        {
            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            pool.Dispose();

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                pool.CloseAsync()
            );

            Assert.AreEqual(ErrorCode.PoolLedgerInvalidPoolHandle, ex.ErrorCode);
        }

        [TestMethod]
        [Ignore] //TODO: Determine why running this test will cause other tests to fail.
        public async Task FinalizeCleansUpIfNotAlreadyDone()
        {
            //This test doesn't do much other than ensure the finalizer is
            //exercised where the pool isn't closed or disposed.
            //
            //Running this test appears to fail when the SDK attempts to call the callback on close.
            //
            //The following message is displayed:
            //      System.AppDomainUnloadedException: Attempted to access an unloaded AppDomain. This can happen if the test(s) started a thread but did not stop it. Make sure that all the threads started by the test(s) are stopped before completion.
            //
            //The pool handle is not released which means subsequent calls to open will fail.
            //
            //Perhaps we need to be able to call indy_close_pool_ledger without a callback?

            var poolName = PoolUtils.CreatePoolLedgerConfig();

            var pool = await Pool.OpenPoolLedgerAsync(poolName, null);
            pool = null;
            GC.Collect();

            using (var newPool = await Pool.OpenPoolLedgerAsync(poolName, null))
            {
            }
        }
    }
}
