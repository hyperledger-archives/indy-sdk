import copy
import json
import sys
ENTERPRISE_DID = '2hoqvcwupRTUNkXn6ArYzs'
config_dev = {
  "sdk_to_remote_verkey": "ECFLTKPNisizb8AyrG4xEMyng1WoRdjspucWNNEPvnfm",
  "institution_did": "2hoqvcwupRTUNkXn6ArYzs",
  "institution_name": "<CHANGE_ME>",
  "remote_to_sdk_verkey": "nroWbyqRFs3SB6EAKCH62dgqWauoySLGiJgtvFRSjgX",
  "agency_verkey": "J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v",
  "wallet_name": "zoidberg",
  "institution_logo_url": "<CHANGE_ME>",
  "wallet_key": "zoidberg",
  "remote_to_sdk_did": "2T94qQsPLUnXmfucC8W1AQ",
  "agency_endpoint": "https://enym-eagency.pdev.evernym.com",
  "sdk_to_remote_did": "RD34zYXMtAkXpbm8GSmSei",
  "agency_did": "YRuVCckY6vfZfX9kcQZe3u",
  "genesis_path": "<CHANGE_ME>",
  "enterprise_verkey": "vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf"
}

config = {
  "institution_did": "XqLGQ6NCi538najLyKBc5y",
  "sdk_to_remote_did": "QDD24KzHvAscC5o2YNfUi7",
  "agency_endpoint": "https://eas01.pps.evernym.com",
  "wallet_key": "jesse",
  "sdk_to_remote_verkey": "DejGJXVGMi2ikwg2TTQ5DgwvMkGbT2t6Z9yKEqkSWKTM",
  "remote_to_sdk_verkey": "HzRKLMoP19XRvwLek2GuJvReYPKWx56dDUZiJS1tBzDz",
  "agency_verkey": "871Kg9brdb6u9tGczaf6eeYobeEnWau5YtdxaZZHmURD",
  "enterprise_verkey": "HohhuEmSwsXTWoUeW4quNgHGoZQ1KwLePGp6NkorGhE4",
  "agency_did": "E2vjDzFQP2b1AqtY66n2st",
  "remote_to_sdk_did": "YAzmMwSFCubxQjjZXTS2vS",
  "institution_logo_url": "<CHANGE_ME>",
  "institution_name": "<CHANGE_ME>",
  "genesis_path": "<CHANGE_ME>",
  "wallet_name": "jesse"
}


def update_json_values(new_values, old_values):
    return_json = copy.deepcopy(old_values)
    for i in new_values:
        return_json[i] = new_values[i]
    return return_json

