/// <reference types="node" />
import { Buffer } from 'buffer';
import { WebSocket } from 'ws';
export interface ClientOptions {
    endpoint?: string;
    username?: string;
    password?: string;
    isSecure?: boolean;
}
export declare class Client {
    endpoint: string;
    connection: WebSocket;
    subscriptions: Map<string, {
        request: any;
        cursor: string | null;
    }>;
    constructor(options: ClientOptions);
    connect(): Promise<void>;
    disconnect(): Promise<void>;
    send_request(operation: string, params?: {}, options?: {}): Promise<AsyncGenerator<Buffer, void, unknown>>;
    handle_request(id: string): AsyncGenerator<Buffer, void, unknown>;
    ensureConnection(): Promise<void>;
    reconnect_with_backoff(): Promise<void>;
    waitForConnection(timeout?: number): Promise<void>;
    get_status(format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_blocks(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_logs(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_transactions(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_receipts(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_contracts(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_uniswap_v2_pairs(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_uniswap_v2_prices(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_uniswap_v3_pools(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_uniswap_v3_fees(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_uniswap_v3_positions(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_uniswap_v3_prices(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_curve_tokens(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_curve_pools(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_curve_prices(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_transfers(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_erc20_tokens(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_erc20_approvals(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_erc20_transfers(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_fuel_spark_orders(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
    get_fuel_unspent_utxos(params: Object, deltas?: boolean, format?: string): Promise<AsyncGenerator<Buffer, void, unknown>>;
}
