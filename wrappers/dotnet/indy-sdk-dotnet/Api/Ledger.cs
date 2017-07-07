using Indy.Sdk.Dotnet.Wrapper;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Api
{
    public class Ledger
    {
        internal Wrapper.Pool PoolWrapper { get; }

        private Ledger(Wrapper.Pool poolWrapper)
        {
            PoolWrapper = poolWrapper;
        }

        public static Task CreateConfigAsync(string configName, string config)
        {
            return Wrapper.Pool.CreatePoolLedgerConfigAsync(configName, config);
        }

        public static Task DeleteConfigAsync(string configName)
        {
            return Wrapper.Pool.DeletePoolLedgerConfigAsync(configName);
        }

        public static async Task<Ledger> OpenAsync(string configName, string config)
        {
            var poolWrapper = await Wrapper.Pool.OpenPoolLedgerAsync(configName, config);
            return new Ledger(poolWrapper);
        }

        public Task RefreshAsync()
        {
            return PoolWrapper.RefreshAsync();
        }

        public Task CloseAsync()
        {
            return PoolWrapper.CloseAsync();
        }

        public Task<string> SignAndSubmitRequestAsync(Wallet wallet, string submitterDid, string requstJson)
        {
            return Wrapper.Ledger.SignAndSubmitRequestAsync(PoolWrapper, wallet.WalletWrapper, submitterDid, requstJson);
        }

        public Task<string> SubmitRequestAsync(string requstJson)
        {
            return Wrapper.Ledger.SubmitRequestAsync(PoolWrapper, requstJson);
        }

        public static Task<string> BuildGetDdoRequestAsync(string submitterDid, string targetDid)
        {
            return Wrapper.Ledger.BuildGetDdoRequestAsync(submitterDid, targetDid);
        }

        public static Task<string> BuildNymRequestAsync(string submitterDid, string targetDid, string verKey, string alias, string role)
        {
            return Wrapper.Ledger.BuildNymRequestAsync(submitterDid, targetDid, verKey, alias, role);
        }
    }
}
