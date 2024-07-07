import { Routes, Route, Navigate } from "react-router-dom";
import { useEffect } from "react";
import Game from "./Game";
import { Home } from "./Home";
export const Body = () => {

    return (
        <>
            <Routes>
                <Route path="*" element={<Navigate to={"/"} replace />} />
                <Route path="/" element={<Home />} />
                <Route path="/app" element={<Game />} />
            </Routes>

        </>
    );
};