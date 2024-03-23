import { createSign, createVerify, generateKeyPairSync } from "crypto";
import Blockchain from "./Blockchain.mjs";

export default class Wallet {
    constructor(name) {
        this.name = name;
        if (name === "Treasury") {
            this.publicKey = "Treasury";
            this.privateKey = "Treasury";
        } else {
            const { publicKey, privateKey } = generateKeyPairSync("ec", {
                namedCurve: "secp256k1",
                publicKeyEncoding: {
                    type: "spki",
                    format: "pem",
                },
                privateKeyEncoding: {
                    type: "pkcs8",
                    format: "pem",
                    cipher: "aes-256-cbc",
                    passphrase: "",
                },
            });
            this.privateKey = privateKey;
            this.publicKey = publicKey;
        }
    }

    signTransaction(transaction) {
        const sign = createSign("SHA256");
        sign.update(transaction.toString()).end();
        const signature = sign.sign(
            {
                key: this.privateKey,
                passphrase: "",
            },
            "hex"
        );
        transaction.signature = signature;
    }

    static calculateBalance(publicKey) {
        let balance = 0;
        for (let i = 0; i < Blockchain.blockchain.length; i++) {
            if (Blockchain.blockchain[i].data.toAddress === publicKey)
                balance += Blockchain.blockchain[i].data.amount;
            else if (Blockchain.blockchain[i].data.fromAddress === publicKey)
                balance -= Blockchain.blockchain[i].data.amount;
        }
        return balance;
    }

    displayWalletInfo() {
        console.log(
            `\n-----------------------------------------------------------
            \nWallet ${this.name} balance: ${Wallet.calculateBalance(
                this.publicKey
            )}
            \nWallet ${this.name} public key: ${this.publicKey}`
        );
    }
}
