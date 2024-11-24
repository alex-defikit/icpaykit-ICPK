import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Charge {
  'id' : string,
  'status' : ChargeStatus,
  'merchant_id' : Principal,
  'pricing_type' : string,
  'payments' : Array<Payment>,
  'metadata' : [] | [Metadata],
  'name' : string,
  'local_price' : LocalPrice,
  'subaccount' : Uint8Array | number[],
  'description' : string,
  'created_at' : bigint,
  'payment_block_height' : [] | [bigint],
  'btc_address' : string,
  'release_block_height' : [] | [bigint],
}
export interface ChargeCreate {
  'pricing_type' : string,
  'metadata' : [] | [Metadata],
  'name' : string,
  'local_price' : LocalPrice,
  'description' : string,
}
export type ChargeStatus = { 'Failed' : null } |
  { 'Completed' : null } |
  { 'Expired' : null } |
  { 'Pending' : null };
export interface Checkout {
  'id' : string,
  'status' : CheckoutStatus,
  'requested_info' : Array<string>,
  'updated_at' : bigint,
  'payment_link' : string,
  'merchant_id' : Principal,
  'pricing_type' : string,
  'name' : string,
  'local_price' : LocalPrice,
  'description' : string,
  'created_at' : bigint,
}
export interface CheckoutCreate {
  'requested_info' : Array<string>,
  'pricing_type' : string,
  'name' : string,
  'local_price' : LocalPrice,
  'description' : string,
}
export type CheckoutStatus = { 'Completed' : null } |
  { 'Expired' : null } |
  { 'Pending' : null };
export interface CryptoValue { 'currency' : string, 'amount' : string }
export interface EventData {
  'id' : string,
  'pricing_type' : string,
  'payments' : Array<WhPayment>,
  'metadata' : [] | [WhMetadata],
  'code' : [] | [string],
  'name' : [] | [string],
  'hosted_url' : [] | [string],
  'description' : [] | [string],
  'timeline' : Array<Timeline>,
}
export type EventType = { 'CheckoutCreated' : null } |
  { 'ChargeReleaseFailed' : null } |
  { 'CheckoutExpired' : null } |
  { 'ChargeDelayed' : null } |
  { 'ChargeCreated' : null } |
  { 'ChargePending' : null } |
  { 'ChargeFailed' : null } |
  { 'CheckoutCompleted' : null } |
  { 'ChargeConfirmed' : null };
export interface LocalPrice { 'currency' : string, 'amount' : string }
export interface LocalValue { 'currency' : string, 'amount' : string }
export interface Metadata {
  'customer_id' : [] | [string],
  'customer_name' : [] | [string],
}
export interface Payment {
  'transaction_id' : string,
  'from' : Principal,
  'created_at' : bigint,
  'amount' : string,
}
export interface PaymentValue { 'local' : LocalValue, 'crypto' : CryptoValue }
export interface Timeline { 'status' : string, 'time' : string }
export interface WebhookConfig {
  'url' : string,
  'merchant_id' : Principal,
  'secret' : string,
  'enabled' : boolean,
}
export interface WebhookEvent {
  'id' : string,
  'resource' : string,
  'api_version' : string,
  'data' : EventData,
  'created_at' : bigint,
  'event_type' : EventType,
}
export interface WhMetadata {
  'customer_id' : [] | [string],
  'customer_name' : [] | [string],
}
export interface WhPayment {
  'transaction_id' : string,
  'status' : string,
  'value' : PaymentValue,
  'created_at' : string,
  'block_height' : bigint,
}
export type bitcoin_address = string;
export type block_hash = Uint8Array | number[];
export type block_header = Uint8Array | number[];
export type block_height = number;
export interface get_block_headers_response {
  'tip_height' : block_height,
  'block_headers' : Array<block_header>,
}
export interface get_utxos_response {
  'next_page' : [] | [Uint8Array | number[]],
  'tip_height' : number,
  'tip_block_hash' : block_hash,
  'utxos' : Array<utxo>,
}
export type millisatoshi_per_vbyte = bigint;
export type network = { 'mainnet' : null } |
  { 'regtest' : null } |
  { 'testnet' : null };
export interface outpoint { 'txid' : Uint8Array | number[], 'vout' : number }
export type satoshi = bigint;
export type transaction_id = string;
export interface utxo {
  'height' : number,
  'value' : satoshi,
  'outpoint' : outpoint,
}
export interface _SERVICE {
  'create_charge' : ActorMethod<
    [ChargeCreate],
    { 'Ok' : Charge } |
      { 'Err' : string }
  >,
  'create_checkout' : ActorMethod<
    [CheckoutCreate],
    { 'Ok' : Checkout } |
      { 'Err' : string }
  >,
  'get_balance' : ActorMethod<[bitcoin_address], satoshi>,
  'get_block_headers' : ActorMethod<
    [block_height, [] | [block_height]],
    get_block_headers_response
  >,
  'get_charge' : ActorMethod<[string], { 'Ok' : Charge } | { 'Err' : string }>,
  'get_checkout' : ActorMethod<
    [string],
    { 'Ok' : Checkout } |
      { 'Err' : string }
  >,
  'get_current_fee_percentiles' : ActorMethod<[], BigUint64Array | bigint[]>,
  'get_p2pkh_address' : ActorMethod<[], bitcoin_address>,
  'get_p2tr_raw_key_spend_address' : ActorMethod<[], bitcoin_address>,
  'get_p2tr_script_spend_address' : ActorMethod<[], bitcoin_address>,
  'get_utxos' : ActorMethod<[bitcoin_address], get_utxos_response>,
  'get_webhook_config' : ActorMethod<
    [],
    { 'Ok' : WebhookConfig } |
      { 'Err' : string }
  >,
  'list_charges' : ActorMethod<[], Array<Charge>>,
  'process_payment' : ActorMethod<
    [string],
    { 'Ok' : bigint } |
      { 'Err' : string }
  >,
  'register_merchant' : ActorMethod<[], { 'Ok' : null } | { 'Err' : string }>,
  'register_webhook' : ActorMethod<
    [WebhookConfig],
    { 'Ok' : null } |
      { 'Err' : string }
  >,
  'send_from_p2pkh' : ActorMethod<
    [
      {
        'destination_address' : bitcoin_address,
        'amount_in_satoshi' : satoshi,
      },
    ],
    transaction_id
  >,
  'send_from_p2tr_raw_key_spend' : ActorMethod<
    [
      {
        'destination_address' : bitcoin_address,
        'amount_in_satoshi' : satoshi,
      },
    ],
    transaction_id
  >,
  'send_from_p2tr_script_spend' : ActorMethod<
    [
      {
        'destination_address' : bitcoin_address,
        'amount_in_satoshi' : satoshi,
      },
    ],
    transaction_id
  >,
  'verify_webhook' : ActorMethod<
    [string, string],
    { 'Ok' : WebhookEvent } |
      { 'Err' : string }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
