import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { assert } from "chai";

describe("vault", () => {
  // 1. Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Vault as Program<Vault>;

  // 2. GENERATE RANDOM USER
  // This ensures the PDA (derived from this user's pubkey) is ALWAYS unique.
  // This prevents "Account already in use" errors on Devnet.
  const user = anchor.web3.Keypair.generate();

  // We need to find the PDA for the vault before we can use it
  const [vaultPda, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), user.publicKey.toBuffer()],
    program.programId
  );

  // 3. AIRDROP LOGIC (With Retry)
  before(async () => {
    console.log(`Preparing test for new user: ${user.publicKey.toBase58()}`);
    console.log("Requesting Airdrop...");
    
    try {
      // Airdrop 2 SOL to pay for transactions
      const signature = await provider.connection.requestAirdrop(
        user.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      
      const latestBlockHash = await provider.connection.getLatestBlockhash();
      await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: signature,
      });
      console.log("Airdrop Success!");
    } catch (e) {
      console.error("Airdrop failed. Devnet might be congested.");
      throw e;
    }
  });

  it("Is initialized!", async () => {
    await program.methods
      .initialize()
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user]) // Must sign with the new random keypair
      .rpc();

    const vaultAccount = await program.account.vault.fetch(vaultPda);

    assert.ok(vaultAccount.owner.equals(user.publicKey), "Owner matches");
    assert.ok(vaultAccount.bump === vaultBump, "Bump matches");
    console.log("Vault Initialized at:", vaultPda.toBase58());
  });

  it("Deposits SOL", async () => {
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
    const preBalance = await provider.connection.getBalance(vaultPda);

    await program.methods
      .deposit(amount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const postBalance = await provider.connection.getBalance(vaultPda);
    assert.equal(postBalance, preBalance + amount.toNumber(), "Vault balance incremented by 1 SOL");
  });

  it("Withdraws SOL", async () => {
    const amount = new anchor.BN(0.5 * anchor.web3.LAMPORTS_PER_SOL);
    const preBalance = await provider.connection.getBalance(vaultPda);

    await program.methods
      .withdraw(amount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        owner: user.publicKey, // Explicitly verify owner
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const postBalance = await provider.connection.getBalance(vaultPda);
    assert.equal(postBalance, preBalance - amount.toNumber(), "Vault balance decremented by 0.5 SOL");
  });

  it("Fails to withdraw more than available", async () => {
    const amount = new anchor.BN(10 * anchor.web3.LAMPORTS_PER_SOL);

    try {
      await program.methods
        .withdraw(amount)
        .accounts({
          vault: vaultPda,
          user: user.publicKey,
          owner: user.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user])
        .rpc();
      
      assert.fail("Should have failed due to insufficient funds");
    } catch (error) {
      assert.include(error.message, "InsufficientFunds");
    }
  });
});
