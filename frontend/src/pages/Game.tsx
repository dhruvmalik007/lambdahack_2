import { useEffect, useRef, useState } from "react";
import { Menu, MenuButton, MenuItem } from "@szhsin/react-menu";
import Grid, { GameState } from "../components/grid";
import { createPortal } from "react-dom";
import JSConfetti from "js-confetti";
import AudioPlayer from "../components/audioPlayer";
import { ClockIcon, DownArrowIcon, MusicIcon, RestartIcon } from "../components/icons";
import "../App.css";

const jsConfetti = new JSConfetti();
const initialMute = localStorage.getItem("muted") === null ? true : localStorage.getItem("muted") === "true";

const Game = () => {
    const restartBtn = useRef<HTMLButtonElement>(null);
    const timerRef = useRef(0);
    const [difficulty, setDifficulty] = useState<Difficulty>(difficulties[1]);
    const [ui, setUI] = useState({ time: 0, flags: difficulty.mines });
    const [state, setState] = useState<GameState>("waiting");
    const [muted, setMuted] = useState(initialMute);
    const [betAmount, setBetAmount] = useState(0);
    const [fontSize, setFontSize] = useState(parseInt(localStorage.getItem("fontSize") ?? "0"));
    const player = useRef(new AudioPlayer());

    const [remaining, setRemaining] = useState<number>(10);

    // Function to update the remaining count based on localStorage
    const updateBet = () => {
        const bet = localStorage.getItem('bet');
        const betLength = bet ? JSON.parse(bet).length : 0;
        setRemaining(betLength);
        document.getElementById("buttonRemaining").innerHTML = `Bets remaining: ${10 - betLength}`
    };

    const updateBetAmount = (value: any) => {
        setBetAmount(value)
        localStorage.setItem("betAmount", betAmount.toString())
        console.log(betAmount)
    }

    useEffect(() => {
        // Update bet on initial render
        updateBet();

        // Add event listener to update bet when document is clicked
        const handleClick = () => {
            updateBet();
        };
        document.addEventListener('click', handleClick);

        // Clean up the event listener on component unmount
        return () => {
            document.removeEventListener('click', handleClick);
        };
    }, []);

    useEffect(() => {
        //We can ignore the promises
        if (muted) {
            localStorage.setItem("muted", "true");
            player.current.muted = true;
        } else {
            localStorage.setItem("muted", "false");
            player.current.muted = false;
        }
    }, [muted]);

    useEffect(() => {
        if (state === "won") {
            jsConfetti.addConfetti();
            clearInterval(timerRef.current);
            player.current.play("win");
        } else if (state === "lost") {
            clearInterval(timerRef.current);
            player.current.play("gameover");
        } else if (state === "playing") {
            setUI({ time: 0, flags: difficulty.mines });
        } else {
            clearInterval(timerRef.current);
            player.current.stop("gameover");
        }
    }, [state]);

    return (
        <>
            <div className="App">
                <div className="infoBar">
                    <button
                        ref={restartBtn}
                        onClick={() => {
                            setState("waiting");
                            setUI({ time: 0, flags: difficulty.mines });
                            localStorage.setItem("isRestart", true.toString())
                        }}
                    >
                        <RestartIcon />
                        Restart
                    </button>
                    <button>BET</button>
                    <button id="buttonRemaining"></button>
                    <div className="">
                        <input
                            type="number"
                            min="10"
                            onChange={(e) => updateBetAmount(e.target.value)}
                        />
                        <button type="button" onSubmit={/*getBetAmt*/ () => { console.log("bip") }} />
                    </div>
                </div>


                <Grid
                    mines={difficulty.mines}
                    size={difficulty.size}
                    disabled={state === "won" || state === "lost"}
                    showMines={state === "lost"}
                    restartBtn={restartBtn}
                    onUiUpdate={(flags) => setUI({ ...ui, flags: ui.flags + flags })}
                    onStateUpdate={(state) => setState(state)}
                    onSoundEvent={(sound) => player.current.play(sound)}
                />
            </div>

            {(state === "won" || state === "lost") &&
                createPortal(
                    <div className="overlay">
                        <h1>{state === "lost" ? "Game over" : "Well done!"}</h1>

                        {state !== "lost" && <p style={{ fontSize: "1.8em" }}>You won in {ui.time} seconds</p>}
                        <span style={{ marginBottom: "8px" }}>Click the restart button to {state === "lost" ? "Restart" : "Play again"}</span>
                        <button onClick={() => restartBtn.current?.click()}>{state === "lost" ? "Restart" : "Play again"}</button>
                    </div>,
                    document.querySelector(".grid") as Element
                )}

            <div
                style={{
                    position: "absolute",
                    display: "flex",
                    justifyContent: "center",
                    gap: "8px",
                    top: "8px",
                    right: "8px",
                    transition: "all 0.3s ease"
                }}
            >
                <button className="musicBtn" onClick={() => setMuted(!muted)}>
                    <MusicIcon muted={muted} />
                </button>

            </div>

            <footer>
                v0.1
                <span>
                    <a href="#">Lambda Hackathon</a> - {new Date().getFullYear()}
                </span>
                <a href="https://github.com/UltimateDoge5/Minesweeper">Source code</a>
            </footer>
        </>

    );
};
const capitalize = (str: string) => str[0].toUpperCase() + str.slice(1);

const difficulties: Difficulty[] = [
    { size: [10, 10], mines: 10, name: "beginner" },
    { size: [10, 10], mines: 40, name: "intermediate" },
    { size: [10, 30], mines: 99, name: "expert" }
];

interface Difficulty {
    size: [number, number];
    mines: number;
    name: string;
}

export default Game;
