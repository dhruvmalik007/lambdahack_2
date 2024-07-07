import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { createClient, http, parseEther } from 'viem';
import { ethers } from 'ethers'
import { holesky, mainnet, sepolia } from 'viem/chains';


//let ethers = require('../../node_modules/ethers')

// Add the global type declaration
declare global {
    interface Window {
        ethereum?: {
            isMetaMask?: boolean;
            request: (args: { method: string; params?: unknown[] | object }) => Promise<unknown>;
        };
    }
}

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

        try {
            const client = createClient({
                chain: holesky,
                transport: http(),
            });

            const provider = new ethers.BrowserProvider(window.ethereum);
            const signer = await provider.getSigner();

            await provider.send('eth_requestAccounts', []);

            const tx = {
                to: await signer.getAddress(), // Replace with actual recipient address
                value: parseEther("0.01"),
                chainId: 5, // Holesky testnet chain ID
            };

            const transaction = await signer.sendTransaction(tx);
            console.log('Transaction sent:', transaction);

            await transaction.wait();
            console.log('Transaction confirmed:', transaction);

            navigate('/app'); // Replace with your target page
        } catch (error) {
            console.error('Transaction failed:', error);
            alert('Transaction failed. Please try again.');
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div>
            <button onClick={handleTransaction} disabled={isLoading}>
                {isLoading ? 'Processing...' : 'Pay with Crypto'}
            </button>
        </div>
    );
};

export default WalletButton;
