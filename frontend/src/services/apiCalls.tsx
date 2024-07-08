const root = "http://localhost:8000";

export const getTest = async (): Promise<any> => {
    const options = {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        }
    };

    try {
        const response = await fetch(`${root}/`, options);
        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.message || "An error occurred");
        }

        return data;
    } catch (error) {
        return { success: false, message: (error as Error).message };
    }
};