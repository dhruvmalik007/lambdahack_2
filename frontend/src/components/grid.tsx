import { RefObject, useEffect, useRef, useState } from 'react';
import Cell, { CellData } from './cell';

type betArray = [number, number][];
const localStorageKey = 'bet';

const Grid = ({
    mines,
    restartBtn,
    size,
    disabled,
    showMines,
    onUiUpdate,
    onStateUpdate,
    onSoundEvent,
}: GridProps) => {
    const [data, setData] = useState<CellData[][]>([]);
    const isFirstClick = useRef(true);

    const [bet, setBet] = useState<betArray>(() => {
        return [];
    });

    useEffect(() => {
        localStorage.setItem(localStorageKey, JSON.stringify(bet));
    }, [bet]);

    const addBet = (newBet: [number, number]) => {
        setBet((prevBet) => {
            const betExists = prevBet.some(
                (bet) => bet[0] === newBet[0] && bet[1] === newBet[1]
            );
            if (betExists) {
                return prevBet.filter(
                    (bet) => bet[0] !== newBet[0] || bet[1] !== newBet[1]
                );
            } else if (bet.length >= 10) {
                return [...prevBet];
            } else {
                return [...prevBet, newBet];
            }
        });
    };

    const resetBet = () => {
        setBet([]);
        return;
    };

    //Generate the grid for the game
    useEffect(() => {
        isFirstClick.current = true;
        constructGrid(size);
    }, [size]);

    useEffect(() => {
        const resetBoard = () => {
            isFirstClick.current = true;
            constructGrid(size);
        };

        const button = restartBtn.current;
        button?.addEventListener('click', resetBoard);

        return () => button?.removeEventListener('click', resetBoard);
    }, [restartBtn, size]);

    const constructGrid = (size: [number, number]) => {
        const grid: CellData[][] = [];
        for (let y = 0; y < size[0]; y++) {
            grid.push([]);
            for (let x = 0; x < size[1]; x++) {
                grid[y].push({
                    y,
                    x,
                    state: 'hidden',
                    isMine: false,
                    safe: false,
                });
            }
        }

        setData(grid);
    };

    //Mines are placed after the first click to prevent the first click from being a mine

    const handleCellClick = (cell: CellData, button: number) => {
        let newGrid = [...data];

        if (button === 0) {
            if (localStorage.getItem('isRestart') == 'true') {
                localStorage.setItem('isRestart', false.toString());
                resetBet();
            }

            addBet([cell.x, cell.y]);

            if (bet.length === 10) return;

            newGrid[cell.y][cell.x].state =
                cell.state === 'hidden' ? 'flagged' : 'hidden';
            onSoundEvent(cell.state === 'flagged' ? 'flag' : 'unflag');
        }
    };

    return (
        <div className="grid">
            {data.map((row, rowIndex) => {
                return (
                    <div className="row" key={rowIndex}>
                        {row.map((cell, colIndex) => {
                            return (
                                <Cell
                                    state={cell.state}
                                    onClick={(button) =>
                                        handleCellClick(cell, button)
                                    }
                                    key={`${rowIndex}-${colIndex}`}
                                    larger={
                                        size[0] == 10 &&
                                        rowIndex <= 9 &&
                                        colIndex <= 9
                                    }
                                    neighbors={0}
                                    showMine={showMines}
                                    isMine={cell.isMine}
                                />
                            );
                        })}
                    </div>
                );
            })}
        </div>
    );
};

export type GameState = 'playing' | 'won' | 'waiting' | 'lost';

interface GridProps {
    mines: number;
    size: [number, number];
    restartBtn: RefObject<HTMLButtonElement>;
    disabled: boolean;
    showMines: boolean;
    onUiUpdate: (flags: number) => void;
    onStateUpdate: (state: GameState) => void;
    onSoundEvent: (event: 'uncover' | 'flag' | 'unflag') => void;
}

export default Grid;
