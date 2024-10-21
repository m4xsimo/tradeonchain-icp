export const idlFactory = ({ IDL }) => {
  const ContractSignatories = IDL.Record({
    'seller' : IDL.Tuple(IDL.Principal, IDL.Bool),
    'buyer' : IDL.Tuple(IDL.Principal, IDL.Bool),
  });
  const Contract = IDL.Record({
    'contract_json' : IDL.Text,
    'created_at' : IDL.Nat64,
    'signatories' : ContractSignatories,
  });
  return IDL.Service({
    'create_contract' : IDL.Func(
        [IDL.Text, IDL.Principal, IDL.Principal],
        [IDL.Nat64],
        [],
      ),
    'get_contract' : IDL.Func([IDL.Nat64], [IDL.Opt(Contract)], ['query']),
    'is_signed' : IDL.Func([IDL.Nat64], [IDL.Bool], ['query']),
    'sign_contract' : IDL.Func([IDL.Nat64], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
