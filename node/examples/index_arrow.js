const { Client } = require('superchain');
const dotenv = require('dotenv');
const { tableFromIPC } = require('apache-arrow');

dotenv.config();

const username = process.env.SUPER_USERNAME;
const password = process.env.SUPER_PASSWORD;
const endpoint = process.env.SUPER_URL || "app.superchain.network";

const client = new Client({ username, password, endpoint });

async function arrow() {
    const handle = await client.get_blocks({
        chains: ["ETH"],
    }, false, "arrow");
    for await (const chunk of handle) {
        const table = await tableFromIPC(chunk);
        console.table([...table]);
    }
}

arrow()
