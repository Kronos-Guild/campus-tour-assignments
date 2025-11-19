// No imports needed in SolPG!

describe("vault", () => {
  // In SolPG, the program is automatically loaded into 'pg.program'
  const program = pg.program;
  
  // We keep the "Random User" logic so you don't hit "Already Initialized" errors
  const user = web3.Keypair.generate();

  const [vaultPda, vaultBump] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), user.publicKey.toBuffer()],
    program.programId
  );

  it("Setup: Airdrop SOL", async () => {
    // SolPG makes airdrops easier
    const sig = await pg.connection.requestAirdrop(user.publicKey, 2 * web3.LAMPORTS_PER_SOL);
    await pg.connection.confirmTransaction(sig);
    console.log("Airdropped 2 SOL to random user");
  });

  it("Is initialized!", async () => {
    await program.methods
      .initialize()
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    
    // Assertions in SolPG
    assert(vaultAccount.owner.equals(user.publicKey), "Owner matches");
    assert(vaultAccount.bump === vaultBump, "Bump matches");
    console.log("Vault Initialized at:", vaultPda.toBase58());
  });

  it("Deposits SOL", async () => {
    const amount = new BN(1 * web3.LAMPORTS_PER_SOL);
    
    const preBalance = await pg.connection.getBalance(vaultPda);

    await program.methods
      .deposit(amount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const postBalance = await pg.connection.getBalance(vaultPda);
    
    assert(postBalance === preBalance + amount.toNumber(), "Vault balance incremented by 1 SOL");
    console.log("Deposit successful");
  });

  it("Withdraws SOL", async () => {
    const amount = new BN(0.5 * web3.LAMPORTS_PER_SOL);
    
    const preBalance = await pg.connection.getBalance(vaultPda);

    await program.methods
      .withdraw(amount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        // owner field is removed in our optimized struct, relying on signer
        // if your struct still has 'owner', add it back here
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const postBalance = await pg.connection.getBalance(vaultPda);
    
    assert(postBalance === preBalance - amount.toNumber(), "Vault balance decremented by 0.5 SOL");
    console.log("Withdraw successful");
  });
});
