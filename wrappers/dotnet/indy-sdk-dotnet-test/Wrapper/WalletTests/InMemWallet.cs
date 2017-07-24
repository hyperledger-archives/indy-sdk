using Indy.Sdk.Dotnet.Wrapper;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.WalletTests
{
    public class InMemWallet : WalletBase
    {
        private static IDictionary<string, string> _values = new Dictionary<string, string>();

        public bool IsOpen { get; set; }

        public override ErrorCode Set(string key, string value)
        {
            _values.Add(key, value);
            return ErrorCode.Success;
        }

        public override ErrorCode Get(string key, out string value)
        {
            value = null;

            if (_values.ContainsKey(key))
                return ErrorCode.WalletNotFoundError;

            value = _values[key];

            return ErrorCode.Success;
        }

        public override ErrorCode GetNotExpired(string key, out string value)
        {
            value = null;

            if (_values.ContainsKey(key))
                return ErrorCode.WalletNotFoundError;

            value = _values[key]; //Nothing is ever expired in *this* wallet...

            return ErrorCode.Success;
        }

        public override ErrorCode List(string keyPrefix, out string valuesJson)
        {
            var matchingValues = _values.Where((x) => x.Key.StartsWith(keyPrefix));

            valuesJson = JsonConvert.SerializeObject(matchingValues.ToArray());

            return ErrorCode.Success;
        }
    }
}
