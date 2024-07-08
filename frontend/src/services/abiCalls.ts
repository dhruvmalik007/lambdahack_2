const PORT = process.env.PORT || 5000;
const URL_HOST = process.env.URL_HOST || "localhost";

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
