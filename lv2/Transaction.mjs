import { createVerify } from "crypto";
import Wallet from "./Wallet.mjs";

export default class Transaction {
    constructor(fromAddress, toAddress, amount) {
        this.fromAddress = fromAddress;
        this.toAddress = toAddress;
        this.amount = amount;
        this.signature = null;
    }

    toString() {
        return JSON.stringify({
            fromAddress: this.fromAddress,
            toAddress: this.toAddress,
            amount: this.amount,
        });
    }

    verifySignature() {
        if (this.fromAddress === "Treasury") {
            return;
        }

        if (!this.fromAddress || !this.signature) {
            throw new Error("No signature in this transaction!");
        }

        const verify = createVerify("SHA256");
        verify.update(this.toString());
        if (!verify.verify(this.fromAddress, this.signature, "hex")) {
            throw new Error("Invalid signature!");
        }

        if (Wallet.calculateBalance(this.fromAddress) < this.amount) {
            throw new Error("Issuficient balance to complete transaction!");
        }
    }
}
