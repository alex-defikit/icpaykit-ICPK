export const idlFactory = ({ IDL }) => {
  const network = IDL.Variant({
    'mainnet' : IDL.Null,
    'regtest' : IDL.Null,
    'testnet' : IDL.Null,
  });
  const Metadata = IDL.Record({
    'customer_id' : IDL.Opt(IDL.Text),
    'customer_name' : IDL.Opt(IDL.Text),
  });
  const LocalPrice = IDL.Record({ 'currency' : IDL.Text, 'amount' : IDL.Text });
  const ChargeCreate = IDL.Record({
    'pricing_type' : IDL.Text,
    'metadata' : IDL.Opt(Metadata),
    'name' : IDL.Text,
    'local_price' : LocalPrice,
    'description' : IDL.Text,
  });
  const ChargeStatus = IDL.Variant({
    'Failed' : IDL.Null,
    'Completed' : IDL.Null,
    'Expired' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const CPayment = IDL.Record({
    'transaction_id' : IDL.Text,
    'from' : IDL.Principal,
    'created_at' : IDL.Nat64,
    'amount' : IDL.Text,
  });
  const Charge = IDL.Record({
    'id' : IDL.Text,
    'status' : ChargeStatus,
    'merchant_id' : IDL.Principal,
    'pricing_type' : IDL.Text,
    'payments' : IDL.Vec(CPayment),
    'metadata' : IDL.Opt(Metadata),
    'name' : IDL.Text,
    'local_price' : LocalPrice,
    'subaccount' : IDL.Vec(IDL.Nat8),
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'payment_block_height' : IDL.Opt(IDL.Nat64),
    'btc_address' : IDL.Text,
    'release_block_height' : IDL.Opt(IDL.Nat64),
  });
  const CheckoutCreate = IDL.Record({
    'requested_info' : IDL.Vec(IDL.Text),
    'pricing_type' : IDL.Text,
    'name' : IDL.Text,
    'local_price' : LocalPrice,
    'description' : IDL.Text,
  });
  const CheckoutStatus = IDL.Variant({
    'Completed' : IDL.Null,
    'Expired' : IDL.Null,
    'Pending' : IDL.Null,
  });
  const Checkout = IDL.Record({
    'id' : IDL.Text,
    'status' : CheckoutStatus,
    'requested_info' : IDL.Vec(IDL.Text),
    'updated_at' : IDL.Nat64,
    'payment_link' : IDL.Text,
    'merchant_id' : IDL.Principal,
    'pricing_type' : IDL.Text,
    'name' : IDL.Text,
    'local_price' : LocalPrice,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
  });
  const bitcoin_address = IDL.Text;
  const satoshi = IDL.Nat64;
  const block_height = IDL.Nat32;
  const block_header = IDL.Vec(IDL.Nat8);
  const get_block_headers_response = IDL.Record({
    'tip_height' : block_height,
    'block_headers' : IDL.Vec(block_header),
  });
  const millisatoshi_per_vbyte = IDL.Nat64;
  const block_hash = IDL.Vec(IDL.Nat8);
  const outpoint = IDL.Record({
    'txid' : IDL.Vec(IDL.Nat8),
    'vout' : IDL.Nat32,
  });
  const utxo = IDL.Record({
    'height' : IDL.Nat32,
    'value' : satoshi,
    'outpoint' : outpoint,
  });
  const get_utxos_response = IDL.Record({
    'next_page' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'tip_height' : IDL.Nat32,
    'tip_block_hash' : block_hash,
    'utxos' : IDL.Vec(utxo),
  });
  const WebhookConfig = IDL.Record({
    'url' : IDL.Text,
    'merchant_id' : IDL.Principal,
    'secret' : IDL.Text,
    'enabled' : IDL.Bool,
  });
  const transaction_id = IDL.Text;
  const LocalValue = IDL.Record({ 'currency' : IDL.Text, 'amount' : IDL.Text });
  const CryptoValue = IDL.Record({
    'currency' : IDL.Text,
    'amount' : IDL.Text,
  });
  const PaymentValue = IDL.Record({
    'local' : LocalValue,
    'crypto' : CryptoValue,
  });
  const WhPayment = IDL.Record({
    'transaction_id' : IDL.Text,
    'status' : IDL.Text,
    'value' : PaymentValue,
    'created_at' : IDL.Text,
    'block_height' : IDL.Nat64,
  });
  const WhMetadata = IDL.Record({
    'customer_id' : IDL.Opt(IDL.Text),
    'customer_name' : IDL.Opt(IDL.Text),
  });
  const Timeline = IDL.Record({ 'status' : IDL.Text, 'time' : IDL.Text });
  const EventData = IDL.Record({
    'id' : IDL.Text,
    'pricing_type' : IDL.Text,
    'payments' : IDL.Vec(WhPayment),
    'metadata' : IDL.Opt(WhMetadata),
    'code' : IDL.Opt(IDL.Text),
    'name' : IDL.Opt(IDL.Text),
    'hosted_url' : IDL.Opt(IDL.Text),
    'description' : IDL.Opt(IDL.Text),
    'timeline' : IDL.Vec(Timeline),
  });
  const EventType = IDL.Variant({
    'CheckoutCreated' : IDL.Null,
    'ChargeReleaseFailed' : IDL.Null,
    'CheckoutExpired' : IDL.Null,
    'ChargeDelayed' : IDL.Null,
    'ChargeCreated' : IDL.Null,
    'ChargePending' : IDL.Null,
    'ChargeFailed' : IDL.Null,
    'CheckoutCompleted' : IDL.Null,
    'ChargeConfirmed' : IDL.Null,
  });
  const WebhookEvent = IDL.Record({
    'id' : IDL.Text,
    'resource' : IDL.Text,
    'api_version' : IDL.Text,
    'data' : EventData,
    'created_at' : IDL.Nat64,
    'event_type' : EventType,
  });
  return IDL.Service({
    'create_charge' : IDL.Func(
        [ChargeCreate],
        [IDL.Variant({ 'Ok' : Charge, 'Err' : IDL.Text })],
        [],
      ),
    'create_checkout' : IDL.Func(
        [CheckoutCreate],
        [IDL.Variant({ 'Ok' : Checkout, 'Err' : IDL.Text })],
        [],
      ),
    'get_balance' : IDL.Func([bitcoin_address], [satoshi], []),
    'get_block_headers' : IDL.Func(
        [block_height, IDL.Opt(block_height)],
        [get_block_headers_response],
        [],
      ),
    'get_charge' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : Charge, 'Err' : IDL.Text })],
        ['query'],
      ),
    'get_checkout' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : Checkout, 'Err' : IDL.Text })],
        ['query'],
      ),
    'get_current_fee_percentiles' : IDL.Func(
        [],
        [IDL.Vec(millisatoshi_per_vbyte)],
        [],
      ),
    'get_p2pkh_address' : IDL.Func([], [bitcoin_address], []),
    'get_p2tr_raw_key_spend_address' : IDL.Func([], [bitcoin_address], []),
    'get_p2tr_script_spend_address' : IDL.Func([], [bitcoin_address], []),
    'get_utxos' : IDL.Func([bitcoin_address], [get_utxos_response], []),
    'get_webhook_config' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : WebhookConfig, 'Err' : IDL.Text })],
        ['query'],
      ),
    'list_charges' : IDL.Func([], [IDL.Vec(Charge)], ['query']),
    'process_payment' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text })],
        [],
      ),
    'register_merchant' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text })],
        [],
      ),
    'register_merchant_addresses' : IDL.Func(
        [IDL.Text],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text })],
        [],
      ),
    'register_webhook' : IDL.Func(
        [WebhookConfig],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text })],
        [],
      ),
    'send_from_p2pkh' : IDL.Func(
        [
          IDL.Record({
            'destination_address' : bitcoin_address,
            'amount_in_satoshi' : satoshi,
          }),
        ],
        [transaction_id],
        [],
      ),
    'send_from_p2tr_raw_key_spend' : IDL.Func(
        [
          IDL.Record({
            'destination_address' : bitcoin_address,
            'amount_in_satoshi' : satoshi,
          }),
        ],
        [transaction_id],
        [],
      ),
    'send_from_p2tr_script_spend' : IDL.Func(
        [
          IDL.Record({
            'destination_address' : bitcoin_address,
            'amount_in_satoshi' : satoshi,
          }),
        ],
        [transaction_id],
        [],
      ),
    'verify_webhook' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Variant({ 'Ok' : WebhookEvent, 'Err' : IDL.Text })],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const network = IDL.Variant({
    'mainnet' : IDL.Null,
    'regtest' : IDL.Null,
    'testnet' : IDL.Null,
  });
  return [network];
};
