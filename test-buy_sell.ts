import DLMM from "@meteora-ag/dlmm"
import { getAssociatedTokenAddress, NATIVE_MINT } from "@solana/spl-token"
import { Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, Transaction, TransactionMessage } from "@solana/web3.js";

export const mainKp = Keypair.fromSecretKey(base58.decode(PRIVATE_KEY!))

const buy = () => {
    const POOL_ID = "";
    const TOKEN_CA = "";
    const solAmount = ;
    const dlmmPool = await DLMM.create(solanaConnection, POOL_ID);
    await swap(dlmmPool, NATIVE_MINT, TOKEN_CA, new BN(solAmount), slippage, mainKp);
}

const buy = () => {
    const POOL_ID = "";
    const TOKEN_CA = "";
    const ata = await getAssociatedTokenAddress(TOKEN_CA, mainKp.publicKey)

    const tokenInfo = await solanaConnection.getTokenAccountBalance(ata);
    const tokenAmount = Number(tokenInfo.value.amount!);
    await swap(dlmmPool, TOKEN_CA, NATIVE_MINT, new BN(tokenAmount), slippage, mainKp);
}

export const swap = async (dlmmPool: DLMM, inputMint: PublicKey, outputMint: PublicKey, buyAmount: BN, slippage: number, user: Keypair) => {
    console.log("🚀 ~ swap ~ buyAmount:", buyAmount)
    try {
        // Swap quote
        let swapYtoX: boolean = false;
        if (inputMint.equals(NATIVE_MINT)) {
            swapYtoX = true;
        }

        const binArrays = await dlmmPool.getBinArrayForSwap(swapYtoX);

        const swapQuote = await dlmmPool.swapQuote(buyAmount, swapYtoX, new BN(slippage), binArrays);
        console.log("🚀 ~ swap ~ swapQuote.outAmount:", Number(swapQuote.outAmount))
        console.log("🚀 ~ swap ~ swapQuote.minOutAmount:", Number(swapQuote.minOutAmount))

        // Swap
        const swapTransaction = await dlmmPool.swap({
            inToken: inputMint,
            outToken: outputMint,
            binArraysPubkey: swapQuote.binArraysPubkey,
            inAmount: buyAmount,
            lbPair: dlmmPool.pubkey,
            user: user.publicKey,
            minOutAmount: swapQuote.minOutAmount,
        });

        swapTransaction.feePayer = user.publicKey;
        swapTransaction.recentBlockhash = (await solanaConnection.getLatestBlockhash()).blockhash
        swapTransaction.lastValidBlockHeight = (await solanaConnection.getLatestBlockhash()).lastValidBlockHeight;

        try {
            const simulationResult = await solanaConnection.simulateTransaction(swapTransaction);
            const { value } = simulationResult;
            console.log("🚀 ~ buy ~ value:", value)
            if (value.err) {
                logger.error("Simulation failed:", value.err);
            } else {
                logger.info("Simulation successful:", value.logs);
            }
        } catch (error: any) {
            throw new Error(`Transaction simulation failed: ${error.message}`)
        }

        const createSig = await sendAndConfirmTransaction(solanaConnection, swapTransaction, [user]);
        console.log(`Create BondingCurve Sig : https://solscan.io/tx/${createSig}`);
    } catch (error) {
        console.log("error => ", error)
    }
}