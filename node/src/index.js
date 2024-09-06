"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
Object.defineProperty(exports, "Client", {
    enumerable: true,
    get: function() {
        return Client;
    }
});
const _uuid = require("uuid");
const _buffer = require("buffer");
const _ws = require("ws");
function applyDefaults(options) {
    return {
        endpoint: options.endpoint || 'app.superchain.network',
        username: options.username,
        password: options.password,
        isSecure: options.isSecure === undefined ? true : options.isSecure
    };
}
let Client = class Client {
    async connect() {
        try {
            if (!this.connection || this.connection.readyState === _ws.WebSocket.CLOSED) {
                this.connection = new _ws.WebSocket(this.endpoint);
            }
            await this.waitForConnection();
        } catch (error) {
            console.error('WebSocket connection error:', error);
            throw error;
        }
    }
    async disconnect() {
        if (this.connection) {
            this.connection?.close();
        }
    }
    async send_request(operation, params = {}, options = {}) {
        await this.ensureConnection(); // Ensure connection before sending
        const id = (0, _uuid.v4)();
        const request = {
            id,
            cursor: null,
            operation,
            ...options,
            ...params
        };
        // Add cursor if present
        if (this.subscriptions.has(id)) {
            request.cursor = this.subscriptions.get(id)?.cursor;
        } else {
            this.subscriptions.set(id, {
                request,
                cursor: null
            }); // Initialize subscription state with null cursor
        }
        this.connection?.send(JSON.stringify(request));
        return this.handle_request(id);
    }
    async *handle_request(id) {
        const queue = [];
        // Register a message event listener
        this.connection?.on('message', async (raw_data)=>{
            if (!raw_data) {
                return;
            }
            let data = _buffer.Buffer.from([]);
            if (raw_data instanceof ArrayBuffer) {
                data = _buffer.Buffer.from(raw_data);
            } else if (raw_data instanceof _buffer.Buffer) {
                data = raw_data;
            } else if (typeof raw_data === 'string') {
                data = _buffer.Buffer.from(raw_data);
            } else if (Array.isArray(raw_data)) {
                data = _buffer.Buffer.concat(raw_data);
            } else if (data instanceof Blob) {
                let buffer = await data.arrayBuffer();
                data = _buffer.Buffer.from(buffer);
            }
            const newlineIndex = data.indexOf('\n');
            if (newlineIndex === -1) {
                return;
            }
            const headerJSON = data.subarray(0, newlineIndex).toString();
            const body = data.subarray(newlineIndex + 1); // Preserving the body as bytes
            const header = JSON.parse(headerJSON);
            if (header.id === _uuid.NIL && header.kind === 'Error') {
                throw new Error(body.toString());
            }
            if (header.id !== id) return;
            queue.push({
                header,
                body
            });
        });
        while(true){
            if (queue.length === 0) {
                await new Promise((resolve)=>setTimeout(resolve, 50));
                continue;
            }
            const item = queue.shift();
            if (!item) {
                continue;
            }
            const { header, body } = item;
            if (header?.kind && header.kind.startsWith('Continue') && item.header.cursor && this.subscriptions.get(id)) {
                this.subscriptions.get(id).cursor = item.header.cursor;
            }
            if (header.kind === 'Start') {
                continue;
            } else if ([
                'Continue',
                'ContinueWithError'
            ].includes(header.kind)) {
                yield body;
            } else if (header.kind === 'Error') {
                throw new Error(body.toString());
            } else if (header.kind === 'End') {
                break;
            } else {
                throw new Error(`Unexpected kind of response from server: ${header.kind}`);
            }
        }
    }
    async ensureConnection() {
        while(this.connection && this.connection.readyState === _ws.WebSocket.CONNECTING){
            await new Promise((resolve)=>setTimeout(resolve, 100));
        }
        if (this.connection && this.connection.readyState === _ws.WebSocket.OPEN) {
            return;
        }
        console.log('Connection lost. Attempting to reconnect...');
        try {
            await this.connect();
        } catch (error) {
            // Trigger reconnection attempt
            await this.reconnect_with_backoff();
        }
    }
    async reconnect_with_backoff() {
        let backoffSeconds = 1;
        const MAX_BACKOFF_SECONDS = 60;
        while(true){
            try {
                await this.connect();
                // Resubscribe after successful connection
                for (const [id, subscription] of this.subscriptions.entries()){
                    await this.send_request(subscription.request.operation, subscription.request, {
                        deltas: subscription.request.deltas,
                        format: subscription.request.format
                    });
                }
                backoffSeconds = 1; // Reset backoff on successful reconnection
                return;
            } catch (error) {
                console.error(`Reconnection failed: ${error}. Retrying in ${backoffSeconds} seconds...`);
                await new Promise((resolve)=>setTimeout(resolve, backoffSeconds * 1000));
                backoffSeconds = Math.min(backoffSeconds * 2, MAX_BACKOFF_SECONDS);
            }
        }
    }
    async waitForConnection(timeout = 5000) {
        return new Promise((resolve, reject)=>{
            if (this.connection?.readyState === _ws.WebSocket.OPEN) {
                resolve();
                return;
            }
            const onOpen = ()=>{
                clearTimeout(timeoutId);
                resolve();
                this.connection?.removeEventListener('open', onOpen);
            };
            const onError = (error)=>{
                console.log('WebSocket connection error:', error);
                clearTimeout(timeoutId);
                reject(error);
                this.connection?.removeEventListener('error', onError);
            };
            const timeoutId = setTimeout(()=>{
                reject(new Error('WebSocket connection timed out'));
                this.connection?.removeEventListener('open', onOpen);
                this.connection?.removeEventListener('error', onError);
            }, timeout);
            this.connection?.addEventListener('open', onOpen);
            this.connection?.addEventListener('error', onError);
        });
    }
    async get_status(format = 'json_stream') {
        return await this.send_request('getStatus', {
            format
        });
    }
    async get_blocks(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getBlocks', params, {
            deltas,
            format
        });
    }
    async get_logs(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getLogs', params, {
            deltas,
            format
        });
    }
    async get_transactions(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getTxs', params, {
            deltas,
            format
        });
    }
    async get_receipts(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getReceipts', params, {
            deltas,
            format
        });
    }
    async get_contracts(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getContracts', params, {
            deltas,
            format
        });
    }
    async get_uniswap_v2_pairs(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getUniswapV2Pairs', params, {
            deltas,
            format
        });
    }
    async get_uniswap_v2_prices(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getUniswapV2Prices', params, {
            deltas,
            format
        });
    }
    async get_uniswap_v3_pools(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getUniswapV3Pools', params, {
            deltas,
            format
        });
    }
    async get_uniswap_v3_fees(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getUniswapV3Fees', params, {
            deltas,
            format
        });
    }
    async get_uniswap_v3_positions(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getUniswapV3Positions', params, {
            deltas,
            format
        });
    }
    async get_uniswap_v3_prices(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getUniswapV3Prices', params, {
            deltas,
            format
        });
    }
    async get_curve_tokens(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getCurveTokens', params, {
            deltas,
            format
        });
    }
    async get_curve_pools(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getCurvePools', params, {
            deltas,
            format
        });
    }
    async get_curve_prices(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getCurvePrices', params, {
            deltas,
            format
        });
    }
    async get_transfers(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getTransfers', params, {
            deltas,
            format
        });
    }
    async get_erc20_tokens(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getErc20', params, {
            deltas,
            format
        });
    }
    async get_erc20_approvals(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getErc20Approvals', params, {
            deltas,
            format
        });
    }
    async get_erc20_transfers(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getErc20Transfers', params, {
            deltas,
            format
        });
    }
    async get_fuel_spark_orders(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getSparkOrder', params, {
            deltas,
            format
        });
    }
    async get_fuel_unspent_utxos(params, deltas = false, format = 'json_stream') {
        return await this.send_request('getUnspentUtxos', params, {
            deltas,
            format
        });
    }
    constructor(options){
        options = applyDefaults(options);
        let endpoint = options.endpoint;
        if (options.username && options.password) {
            endpoint = `${options.username}:${options.password}@${endpoint}`;
        }
        this.endpoint = options.isSecure ? `wss://${endpoint}/v1/websocket` : `ws://${endpoint}/v1/websocket`;
        this.subscriptions = new Map();
        this.connection = new _ws.WebSocket(this.endpoint);
        this.connect();
    }
};

//# sourceMappingURL=index.js.map