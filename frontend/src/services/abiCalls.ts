const PORT = process.env.PORT || 5000;
const URL_HOST = process.env.URL_HOST || "localhost";

// get game id from localstorage, get that id when initziate game

const callGuessMethod = async () => {
    const guess = await fetch(
        `https://${URL_HOST}:${PORT}`,
        {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                //difficulty: difficulty.name,
                //time: ui.time,
                //flags: ui.flags,
                //mines: difficulty.mines,
            }),
        }
    );
}


const submitGuessMethod = async () => {
    const bet = localStorage.getItem("bet")
    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ bet }),
    }

    try {
        const response = await fetch(`https://${URL_HOST}:${PORT}`, options);
        return response
    } catch (error) {
        return error
    }

}