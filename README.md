
![NFT MARKETPLACE](./Screenshot%202025-11-18%20161759.png)




# **Aether Exchange — NFT Escrow Marketplace (Solana / Anchor)**

Aether Exchange is a secure, trust-minimized NFT escrow marketplace built on the Solana blockchain using the Anchor framework.
It provides a fully on-chain mechanism for listing, purchasing, and unlisting NFTs through program-owned escrow accounts.
The system eliminates the risks of manual peer-to-peer transfers by enforcing atomic and deterministic trade logic.

---

## **Overview**

The project addresses the inefficiencies and security risks associated with manual NFT trading, where users typically rely on informal agreements or third parties.
Aether Exchange ensures that:

* Sellers can safely list NFTs for sale.
* Buyers can securely purchase listed NFTs without counterparty risk.
* Sellers retain full control and may cancel listings at any time.
* All asset movements occur through program-derived addresses controlled solely by the program logic.

---

## **Features**

### **Initialize Escrow**

Creates an escrow state account to store relevant listing metadata, including seller information, NFT mint address, price, and status flags.

### **List**

Transfers the seller’s NFT into a program-owned escrow account and marks the listing as available for purchase.

### **Buy**

Executes an atomic asset exchange:

* The buyer transfers the specified amount of SOL to the seller.
* The program transfers the NFT from escrow to the buyer’s token account.

Both steps succeed or fail together, ensuring transactional integrity.

### **Unlist**

Allows the seller to cancel the listing and reclaim the NFT from the program escrow account.

---

## **Instruction Set**

| Instruction  | Description                                       |
| ------------ | ------------------------------------------------- |
| `initialize` | Creates and initializes the escrow state account. |
| `list`       | Moves the NFT into escrow and marks it as listed. |
| `buy`        | Processes the atomic SOL-for-NFT exchange.        |
| `unlist`     | Releases the NFT from escrow back to the seller.  |

---

## **Program Functions**

| Function       | Purpose                                                         |
| -------------- | --------------------------------------------------------------- |
| `initialize()` | Allocates and initializes the escrow account on-chain.          |
| `list()`       | Escrows the NFT and records listing details.                    |
| `buy()`        | Transfers SOL to the seller and transfers the NFT to the buyer. |
| `unlist()`     | Cancels the listing and returns the NFT to the seller.          |

---

## **Architecture**

Aether Exchange is built on the following components:

* **Program-Derived Addresses (PDAs)** for secure, signer-less control over escrowed assets.
* **SPL Token Program** for all token transfers.
* **Metaplex Core** is adopted as the NFT standard.
* **Anchor account types and constraints** ensuring strict validation and controlled state transitions.
* **Deterministic and verifiable account ownership** preventing unauthorized access or modification.

---

## **Security Considerations**

* NFTs remain locked under a program-derived address until purchase or cancellation.
* Buyers cannot receive the NFT without paying the listed price in full.
* Sellers cannot withdraw assets after a purchase has been executed.
* All state transitions and asset flows are governed by explicit account constraints.
* Escrow accounts are derived deterministically to prevent collisions or tampering.

---

## **Potential Extensions**

* Marketplace fee structure
* Royalty processing via Metaplex metadata
* Offer (bid) functionality
* Batch listings and batch purchases
* Indexer integration for search and discovery
* Administrative controls for program governance

---

## **License**

This project is released under the MIT License.

