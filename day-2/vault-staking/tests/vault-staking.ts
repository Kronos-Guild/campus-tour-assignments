import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { VaultStaking } from "../target/types/vault_staking";
import { expect } from "chai";

// CONCEPT: Anchor Testing Framework
// Reference: https://www.anchor-lang.com/docs/testing
// Anchor provides utilities for testing Solana programs locally

describe("vault-staking", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.vaultStaking as Program<VaultStaking>;
  const provider = anchor.getProvider();

  // The wallet that will interact with the program
  const user = provider.wallet as anchor.Wallet;

  // PDA (Program Derived Address) for the vault
  let vaultPda: anchor.web3.PublicKey;
  let vaultBump: number;

  // Constants matching the program
  const MIN_STAKE_DURATION = 60; // seconds
  const REWARD_RATE_BASIS_POINTS = 1000; // 10%
  const BASIS_POINTS_DIVISOR = 10000;

  // Helper function to wait for a specific duration
  const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

  // Helper function to calculate expected reward
  function calculateReward(amount: number): number {
    return Math.floor((amount * REWARD_RATE_BASIS_POINTS) / BASIS_POINTS_DIVISOR);
  }

  before(async () => {
    // CONCEPT: PDA Derivation on Client Side
    // We derive the PDA the same way as in the program
    // Seeds: ["vault", user's public key]
    [vaultPda, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      program.programId
    );

    console.log("\n📋 Test Setup:");
    console.log("   Program ID:", program.programId.toString());
    console.log("   User:", user.publicKey.toString());
    console.log("   Vault PDA:", vaultPda.toString());
    console.log("   Vault Bump:", vaultBump);
  });

  // ==============================================
  // TEST 1: Initialize Vault
  // ==============================================
  it("Initializes a vault for the user", async () => {
    console.log("\n🧪 TEST 1: Initialize Vault");

    // CONCEPT: Calling Instructions
    // program.methods.instructionName() creates a method builder
    // .accounts() specifies required accounts
    // .rpc() sends the transaction and returns the signature
    const tx = await program.methods
      .initialize()
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("   ✅ Transaction signature:", tx);

    // CONCEPT: Fetching Account Data
    // program.account.accountName.fetch() retrieves and deserializes account data
    const vaultAccount = await program.account.vault.fetch(vaultPda);

    // Verify the vault was initialized correctly
    expect(vaultAccount.owner.toString()).to.equal(user.publicKey.toString());
    expect(vaultAccount.bump).to.equal(vaultBump);
    expect(vaultAccount.totalDeposited.toNumber()).to.equal(0);
    expect(vaultAccount.stakedAmount.toNumber()).to.equal(0);
    expect(vaultAccount.stakeTimestamp.toNumber()).to.equal(0);
    expect(vaultAccount.isStaked).to.equal(false);

    console.log("   ✅ Vault initialized successfully");
    console.log("      Owner:", vaultAccount.owner.toString());
    console.log("      Bump:", vaultAccount.bump);
  });

  // ==============================================
  // TEST 2: Deposit SOL
  // ==============================================
  it("Deposits SOL into the vault", async () => {
    console.log("\n🧪 TEST 2: Deposit SOL");

    // Amount to deposit: 2 SOL
    const depositAmount = new BN(2_000_000_000);

    // Get balances before deposit
    const vaultBalanceBefore = await provider.connection.getBalance(vaultPda);
    const userBalanceBefore = await provider.connection.getBalance(user.publicKey);

    console.log("   📊 Before Deposit:");
    console.log("      Vault balance:", vaultBalanceBefore, "lamports");
    console.log("      User balance:", userBalanceBefore, "lamports");

    // Execute deposit
    const tx = await program.methods
      .deposit(depositAmount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("   ✅ Deposit transaction:", tx);

    // Get balances after deposit
    const vaultBalanceAfter = await provider.connection.getBalance(vaultPda);
    const userBalanceAfter = await provider.connection.getBalance(user.publicKey);

    console.log("   📊 After Deposit:");
    console.log("      Vault balance:", vaultBalanceAfter, "lamports");
    console.log("      User balance:", userBalanceAfter, "lamports");

    // Verify vault balance increased by deposit amount
    expect(vaultBalanceAfter - vaultBalanceBefore).to.equal(depositAmount.toNumber());

    // Verify vault account state
    const vaultAccount = await program.account.vault.fetch(vaultPda);
    expect(vaultAccount.totalDeposited.toNumber()).to.equal(depositAmount.toNumber());
    expect(vaultAccount.stakedAmount.toNumber()).to.equal(0);

    console.log("   ✅ Deposit successful!");
    console.log("      Total deposited:", vaultAccount.totalDeposited.toNumber());
  });

  // ==============================================
  // TEST 3: Stake Funds
  // ==============================================
  it("Stakes deposited funds", async () => {
    console.log("\n🧪 TEST 3: Stake Funds");

    // Amount to stake: 1 SOL
    const stakeAmount = new BN(1_000_000_000);

    const vaultAccountBefore = await program.account.vault.fetch(vaultPda);
    console.log("   📊 Before Stake:");
    console.log("      Total deposited:", vaultAccountBefore.totalDeposited.toNumber());
    console.log("      Staked amount:", vaultAccountBefore.stakedAmount.toNumber());
    console.log("      Is staked:", vaultAccountBefore.isStaked);

    // Execute stake
    const tx = await program.methods
      .stake(stakeAmount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        owner: user.publicKey,
      })
      .rpc();

    console.log("   ✅ Stake transaction:", tx);

    // Verify vault account state
    const vaultAccountAfter = await program.account.vault.fetch(vaultPda);
    expect(vaultAccountAfter.stakedAmount.toNumber()).to.equal(stakeAmount.toNumber());
    expect(vaultAccountAfter.isStaked).to.equal(true);
    expect(vaultAccountAfter.stakeTimestamp.toNumber()).to.be.greaterThan(0);

    console.log("   📊 After Stake:");
    console.log("      Staked amount:", vaultAccountAfter.stakedAmount.toNumber());
    console.log("      Is staked:", vaultAccountAfter.isStaked);
    console.log("      Stake timestamp:", vaultAccountAfter.stakeTimestamp.toNumber());
    console.log("   ✅ Staking successful!");
  });

  // ==============================================
  // TEST 4: Unstake Early (No Rewards)
  // ==============================================
  it("Unstakes early without rewards (before 60 seconds)", async () => {
    console.log("\n🧪 TEST 4: Unstake Early (No Rewards)");

    // First, stake some funds
    const stakeAmount = new BN(500_000_000); // 0.5 SOL

    // Unstake the previous stake first
    await program.methods
      .unstake()
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        owner: user.publicKey,
      })
      .rpc();

    // Now stake new amount
    await program.methods
      .stake(stakeAmount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        owner: user.publicKey,
      })
      .rpc();

    const vaultAccountBefore = await program.account.vault.fetch(vaultPda);
    const totalBefore = vaultAccountBefore.totalDeposited.toNumber();

    console.log("   📊 Staked:", stakeAmount.toNumber(), "lamports");
    console.log("   ⏰ Waiting 5 seconds (less than 60 second minimum)...");

    // Wait only 5 seconds (less than minimum 60)
    await sleep(5000);

    // Unstake
    const tx = await program.methods
      .unstake()
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        owner: user.publicKey,
      })
      .rpc();

    console.log("   ✅ Unstake transaction:", tx);

    // Verify no rewards were given
    const vaultAccountAfter = await program.account.vault.fetch(vaultPda);
    const totalAfter = vaultAccountAfter.totalDeposited.toNumber();

    expect(vaultAccountAfter.isStaked).to.equal(false);
    expect(vaultAccountAfter.stakedAmount.toNumber()).to.equal(0);

    // Total should not increase (no rewards)
    expect(totalAfter).to.equal(totalBefore);

    console.log("   ❌ No rewards earned (duration < 60 seconds)");
    console.log("   ✅ Principal returned:", stakeAmount.toNumber());
  });

  // ==============================================
  // TEST 5: Unstake After Duration (With Rewards)
  // ==============================================
  it("Unstakes after 60+ seconds and receives rewards", async () => {
    console.log("\n🧪 TEST 5: Unstake with Rewards (60+ seconds)");

    // Stake amount: 1 SOL
    const stakeAmount = new BN(1_000_000_000);

    // Stake funds
    await program.methods
      .stake(stakeAmount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        owner: user.publicKey,
      })
      .rpc();

    const vaultAccountBefore = await program.account.vault.fetch(vaultPda);
    const totalBefore = vaultAccountBefore.totalDeposited.toNumber();

    console.log("   📊 Staked:", stakeAmount.toNumber(), "lamports");
    console.log("   ⏰ Waiting 61 seconds for rewards eligibility...");
    console.log("      (This test takes a while, please wait)");

    // Wait 61 seconds to meet minimum duration
    await sleep(61000);

    // Calculate expected reward (10% of staked amount)
    const expectedReward = calculateReward(stakeAmount.toNumber());

    console.log("   💰 Expected reward:", expectedReward, "lamports");

    // Unstake
    const tx = await program.methods
      .unstake()
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        owner: user.publicKey,
      })
      .rpc();

    console.log("   ✅ Unstake transaction:", tx);

    // Verify rewards were distributed
    const vaultAccountAfter = await program.account.vault.fetch(vaultPda);
    const totalAfter = vaultAccountAfter.totalDeposited.toNumber();

    expect(vaultAccountAfter.isStaked).to.equal(false);
    expect(vaultAccountAfter.stakedAmount.toNumber()).to.equal(0);

    // Total should increase by reward amount
    const actualReward = totalAfter - totalBefore;
    expect(actualReward).to.equal(expectedReward);

    console.log("   ✅ Rewards received:", actualReward, "lamports");
    console.log("   📊 New total balance:", totalAfter, "lamports");
  });

  // ==============================================
  // TEST 6: Withdraw Unstaked Funds
  // ==============================================
  it("Withdraws unstaked funds from vault", async () => {
    console.log("\n🧪 TEST 6: Withdraw Funds");

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    const availableBalance = vaultAccount.totalDeposited.toNumber();

    // Withdraw half of available balance
    const withdrawAmount = new BN(Math.floor(availableBalance / 2));

    console.log("   📊 Available balance:", availableBalance);
    console.log("   💸 Withdrawing:", withdrawAmount.toNumber());

    const userBalanceBefore = await provider.connection.getBalance(user.publicKey);

    // Execute withdrawal
    const tx = await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("   ✅ Withdraw transaction:", tx);

    const userBalanceAfter = await provider.connection.getBalance(user.publicKey);

    // Verify vault state
    const vaultAccountAfter = await program.account.vault.fetch(vaultPda);
    expect(vaultAccountAfter.totalDeposited.toNumber()).to.equal(
      availableBalance - withdrawAmount.toNumber()
    );

    console.log("   ✅ Withdrawal successful!");
    console.log("   📊 Remaining in vault:", vaultAccountAfter.totalDeposited.toNumber());
  });

  // ==============================================
  // ERROR TESTS
  // ==============================================

  it("Fails to deposit 0 lamports", async () => {
    console.log("\n🧪 TEST: Error - Deposit 0 lamports");

    try {
      await program.methods
        .deposit(new BN(0))
        .accounts({
          vault: vaultPda,
          user: user.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      expect.fail("Should have thrown InvalidAmount error");
    } catch (error: any) {
      expect(error.error.errorCode.code).to.equal("InvalidAmount");
      console.log("   ✅ Correctly rejected: InvalidAmount");
    }
  });

  it("Fails to stake when already staked", async () => {
    console.log("\n🧪 TEST: Error - Double Stake");

    // First stake
    await program.methods
      .stake(new BN(100_000_000))
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        owner: user.publicKey,
      })
      .rpc();

    // Try to stake again (should fail)
    try {
      await program.methods
        .stake(new BN(100_000_000))
        .accounts({
          vault: vaultPda,
          user: user.publicKey,
          owner: user.publicKey,
        })
        .rpc();

      expect.fail("Should have thrown AlreadyStaked error");
    } catch (error: any) {
      expect(error.error.errorCode.code).to.equal("AlreadyStaked");
      console.log("   ✅ Correctly rejected: AlreadyStaked");
    }

    // Cleanup: unstake
    await program.methods
      .unstake()
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        owner: user.publicKey,
      })
      .rpc();
  });

  it("Fails to unstake when not staked", async () => {
    console.log("\n🧪 TEST: Error - Unstake Without Staking");

    try {
      await program.methods
        .unstake()
        .accounts({
          vault: vaultPda,
          user: user.publicKey,
          owner: user.publicKey,
        })
        .rpc();

      expect.fail("Should have thrown NotStaked error");
    } catch (error: any) {
      expect(error.error.errorCode.code).to.equal("NotStaked");
      console.log("   ✅ Correctly rejected: NotStaked");
    }
  });

  it("Fails to withdraw more than available balance", async () => {
    console.log("\n🧪 TEST: Error - Withdraw Exceeding Balance");

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    const availableBalance = vaultAccount.totalDeposited.toNumber();

    // Try to withdraw more than available
    const excessiveAmount = new BN(availableBalance + 1_000_000_000);

    try {
      await program.methods
        .withdraw(excessiveAmount)
        .accounts({
          vault: vaultPda,
          user: user.publicKey,
          owner: user.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      expect.fail("Should have thrown InsufficientFunds error");
    } catch (error: any) {
      expect(error.error.errorCode.code).to.equal("InsufficientFunds");
      console.log("   ✅ Correctly rejected: InsufficientFunds");
    }
  });

  it("Fails to stake more than available balance", async () => {
    console.log("\n🧪 TEST: Error - Stake Exceeding Balance");

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    const availableBalance = vaultAccount.totalDeposited.toNumber();

    // Try to stake more than available
    const excessiveAmount = new BN(availableBalance + 1_000_000_000);

    try {
      await program.methods
        .stake(excessiveAmount)
        .accounts({
          vault: vaultPda,
          user: user.publicKey,
          owner: user.publicKey,
        })
        .rpc();

      expect.fail("Should have thrown InsufficientUnstakedBalance error");
    } catch (error: any) {
      expect(error.error.errorCode.code).to.equal("InsufficientUnstakedBalance");
      console.log("   ✅ Correctly rejected: InsufficientUnstakedBalance");
    }
  });

  // ==============================================
  // SUMMARY
  // ==============================================
  after(async () => {
    console.log("\n" + "=".repeat(50));
    console.log("🎉 ALL TESTS PASSED!");
    console.log("=".repeat(50));

    const vaultAccount = await program.account.vault.fetch(vaultPda);
    console.log("\n📊 Final Vault State:");
    console.log("   Owner:", vaultAccount.owner.toString());
    console.log("   Total Deposited:", vaultAccount.totalDeposited.toNumber(), "lamports");
    console.log("   Staked Amount:", vaultAccount.stakedAmount.toNumber(), "lamports");
    console.log("   Is Staked:", vaultAccount.isStaked);
    console.log("=".repeat(50) + "\n");
  });
});

// CONCEPT: Test Structure Best Practices
//
// 1. Setup (before): Initialize shared state
// 2. Happy Path Tests: Test normal functionality
// 3. Error Tests: Test edge cases and failures
// 4. Cleanup (after): Display summary
//
// Each test should:
// - Have a clear purpose
// - Be independent (can run alone)
// - Test one thing
// - Have clear assertions
// - Log helpful information

// CONCEPT: Why Testing Matters
//
// ✅ Catch bugs before deployment
// ✅ Document expected behavior
// ✅ Prevent regressions
// ✅ Build confidence in code
// ✅ Save time debugging
//
// For production programs:
// - Write tests FIRST (TDD)
// - Test all edge cases
// - Test error conditions
// - Test with different users
// - Fuzz test with random inputs
