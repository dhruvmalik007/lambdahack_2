const PORT = process.env.PORT || 5000;
const URL_HOST = process.env.URL_HOST || 'localhost';

// get game id from localstorage, get that id when initiate game

const startGameMethod = async () => {
    const bet = localStorage.getItem('bet');
    const options = {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ bet }),
    };

    try {
        const response = await fetch(`https://${URL_HOST}:${PORT}`, options);
        return response;
    } catch (error) {
        return error;
    }
};

const submitGuessMethod = async () => {
    const bet = localStorage.getItem('bet');
    const options = {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ bet }),
    };

    try {
        const response = await fetch(`https://${URL_HOST}:${PORT}`, options);
        return response;
    } catch (error) {
        return error;
    }
};
