import * as fs from "fs";
import Block from "./Block.mjs";

class Blockchain {
    constructor() {
        if (!Blockchain.instance) {
            this.blockchain = [new Block("0", "Genesis Block")];
            Blockchain.instance = this;
        }
        return Blockchain.instance;
    }

    addBlock(data) {
        try {
            data.verifySignature();
        } catch (err) {
            console.log(err.message);
            return;
        }
        const previousBlock = this.blockchain[this.blockchain.length - 1];
        const newBlock = new Block(previousBlock.hash, data);
        this.blockchain.push(newBlock);
    }

    saveBlockchain() {
        fs.writeFileSync(
            "blockchain.json",
            JSON.stringify({ blockchain: this.blockchain })
        );
    }

    displayBlockchainInfo() {
        console.log(this.blockchain);
    }
}

const instance = new Blockchain();
Object.freeze(instance);

export default instance;
