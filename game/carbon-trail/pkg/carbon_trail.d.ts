/* tslint:disable */
/* eslint-disable */

export function get_all_choice_labels(): string;

export function get_grade(total_co2: number): string;

export function get_industry_avg(): number;

export function get_stage_info(stage_index: number): string;

export function get_total_stages(): number;

export function make_choice(state_json: string, stage_index: number, choice_index: number): string;

export function new_game(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly get_all_choice_labels: () => [number, number];
    readonly get_grade: (a: number) => [number, number];
    readonly get_industry_avg: () => number;
    readonly get_stage_info: (a: number) => [number, number];
    readonly get_total_stages: () => number;
    readonly make_choice: (a: number, b: number, c: number, d: number) => [number, number];
    readonly new_game: () => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
