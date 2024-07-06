import { Routes, Route, Navigate, useNavigate } from "react-router-dom";
import { useEffect } from "react";
import Game from "./Game";
export const Home = () => {

    const navigate = useNavigate()

    return (
        <>
            <button onClick={() => navigate("/app")}> Start Playing</button>
        </>
    );
};