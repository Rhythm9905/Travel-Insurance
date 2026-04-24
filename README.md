# Travel-Insurance
#  Smart Travel Insurance on Stellar (Soroban)

##  Project Description
This project is a decentralized travel insurance system built using Soroban smart contracts on the Stellar blockchain. It allows users to purchase travel insurance policies and receive automated claim payouts when predefined conditions (like flight delays or cancellations) are met.

The goal is to eliminate manual claim processing, reduce fraud, and ensure fast, transparent payouts using blockchain technology.

---

##  What It Does
- Users can purchase a travel insurance policy by paying a premium.
- The contract stores policy details securely on-chain.
- When a valid travel disruption occurs (verified off-chain or via oracle), users can trigger a claim.
- The contract automatically processes and records payouts.
- Ensures claims are only paid once per policy.

---

##  Features

###  Policy Purchase
- Users can buy insurance policies by submitting a premium.
- Policy details are stored securely on-chain.

###  Secure Authentication
- Uses Stellar’s address-based authentication (`require_auth`) to ensure only the policyholder can act.

###  Policy Status Tracking
- Check whether a policy is active.
- Track whether a claim has already been processed.

###  Automated Claim Payouts
- Claims are processed instantly when triggered.
- Prevents duplicate claims using smart contract logic.

###  Event Logging
- Emits blockchain events when claims are processed.
- Enables transparency and easy tracking.

###  Single-Use Claim Protection
- Ensures each policy can only be claimed once.

---

##  Future Enhancements
- Integration with real-world data oracles (e.g., flight APIs).
- Dynamic premium calculation based on risk.
- Multi-tier insurance plans.
- Token-based payouts using Stellar assets.
- Admin/oracle role for automated claim validation.

---

##  Tech Stack
- **Soroban SDK (Rust)**
- **Stellar Blockchain**
- Smart Contract Storage & Events

---

##  Use Case
Ideal for travelers who want quick, transparent insurance payouts without dealing with traditional insurance claim delays.

---

##  Note
This is a basic prototype for learning and demonstration purposes. Real-world deployment should include:
- Oracle integration
- Token transfer logic
- Security audits

**Wallet Address**:GAAMCQBRTMA26OV6IS6PPSO7ZMEUB5JOVKLMCPK4GASNBVFTHPLTNFCQ

**Contract Address**:CD3VWBJQ2RW27CJ2YV2CQJ4FIDKRC46LLR2GCS7H55A2KEJ2SWCEY4RS

https://stellar.expert/explorer/testnet/contract/CD3VWBJQ2RW27CJ2YV2CQJ4FIDKRC46LLR2GCS7H55A2KEJ2SWCEY4RS

<img width="1897" height="902" alt="image" src="https://github.com/user-attachments/assets/1998ee29-0e3e-400c-a971-e8b92dd3f26e" />
