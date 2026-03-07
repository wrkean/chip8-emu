/* tslint:disable */
/* eslint-disable */

export class WasmChip8 {
    free(): void;
    [Symbol.dispose](): void;
    clear_draw_flag(): void;
    draw_flag(): boolean;
    emulate_cycle(): Uint8Array;
    key_down(key: number): void;
    key_up(key: number): void;
    load_rom(rom: Uint8Array): void;
    constructor();
    reset(): void;
    update_timers(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_wasmchip8_free: (a: number, b: number) => void;
    readonly wasmchip8_clear_draw_flag: (a: number) => void;
    readonly wasmchip8_draw_flag: (a: number) => number;
    readonly wasmchip8_emulate_cycle: (a: number) => [number, number];
    readonly wasmchip8_key_down: (a: number, b: number) => void;
    readonly wasmchip8_key_up: (a: number, b: number) => void;
    readonly wasmchip8_load_rom: (a: number, b: number, c: number) => void;
    readonly wasmchip8_new: () => number;
    readonly wasmchip8_reset: (a: number) => void;
    readonly wasmchip8_update_timers: (a: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
