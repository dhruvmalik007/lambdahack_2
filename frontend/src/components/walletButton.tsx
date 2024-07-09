import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { createClient, http } from 'viem';
import { ethers } from 'ethers';
import { holesky } from 'viem/chains'; // Ensure this import is correct
import { walletService } from "../services/walletService";

declare global {
    interface Window {
        ethereum?: {
            isMetaMask?: boolean;
            request: (args: { method: string; params?: unknown[] | object }) => Promise<unknown>;
        };
    }
    let signer: ethers.Signer;
}

// Define the Holesky network object for ethers.js
const holeskyNetwork = {
    name: 'holesky',
    chainId: 17000,
    _defaultProvider: (providers: any) => new providers.JsonRpcProvider('https://ethereum-holesky-rpc.publicnode.com'),
};

const WalletButton: React.FC = () => {
    const [isLoading, setIsLoading] = useState(false);
    const navigate = useNavigate();

    const handleTransaction = async () => {
        setIsLoading(true);
        if (!window.ethereum) {
            alert('Please install MetaMask or another Ethereum wallet extension');
            setIsLoading(false);
            return;
        }
        await walletService.connectWallet()
        navigate('/app');
    }
    return (
        <div className='login'>
            <button onClick={() => handleTransaction()} disabled={isLoading} >
                {isLoading ? 'Processing...' : 'Login'}
            </button>
        </div>
    );
};

export default WalletButton;
