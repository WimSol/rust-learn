const assert = require('assert');
const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

describe('mycalculatordapp', () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const calculator = anchor.web3.Keypair.generate();
    const program = anchor.workspace.Mycalculatordapp;

    it('Creates a calculator', async () => {
        console.log('Calculator public key:', calculator.publicKey.toString());

        try {
            await program.rpc.create("Welcome to Solana", {
                accounts: {
                    calculator: calculator.publicKey,
                    user: provider.wallet.publicKey,
                    systemProgram: SystemProgram.programId
                },
                signers: [calculator]
            });
            console.log('Calculator created successfully');
        } catch (err) {
            console.error('Error during calculator creation:', err);
            throw err;
        }

        try {
            // Fetch the created calculator account
            const account = await program.account.calculator.fetch(calculator.publicKey);
            console.log('Fetched account:', account);

            assert.ok(account.greeting === "Welcome to Solana");
        } catch (err) {
            console.error('Error fetching calculator account:', err);
            throw err;
        }
    });
});
