import { createHash } from "crypto";

export default class Block {
    static lastIndex = 0;

    constructor(previous, data) {
        this.index = Block.lastIndex;
        this.timestamp = Date.now();
        this.previous = previous;
        this.data = { ...data };
        this.hash = this.hashBlock();
        Block.lastIndex++;
    }

    hashBlock() {
        const blockData = `${this.index}${this.timestamp}${this.previous}${this.data}`;
        return createHash("sha256").update(blockData).digest("hex");
    }
}
