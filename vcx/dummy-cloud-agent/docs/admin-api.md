# VCX Agency Admin API
When troubleshooting, it's handy to be able quickly find out more information about the state of agency and the
entities in it. For that, you enable "Admin API" in agency configuration and query the Agency via HTTP.

## Enabling Admin API
Add following section to your agency configuration
```
"server_admin": {
    "enabled": true,
    "addresses": [
      "127.0.0.1:8090"
    ]
  }
```
This will expose Admin API on address `127.0.0.1:8090`.

## Endpoints

Following endpoints are implemented:

- `/admin` - returns information about entities in the agency, such as Forward Agent Connections, Agents and
Agent Connections.
- `/admin/forward-agent` - Returns detailed information about Forward Agent
- `/admin/agent/{agent_did}` - Returns detailed information about particular Agent, identified by its DID.
- `/admin/agent-connection/{agent_pairwise_did}` - Returns detailed information about particular Agent Connection,
identified by its DID.

## Admin API Response examples

##### Information about entities in agency
```shell script
curl -s localhost:8090/admin | jq
```
```json
{
  "Admin": {
    "forwardAgentConnections": [
      "2Dy3j4ySzAhpH643KXew9h",
      "V6eRXvJjjZ4dceFzqPKrfv",
      "PJwjVvWg9wX5tHsWjtCZGm",
      "GFqLSeECGnwtH8Ks46YSqp",
      "RdCVcgkewG5mFzHF2j66xK",
      "9SvSzKJMKj1KPS8bND2THM"
    ],
    "agents": [
      "Fvd3NCJdR2yPBtHoyghTBn",
      "SoDndittvKzDgg3Vzc7JuJ",
      "UHZQCCpWG6XXySwfhTSPTJ",
      "8FUUsfdv6qTLLoR8HjNKDW",
      "Fk5DZEopAAXqcv6yuEiee5",
      "HbBRwBuEWajqwsQCCW7FfW"
    ],
    "agentConnections": [
      "7F3DF8V3TrAdD976oyY5qv",
      "Ckjh79iX3woGNpShRHk1Y2",
      "6k4aBNwssTHswMNJqP5Vbg",
      "PLz5Jqp2TpT5LGJbTji4CT",
      "D8VBMXgdcGjZABT6sgjqps",
      "7ZmnMSav6NiBNsuQcQ1hWH",
      "GGCKtwdeGFfn7DEF7cqopF",
      "6fpJfd7BurtVxwP6PzrWFw",
      "UZKr3UjDWE2WccJ7EUiDur",
      "B2mKkmpi6cAM1EiFAKhLBd",
      "XR6H77GdLXPB4FwSRgtkof"
    ]
  }
}
```

#### Forward agent details
```shell script
curl -s localhost:8090/admin/forward-agent | jq
```

```
{
  "ForwardAgent": {
    "walletHandle": 3,
    "forwardAgentEndpoint": "http://localhost:8080",
    "pairwiseList": [
      "{\"my_did\":\"2Dy3j4ySzAhpH643KXew9h\",\"their_did\":\"ThCkCcyG1jPJ8NKVkcrTFr\",\"metadata\":\"{\\\"is_signed_up\\\":true,\\\"agent\\\":[\\\"dummy_ThCkCcyG1jPJ8NKVkcrTFr_VQPtTFWjsO\\\",\\\"KKZRbTP7IM\\\",\\\"SoDndittvKzDgg3Vzc7JuJ\\\"]}\"}",
      "{\"my_did\":\"PJwjVvWg9wX5tHsWjtCZGm\",\"their_did\":\"VC5xkh9RNUei3DipSVZBov\",\"metadata\":\"{\\\"is_signed_up\\\":true,\\\"agent\\\":[\\\"dummy_VC5xkh9RNUei3DipSVZBov_ELvqB1kLE6\\\",\\\"zmJkdV4uTu\\\",\\\"Fk5DZEopAAXqcv6yuEiee5\\\"]}\"}",
      "{\"my_did\":\"9SvSzKJMKj1KPS8bND2THM\",\"their_did\":\"WqfSc6kHMo6xc4QZYJPgwN\",\"metadata\":\"{\\\"is_signed_up\\\":true,\\\"agent\\\":[\\\"dummy_WqfSc6kHMo6xc4QZYJPgwN_chiZQepFcz\\\",\\\"xDeuVE8D5X\\\",\\\"UHZQCCpWG6XXySwfhTSPTJ\\\"]}\"}",
      "{\"my_did\":\"RdCVcgkewG5mFzHF2j66xK\",\"their_did\":\"R1oNQaEpYo3cMguVR8KAyE\",\"metadata\":\"{\\\"is_signed_up\\\":true,\\\"agent\\\":[\\\"dummy_R1oNQaEpYo3cMguVR8KAyE_EwNfnWNjOn\\\",\\\"ZRMwGm6ogs\\\",\\\"Fvd3NCJdR2yPBtHoyghTBn\\\"]}\"}",
      "{\"my_did\":\"GFqLSeECGnwtH8Ks46YSqp\",\"their_did\":\"8LFL1eBGw9yqdQEPekSyNy\",\"metadata\":\"{\\\"is_signed_up\\\":true,\\\"agent\\\":[\\\"dummy_8LFL1eBGw9yqdQEPekSyNy_Qdqf1JI7vE\\\",\\\"sBsP0E3UgW\\\",\\\"HbBRwBuEWajqwsQCCW7FfW\\\"]}\"}",
      "{\"my_did\":\"V6eRXvJjjZ4dceFzqPKrfv\",\"their_did\":\"17e3aKmnqi4TPwGrAhHp82\",\"metadata\":\"{\\\"is_signed_up\\\":true,\\\"agent\\\":[\\\"dummy_17e3aKmnqi4TPwGrAhHp82_eQNHp3Hfn8\\\",\\\"Bw3jF9YTuw\\\",\\\"8FUUsfdv6qTLLoR8HjNKDW\\\"]}\"}"
    ]
  }
}
```

### Agent detail
```shell script
curl -s localhost:8090/admin/agent/HbBRwBuEWajqwsQCCW7FfW | jq
```
```json
{
  "Agent": {
    "ownerDid": "8LFL1eBGw9yqdQEPekSyNy",
    "ownerVerkey": "4zmu2PZXgDS84npSMgj7ZVsWYY5quSfsT83JcRUBsxXJ",
    "did": "HbBRwBuEWajqwsQCCW7FfW",
    "verkey": "A3RQGPDyBt5BCeecVT33ANb7R3jLwkpgcqGyP6B54bEm",
    "configs": [
      [
        "name",
        "absa"
      ]
    ]
  }
}
```

### Agent connection detail
```shell script
curl -s localhost:8090/admin/agent-connection/UZKr3UjDWE2WccJ7EUiDur | jq
```
```json
{
  "AgentConn": {
    "agentDetailVerkey": "8anoTkF58672bu1MsC5aBedJ9KyfR9oEAASNVozVrXky",
    "agentDetailDid": "EuuPPHzgpJuRERy4TT9rge",
    "forwardAgentDetailVerkey": "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR",
    "forwardAgentDetailDid": "VsKV7grR1BUE29mG2Fm2kX",
    "forwardAgentDetailEndpoint": "http://localhost:8080",
    "agentConfigs": [
      [
        "name",
        "absa"
      ]
    ],
    "name": "absa",
    "logo": "unknown"
  }
}
```
