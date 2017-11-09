using Hyperledger.Indy.WalletApi;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Hyperledger.Indy.Test.WalletTests
{
    public class InMemWallet : ICustomWallet
    {
        private IDictionary<string, WalletRecord> _records = new Dictionary<string, WalletRecord>();
        private TimeSpan _freshnessDuration;

        public InMemWallet(TimeSpan freshnessDuration)
        {
            _freshnessDuration = freshnessDuration;
        }

        public bool IsOpen { get; set; }
        
        public ErrorCode Set(string key, string value)
        {
            var record = new WalletRecord() { Value = value, TimeCreated = DateTime.Now };

            _records[key] = record;
            return ErrorCode.Success;
        }

        public ErrorCode Get(string key, out string value)
        {
            value = null;

            if (!_records.ContainsKey(key))
                return ErrorCode.WalletNotFoundError;

            var record = _records[key];

            value = record.Value;

            return ErrorCode.Success;
        }

        public ErrorCode GetNotExpired(string key, out string value)
        {
            value = null;

            if (!_records.ContainsKey(key))
                return ErrorCode.WalletNotFoundError;

            var record = _records[key];
            var recordAge = DateTime.Now - record.TimeCreated;

            if (recordAge > _freshnessDuration)
                return ErrorCode.WalletNotFoundError;

            value = record.Value;

            return ErrorCode.Success;
        }

        public ErrorCode List(string keyPrefix, out string valuesJson)
        {
            var matchingItems = _records.Where(kvp => kvp.Key.StartsWith(keyPrefix)).ToList();

            var valuesArray = new JArray();

            foreach(var item in matchingItems)
            {
                var record = item.Value;

                var value = new JObject();
                value.Add("key", item.Key);
                value.Add("value", record.Value);

                valuesArray.Add(value);
            }

            var valuesJObject = new JObject();
            valuesJObject.Add("values", valuesArray);

            valuesJson = valuesJObject.ToString(Formatting.None);

            return ErrorCode.Success;
        }

        private class WalletRecord
        {
            public string Value { get; set; }
            public DateTime TimeCreated { get; set; }
        }
    }
}
