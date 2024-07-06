import { useEffect, useRef, useState } from "react";
import { Menu, MenuButton, MenuItem } from "@szhsin/react-menu";
import Grid, { GameState } from "./components/grid";
import { createPortal } from "react-dom";
import JSConfetti from "js-confetti";
import AudioPlayer from "./components/audioPlayer";
import { ClockIcon, DownArrowIcon, MusicIcon, RestartIcon } from "./components/icons";
import "./App.css";
import { Body } from "./pages/Body";

const jsConfetti = new JSConfetti();
const initialMute = localStorage.getItem("muted") === null ? true : localStorage.getItem("muted") === "true";

const App = () => {
	return (
		<>
			<Body />
		</>
	)
}

export default App;
