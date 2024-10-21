import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Contract {
  'contract_json' : string,
  'created_at' : bigint,
  'signatories' : ContractSignatories,
}
export interface ContractSignatories {
  'seller' : [Principal, boolean],
  'buyer' : [Principal, boolean],
}
export interface _SERVICE {
  'create_contract' : ActorMethod<[string, Principal, Principal], bigint>,
  'get_contract' : ActorMethod<[bigint], [] | [Contract]>,
  'is_signed' : ActorMethod<[bigint], boolean>,
  'sign_contract' : ActorMethod<[bigint], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
