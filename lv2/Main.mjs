import blockchain from "./Blockchain.mjs";
import Wallet from "./Wallet.mjs";
import Transaction from "./Transaction.mjs";

const treasury = new Wallet("Treasury");
const professorWallet = new Wallet("Professor");
const studentWallet = new Wallet("Student");
const cryptoWhaleWallet = new Wallet("CryptoWhale");
const invalidWallet = new Wallet("InvalidWallet");

function createAndSignTransaction(senderWallet, toAddress, amount) {
    let transaction = new Transaction(
        senderWallet.publicKey,
        toAddress,
        amount
    );
    if (senderWallet.name !== "Treasury") {
        senderWallet.signTransaction(transaction);
    }
    return transaction;
}
function createAndInvalidSignTransaction(senderWallet, toAddress, amount) {
    let transaction = new Transaction(
        senderWallet.publicKey,
        toAddress,
        amount
    );
    if (senderWallet.name !== "Treasury") {
        invalidWallet.signTransaction(transaction);
    }
    return transaction;
}

blockchain.addBlock(
    createAndSignTransaction(treasury, professorWallet.publicKey, 40)
);
blockchain.addBlock(
    createAndSignTransaction(treasury, studentWallet.publicKey, 40)
);
blockchain.addBlock(
    createAndSignTransaction(treasury, cryptoWhaleWallet.publicKey, 40)
);

console.log("WALLET AMOUNTS AFTER TREASURY TRANSFER:\n");
professorWallet.displayWalletInfo();
studentWallet.displayWalletInfo();
cryptoWhaleWallet.displayWalletInfo();

blockchain.addBlock(
    createAndSignTransaction(professorWallet, studentWallet.publicKey, 10)
);

console.log("WALLET AMOUNTS AFTER PROFFESOR TO STUDENT 10:\n");
professorWallet.displayWalletInfo();
studentWallet.displayWalletInfo();

blockchain.addBlock(
    createAndSignTransaction(studentWallet, cryptoWhaleWallet.publicKey, 30)
);

console.log("WALLET AMOUNTS AFTER STUDENT TO CRYPTOWHALE 30:\n");
studentWallet.displayWalletInfo();
cryptoWhaleWallet.displayWalletInfo();

blockchain.addBlock(
    createAndSignTransaction(cryptoWhaleWallet, professorWallet.publicKey, 10)
);

console.log("WALLET AMOUNTS AFTER CRYPTOWHALE TO PROFESSOR 10:\n");
professorWallet.displayWalletInfo();
cryptoWhaleWallet.displayWalletInfo();

console.log("WALLET AMOUNTS AFTER 2 TRIES OF INVALID TRANSACTIONS:\n");
blockchain.addBlock(
    createAndSignTransaction(professorWallet, studentWallet.publicKey, 100)
);
blockchain.addBlock(
    createAndInvalidSignTransaction(
        professorWallet,
        studentWallet.publicKey,
        10
    )
);

professorWallet.displayWalletInfo();
studentWallet.displayWalletInfo();
cryptoWhaleWallet.displayWalletInfo();

console.log("BLOCKCHAIN INFO AVAILABLE IN blockchain.json");
blockchain.saveBlockchain();
