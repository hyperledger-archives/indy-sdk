using Indy.Sdk.Dotnet.Wrapper;
using Indy.Sdk.Dotnet.Wrapper.Pool;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Api
{
    public class Ledger
    {
        internal IntPtr PoolHandle { get; }

        private Ledger(IntPtr poolHandle)
        {
            PoolHandle = poolHandle;
        }

        public static Task CreateConfigAsync(string configName, string config)
        {
            return PoolWrapper.CreatePoolLedgerConfigAsync(configName, config);
        }

        public static Task DeleteConfigAsync(string configName)
        {
            return PoolWrapper.DeletePoolLedgerConfigAsync(configName);
        }

        public static async Task<Ledger> OpenAsync(string configName, string config)
        {
            var poolHandle = await PoolWrapper.OpenPoolLedgerAsync(configName, config);
            return new Ledger(poolHandle);
        }

        public Task RefreshAsync()
        {
            return PoolWrapper.RefreshPoolLedgerAsync(PoolHandle);
        }

        public Task CloseAsync()
        {
            return PoolWrapper.ClosePoolLedgerAsync(PoolHandle);
        }

        public Task<string> SignAndSubmitRequestAsync(Wallet wallet, string submitterDid, string requstJson)
        {
            return LedgerWrapper.SignAndSubmitRequestAsync(PoolHandle, wallet.Handle, submitterDid, requstJson);
        }

        public Task<string> SubmitRequestAsync(string requstJson)
        {
            return LedgerWrapper.SubmitRequestAsync(PoolHandle, requstJson);
        }

        public static Task<string> BuildGetDdoRequestAsync(string submitterDid, string targetDid)
        {
            return LedgerWrapper.BuildGetDdoRequestAsync(submitterDid, targetDid);
        }

        public static Task<string> BuildNymRequestAsync(string submitterDid, string targetDid, string verKey, string alias, string role)
        {
            return LedgerWrapper.BuildNymRequestAsync(submitterDid, targetDid, verKey, alias, role);
        }
    }
}
