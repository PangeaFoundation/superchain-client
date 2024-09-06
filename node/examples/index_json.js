const { Client } = require('superchain');
const dotenv = require('dotenv');

dotenv.config();

const username = process.env.SUPER_USERNAME;
const password = process.env.SUPER_PASSWORD;
const endpoint = process.env.SUPER_URL || "app.superchain.network";

const client = new Client({ username, password, endpoint });

async function json_stream() {
    const handle = await client.get_blocks({
        chains: ["ETH"],
    }, false, "json_stream");
    for await (const chunk of handle) {
        chunk.toString().split("\n").filter(Boolean).forEach(line => {
            console.log(JSON.parse(line));
        });
    }
}

json_stream()