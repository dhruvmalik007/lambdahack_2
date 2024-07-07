declare global {
    interface Window {
        ethereum?: {
            isMetaMask?: boolean;
            request: (args: { method: string; params?: unknown[] | object }) => Promise<unknown>;
        };
    }
}
