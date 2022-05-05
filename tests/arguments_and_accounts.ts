import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ArgumentsAndAccounts } from "../target/types/arguments_and_accounts";

const { Keypair, SystemProgram } = anchor.web3;

describe("arguments_and_accounts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // The program to execute.
  const program = anchor.workspace
    .ArgumentsAndAccounts as Program<ArgumentsAndAccounts>;

  // The Account to create.
  const myAccount = Keypair.generate();

  it("Is initialized!", async () => {
    // Create the new account and initialize it with the program.
    const tx = await program.methods
      .initialize(new anchor.BN(1234))
      .accounts({
        myAccount: myAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([myAccount])
      .rpc();

    console.log("Your transaction signature", tx);

    /* NOTE: The signers array is a list of all Signer objects 
       needed to sign the transaction. Because myAccount is being 
       created, the Solana runtime requires it to sign the 
       transaction */
  });
});
