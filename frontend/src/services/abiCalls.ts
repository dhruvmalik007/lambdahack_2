const PORT = "8000"

const callGuessMethod = async () => {
    const guess = await fetch(
        `https://localhost:${PORT}`,
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
