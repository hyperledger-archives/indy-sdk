from pygments.token import Token

CLI_NAME = 'Indy-SDK'
NO_ENV = "no-env"
WALLET = 'wallet1'
POOL = 'pool1'
VERSION = '0.0.1'
WORK_DIR = '.indy_client'
STATE_FILE = 'state.json'

# NODE_GRAMS_NODE_COMMAND_REG_EX = \
#     "(\s* (?P<node_command>{}) \s+ (?P<node_or_cli>nodes?) " \
#     "\s+ (?P<node_name>[a-zA-Z0-9\-]+)\s*) "
# NODE_GRAMS_LOAD_PLUGINS_REG_EX = \
#     "(\s* (?P<load_plugins>load\s+plugins\s+from) " \
#     "\s+ (?P<plugin_dir>[a-zA-Z0-9-_:{}]+) \s*)"
#
# CLIENT_GRAMS_CLIENT_COMMAND_REG_EX = \
#     "(\s* (?P<client_command>{}) \s+ (?P<node_or_cli>clients?) " \
#     "\s+ (?P<client_name>[a-zA-Z0-9\-]+) \s*) "
# CLIENT_GRAMS_CLIENT_SEND_REG_EX = \
#     "(\s* (?P<client>client) \s+ (?P<client_name>[a-zA-Z0-9]+) " \
#     "\s+ (?P<cli_action>send) \s+ (?P<msg>\{\s*.*\}) \s*) "
# CLIENT_GRAMS_CLIENT_SHOW_REG_EX = \
#     "(\s* (?P<client>client) \s+ (?P<client_name>[a-zA-Z0-9]+) " \
#     "\s+ (?P<cli_action>show) \s+ (?P<req_id>[0-9]+) \s*) "
# CLIENT_GRAMS_ADD_KEY_REG_EX = \
#     "(\s* (?P<add_key>add\s+key) \s+ (?P<verkey>[a-fA-F0-9]+) " \
#     "\s+ (?P<for_client>for\s+client) \s+ (?P<DID>[a-zA-Z0-9]+) \s*) "
# CLIENT_GRAMS_NEW_KEYPAIR_REG_EX = \
#     "(\s* (?P<new_key>new\skey) \s*" \
#     "\s? (with\s+seed\s+(?P<seed>[a-zA-Z0-9]+))?" \
#     "\s? ((as)?\s+(?P<alias>[a-zA-Z0-9-]+))?" \
#     "\s*) "
#
# CLIENT_GRAMS_RENAME_WALLET_REG_EX = \
#     "(\s*(?P<rename_wallet>rename\s+wallet)" \
#     "\s? (\s+(?P<from>[A-Za-z0-9+=/]*))?" \
#     "\s+ (to\s+(?P<to>[A-Za-z0-9+=/]*))" \
#     "\s*) "
#
# CLIENT_GRAMS_LIST_IDS_REG_EX = "(\s* (?P<list_ids>list\sids) " \
#                                "\s?(?P<with_verkeys>with\s+verkeys)? \s*) "
#
# CLIENT_GRAMS_LIST_WALLETS_REG_EX = "(\s* (?P<list_wallets>list\swallets) \s*) "
#
# CLIENT_GRAMS_BECOME_REG_EX = "(\s* (?P<become>become) " \
#                              "\s+ (?P<id>[a-zA-Z0-9]+) \s*) "
#
# CLIENT_GRAMS_USE_KEYPAIR_REG_EX = "(\s* (?P<use_id>use\s+DID) " \
#                                   "\s+ (?P<DID>[A-Za-z0-9+=/]*) \s*) "
#
# CLIENT_GRAMS_USE_WALLET_REG_EX = "(\s* (?P<use_wallet>use\s+wallet) " \
#                                  "\s+ (?P<wallet>[A-Za-z0-9+-_=/]*) \s*" \
#                                  "\s? ((?P<copy_as>copy\sas)\s" \
#                                  "(?P<copy_as_name>[A-Za-z0-9+-_=/]+)?)? \s*" \
#                                  "\s? (?P<override>override)? " \
#                                  "\s*)"
#
# CLIENT_GRAMS_SAVE_WALLET_REG_EX = "(\s* (?P<save_wallet>save\s+wallet)" \
#                                   "\s? (?P<wallet>[A-Za-z0-9+-_=/]+)? \s*)"
#
# CLIENT_GRAMS_ADD_GENESIS_TXN_REG_EX = \
#     "(\s*(?P<add_gen_txn>add \s+ genesis \s+ transaction)" \
#     "\s+ (?P<type>[a-zA-Z0-9_]+)" \
#     "\s+ (for\s+(?P<dest>[A-Za-z0-9+=/]+))?" \
#     "\s? (by\s+(?P<identifier>[A-Za-z0-9+=/]*))?" \
#     "\s? (with\s+data\s+(?P<data>\{{\s*.*\}}))?" \
#     "\s? (role\s*=\s*(?P<role>{role}))?" \
#     "\s*) ".format(role=Roles.STEWARD.name)
#
# CLIENT_GRAMS_CREATE_GENESIS_TXN_FILE_REG_EX = \
#     "(\s*(?P<create_gen_txn_file>create " \
#     "\s+ genesis \s+ transaction \s+ file)\s*)"

CLI_STYLE = {
    Token.Operator: '#33aa33 bold',
    Token.Gray: '#424242',
    Token.Number: '#aa3333 bold',
    Token.Name: '#ffff00 bold',
    Token.Heading: 'bold',
    Token.TrailingInput: 'bg:#662222 #ffffff',
    Token.BoldGreen: '#33aa33 bold',
    Token.BoldOrange: '#ff4f2f bold',
    Token.BoldBlue: '#095cab bold'
}
