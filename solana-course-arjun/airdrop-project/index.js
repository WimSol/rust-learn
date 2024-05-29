const {
    Connection,
    PublicKey,
    clusterApiUrl,
    Keypair,
    LAMPORTS_PER_SOL,
} = require("@solana/web3.js")

const wallet = new Keypair()

const publicKey = new PublicKey(wallet._keypair.publicKey)
const secretKey = wallet._keypair.secretKey

console.log("Public Key:", publicKey)
console.log("Secret Key:", secretKey)


const getWalletBalance = async() => {
    try {
        const connection = new Connection(clusterApiUrl('devnet'), 'confirmed')
        const walletBalance = await connection.getBalance(publicKey)
        console.log(`Wallet balance: ${walletBalance / LAMPORTS_PER_SOL}`)
        
    } catch(err) {
        console.error(err)
    }
}

const airDropSol = async() => {
    try {
        const connection = new Connection(clusterApiUrl('devnet'), 'confirmed')
        const fromAirdropSignature = await connection.requestAirdrop(new PublicKey(publicKey), 2 * LAMPORTS_PER_SOL)
        const latestBlockHash = await connection.getLatestBlockhash()

        await connection.confirmTransaction({
            signature: fromAirdropSignature,
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight
        })

        console.log('Airdrop successful!')
    } catch(err) {
        console.error(err)
    }
}
const main = async() => {
    await airDropSol()
    await getWalletBalance()

}
main()