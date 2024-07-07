import { Routes, Route, Navigate, useNavigate } from "react-router-dom";
import { createPublicClient, http } from 'viem'
import { mainnet } from 'viem/chains'
import WalletButton from "../components/walletButton";

const client = createPublicClient({
    chain: mainnet,
    transport: http(),
})
export const Home = () => {

    const navigate = useNavigate()

    return (
        <>
            <WalletButton />
        </>
    );
};