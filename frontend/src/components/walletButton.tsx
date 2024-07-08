import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { createClient, http } from 'viem';
import { ethers } from 'ethers';
import { holesky } from 'viem/chains'; // Ensure this import is correct

declare global {
    interface Window {
        ethereum?: {
            isMetaMask?: boolean;
            request: (args: { method: string; params?: unknown[] | object }) => Promise<unknown>;
        };
    }
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

        try {
            const client = createClient({
                chain: holesky,
                transport: http(),
            });

            // Use the properly defined Holesky network object here
            const provider = new ethers.BrowserProvider(window.ethereum, holeskyNetwork);
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
