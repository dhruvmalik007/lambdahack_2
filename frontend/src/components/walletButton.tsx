import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { createClient, http, parseEther } from 'viem';
import { ethers } from 'ethers'
import { holesky, mainnet, sepolia } from 'viem/chains';


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

            const provider = new ethers.BrowserProvider(window.ethereum, holesky);
            const signer = await provider.getSigner();
            alert('Connected to Holesky testnet');
            navigate('/app'); 
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
                {isLoading ? 'Processing...' : 'Login'}
            </button>
        </div>
    );
};

export default WalletButton;
