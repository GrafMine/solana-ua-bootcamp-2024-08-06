import "dotenv/config" ;
import {
	Connection,
	LAMPORTS_PER_SOL,
	PublicKey,
	clusterApiUrl,
} from "@solana/web3.js" ;

const connection = new Connection(clusterApiUrl("devnet"));
console.log("Connected to devnet");

(async () => {
	const publicKey = new PublicKey("5nuT8k8MKWFsKqEkuRpTFceM23hdQHt4usoLqWfmpMQH") ;
	const balanceInLamports = await connection.getBalance(publicKey);
	
	const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;
	
	console.log(`The balance for the wallet at address ${publicKey} is: ${balanceInSOL}`);
})();
