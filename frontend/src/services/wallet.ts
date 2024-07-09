import { ethers } from 'ethers';
import {ContractABI, ContractAddress} from "./contract";

// define a class of wallet service

const ETH_RPC_URL = process.env.ETH_RPC_URL || 'https://ethereum-holesky-rpc.publicnode.com' ;

class WalletService {
    // define the network object
    private network = {
        name: 'holesky',
        chainId: 17000,
        _defaultProvider: (providers: any) => new providers.JsonRpcProvider(ETH_RPC_URL),
    };
    private signer: ethers.Signer;

    // define a method to connect to the wallet
    public async connectWallet(): Promise<null> {
        if (!window.ethereum) {
            throw new Error('Please install MetaMask or another Ethereum wallet extension');
        }

        // Use the properly defined Holesky network object here
        const provider = new ethers.BrowserProvider(window.ethereum, this.network);

        // set the signer
        this.signer = await provider.getSigner();
        return null;
    }

    // define a method to do the transaction
    public async doTransaction(): Promise<null> {
        if (!this.signer) {
            throw new Error('Please connect to the wallet first');
        }

        // do the transaction
        const contract = new ethers.Contract(ContractAddress, ContractABI, this.signer);

        // Call a function from the contract (replace 'myFunction' and 'args' with your function and its arguments)
        const data = contract.interface.encodeFunctionData('', ['arg1', 'arg2', ...]);

        const tx = await this.signer.sendTransaction({
            to: ContractAddress,
            data: data,
        });
}
