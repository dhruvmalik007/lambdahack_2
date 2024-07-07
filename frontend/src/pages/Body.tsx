import { Routes, Route, Navigate, useLocation } from 'react-router-dom';
import { useEffect, useState } from 'react';
import Game from './Game';
import { Home } from './Home';

export const Body = () => {
    const location = useLocation();
    const [backgroundSize, setBackgroundSize] = useState('129.5em');
    const [backgroundPositionY, setBackgroundPositionY] = useState('-26em');

    useEffect(() => {
        if (location.pathname === '/') {
            setBackgroundSize('contain');
            setBackgroundPositionY('0');
        } else if (location.pathname === '/app') {
            setBackgroundSize('140em'); // Set to the desired size for the /app route
            setBackgroundPositionY('-30em'); // Set to the desired position for the /app route
        }
    }, [location.pathname]);

    return (
        <>
            <div
                className="container"
                style={{ backgroundSize, backgroundPositionY }}
            >
                <Routes>
                    <Route path="*" element={<Navigate to={'/'} replace />} />
                    <Route path="/" element={<Home />} />
                    <Route path="/app" element={<Game />} />
                </Routes>
            </div>
            <footer>
                v0.1
                <span>
                    <a href="#">Lambda Hackathon</a> -{' '}
                    {new Date().getFullYear()}
                </span>
                <a href="https://github.com/UltimateDoge5/Minesweeper">
                    Source code
                </a>
            </footer>
        </>
    );
};
