# Tier 1 Debugging Capabilities — Requirements

Scope: an in-emulator instruction **tracer** and a **disassembler**, plus their integration into
`dmg01`. Goal: make it possible to see what the CPU is doing at any moment, instead of guessing
from raw hex dumps.

Reference points in the current code:

- `cpu.step()` — `src/cpu.rs:663` (single chokepoint for the trace hook)
- `handle_interrupts` / `service_interrupt` — `src/cpu.rs:689-715` (interrupt logging hook)
- `Registers` accessors — `src/cpu.rs:730`
- `main.rs:13` — CLI parsing / mode dispatch (`--disasm` must be handled before the window opens)

---

## Tracer — functional requirements

### TR-1. Per-instruction record
Emit exactly one record per executed instruction, including CB-prefixed (`0xCB` + operand) and
any undefined-but-executed path. Record fields:

`instruction_counter, PC, opcode_bytes (1-3), disassembly, AF, BC, DE, HL, SP, flags (Z N H C), IME, LY`

### TR-2. State captured at fetch time
PC and registers are read **before** the instruction executes (the state that caused the
behavior). This matches the format used by reference traces (BGB, binjgb), keeping future Tier 3
diffing possible without reformatting. *(See Open Decisions for an optional post-execution field.)*

### TR-3. Robust emission
The tracer must never panic and never alter execution. If an opcode cannot be decoded, log the
raw bytes with an `UNDEFINED` label and let the emulator continue (its normal behavior).

### TR-4. Interrupt logging
Interrupt dispatch is invisible otherwise. Log lines like `INTERRUPT -> 0x0040 (VBlank)`
including the vector and IME before/after, so unexplained jumps are explainable.

### TR-5. Filtering (mandatory)
A full trace is millions of lines. Must support, composably:

- global on/off (default **off**),
- PC range filter (`0x0000-0x00FF`), inclusive,
- instruction-count cap (auto-disable after N instructions),
- optional opcode filter (e.g. trace only `CALL`/`RET`).

### TR-6. Output control
Buffered (`BufWriter`) output to stdout **or** a file (truncate/append). Configured via CLI args
and env vars (no new dependencies):

- `DMG_TRACE=1`
- `DMG_TRACE_PC=0x0000-0x00FF`
- `DMG_TRACE_FILE=out.txt`
- `DMG_TRACE_COUNT=100000`

### TR-7. Negligible hot-path cost
When disabled, a single boolean check per step — no formatting, no string building. When
enabled, filters are evaluated before building the record.

---

## Disassembler — functional requirements

### DR-1. Complete coverage
Must decode **all 256 main + 256 CB opcodes deterministically, including ones the CPU does not
execute yet**. It must therefore **not** depend on `from_byte` / `from_cb_byte` (those return
`None` for the still-missing opcodes); it needs its own complete table. Unknown/invalid bytes →
`UNDEFINED 0xNN`, never a panic.

### DR-2. Returns `(mnemonic, operand_string, length)`
Length (1-3 bytes) is required so code regions can be walked without executing.

### DR-3. Correct operand formatting
Register pairs (`BC/DE/HL/SP/AF`), condition codes (`NZ Z NC C`), `(HL)` memory syntax,
`(FF00+n)` for LDH, and immediate-vs-address disambiguation (`LD A,d8` vs `LD A,(nn)`).

### DR-4. Resolved branch targets
`JR`/`JP`/`CALL` show absolute targets: `JR NZ,0x0123` (relative offset applied to the
post-operand PC), `CALL 0x0150`.

### DR-5. Range dumper
`disasm_range(mem, start, length)` producing address-aligned lines; safe when the range crosses
end-of-memory or data boundaries.

### DR-6. Shared source of truth *(recommended)*
Rather than a second ad-hoc table that can drift from the executor, replace
`from_byte`/`from_cb_byte` with a single data-driven table (e.g. `src/opcodes.rs`: mnemonic,
operands, length, cycles, flag effects) that drives **both** the CPU decoder and the
disassembler. This also becomes the "proper implementation" reference. Tradeoff: larger refactor
than a standalone disassembler table. *(See Open Decisions.)*

---

## Integration — functional requirements

### IR-1. Zero new dependencies
Pure `std` — manual argument/env parsing.

### IR-2. Hook points
Trace hook inside `cpu.step()`; interrupt hook inside `service_interrupt`. Default-off paths
leave CPU/bus behavior byte-for-byte identical.

### IR-3. Headless disassembly mode
`dmg01 --disasm <rom> <start> <end>` prints disassembly and exits — no window, no execution.
This is how the boot ROM / any code region can be read today.

### IR-4. Modules
`src/opcodes.rs` (decode table), `src/disasm.rs`, `src/tracer.rs`; wire the CLI mode in
`main.rs:13` before the window is created.

---

## Open decisions

- **DR-6:** standalone disassembler table (small, faster to ship) vs. unified data-driven opcode
  table (single source of truth, larger refactor).
- **TR-2:** capture pre-execution state only, or also emit post-execution state as a second
  field for convenience.
