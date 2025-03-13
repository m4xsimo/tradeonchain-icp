# `icp-buyer-seller-contract`

This backend handles:
- the secure storage and management of contracts with their lifecycle (e.g., draft, created, signed, paid, shipped, delivered, completed), ensuring that they are immutable and tamper-proof;
- the authentication and identity verification of all users interacting with the platform allowing users to sign contracts and access the platform without needing traditional login mechanisms;
- manage secure payments between buyer and seller through the balance of crypto. More in details, the backend canister own a wallet on Etherum blockchain to securely store USDC crypto, and interacts with this wallet to allow secure off-ramp transactions sent and authenticated by frontend server.

Functionality:
- Secure storage and management of contracts.
- State management of contracts, reflecting the progress from creation to fulfilment.
- Signature verification and validation via Internet Identity.
- Allow secure off-ramp transactions sent and authenticated by frontend server.

## Running the project locally

If you want to test the project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background --clean

# Deploys the canister to the replica and generates your candid interface
dfx deploy
```

Once the job completes, the canister will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

## Running the project on mainnet

If you want to deploy the project on ICP mainnet, you can use the following commands:

```bash
# Build the canister for the mainnet
dfx build --ic
# Install the canister on the mainnet (erasing previous data)
dfx canister install --mode reinstall --ic icp-buyer-seller-contract-backend
# Upgrade the canister on the mainnet (maintening previous data )
dfx canister install --mode upgrade --ic icp-buyer-seller-contract-backend
```

Once the job completes, the canister will be available at url that will be shown.