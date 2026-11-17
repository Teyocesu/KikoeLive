# AGENTS.md — operational rules for anyone (human or agent) changing KikoeLive

1. Read `SPEC.md` (scope + invariants), `HANDOFF.md` (current state) and `EVIDENCE.md`
   (what has actually been measured) before changing code.
2. English **and** Spanish are both release requirements. Never remove, hide or
   degrade one to make the other pass. Evaluate them separately.
3. Correct meaning beats fluency. Treat inverted/missing negation, wrong subject,
   wrong action, invented details and wrong names as severe defects.
4. Never claim a result you did not measure. Label evidence as automated test,
   real execution, manual visual check, static inspection, simulation or not tested.
   Compilation is not validation; a mock is not end-to-end.
5. Local-first: no telemetry, accounts, cloud inference, network listeners or audio /
   transcript persistence. Network use is limited to user-initiated model downloads
   from the pinned catalog in `crates/kikoe-core/src/models.rs`.
6. Never capture the microphone. System audio only, with visible state.
7. Do not weaken tests, filters, queue bounds or acceptance criteria to get green.
8. Model weights, datasets, generated audio and build outputs never go into Git
   (`.gitignore`). Record provenance and licences in `THIRD-PARTY-NOTICES.md`.
9. Keep platform code isolated (`crates/kikoe-capture`, GPU features); keep the
   shared pipeline (`crates/kikoe-core`) platform-independent and unit-tested.
10. Build: `. .\scripts\dev-env.ps1` (Windows) then see `README.md`. Run
    `cargo test -p kikoe-core -p kikoe-eval` before committing.
11. No pushes, merges to main, tags or releases without explicit owner approval.
