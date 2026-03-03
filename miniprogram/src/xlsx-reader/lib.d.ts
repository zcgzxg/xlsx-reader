/* tslint:disable */
/* eslint-disable */

/**
 * 读取 xlsx 文件，返回包含所有工作表的名称和数据的对象
 */
export function read_xlsx(data: Uint8Array): Map<string, {
    headers: string[] | null,
    rows: (string | number | boolean | undefined)[][]
}>;

export function start(): void;



export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly read_xlsx: (a: number, b: number, c: number) => void;
    readonly start: () => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = WXWebAssembly.BufferSource | WXWebAssembly.Module

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
export default function __wbg_init(path: { path: string } | string): Promise<InitOutput>

