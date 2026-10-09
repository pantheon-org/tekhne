//! Behaviour checks, the assertion core: ported from skilldiff (MIT), commit
//! 9271c26, `src/assertions.ts`. These tests pin the semantics of the six
//! assertion kinds, the tool-name normalisation, and the strict parsing of the
//! two file formats (the scenario YAML and a recorded trace).

use skill_auditor::behaviour::assertions::{evaluate, Kind};
use skill_auditor::behaviour::scenario::Behaviour;
use skill_auditor::behaviour::trace::{normalize_tool_name, Trace};

const HEADER: &str = r#""schema_version": 1,
  "header": { "harness": "claude-code", "date": "2026-10-09", "skill_commit": "abc1234", "source": "recorded" }"#;

fn trace(tool_calls: &str, files: &str, commands: &str, output: &str) -> Trace {
    let text = format!(
        r#"{{
  {HEADER},
  "toolCalls": {tool_calls},
  "filesChanged": {files},
  "commandsRun": {commands},
  "output": {output}
}}"#
    );
    Trace::parse(&text).expect("valid trace")
}

fn behaviour(expect_yaml: &str) -> Behaviour {
    let text = format!("schema_version: 1\nname: demo\nexpect:\n{expect_yaml}");
    Behaviour::parse(&text).expect("valid behaviour file")
}

fn passed(trace: &Trace, expect_yaml: &str) -> Vec<(Kind, bool)> {
    let b = behaviour(expect_yaml);
    evaluate(trace, &b.expect)
        .into_iter()
        .map(|r| (r.kind, r.pass))
        .collect()
}

// ---- tool name normalisation -------------------------------------------------

#[test]
fn tool_names_are_normalised_across_harnesses() {
    let cases = [
        ("Read", "read"),
        ("view_file", "read"),
        ("open_file", "read"),
        ("Write", "write"),
        ("Edit", "write"),
        ("MultiEdit", "write"),
        ("str_replace_editor", "write"),
        ("create_file", "write"),
        ("Bash", "bash"),
        ("exec_command", "bash"),
        ("command_execution", "bash"),
        ("Glob", "glob"),
        ("list_dir", "glob"),
        ("Grep", "grep"),
        ("WebSearch", "grep"),
        ("Task", "spawn"),
        ("subagent", "spawn"),
        ("SomethingElse", "somethingelse"),
    ];
    for (raw, want) in cases {
        assert_eq!(normalize_tool_name(raw), want, "{raw}");
    }
}

#[test]
fn the_first_matching_group_wins() {
    // "read" is checked before "write", so a name holding both reads as read.
    assert_eq!(normalize_tool_name("read_and_write"), "read");
}

// ---- the six kinds, each passing and failing ---------------------------------

#[test]
fn files_changed_matches_part_of_a_path_without_regard_to_case() {
    let t = trace("[]", r#"["Docs/NOTES.md"]"#, "[]", r#""""#);
    assert_eq!(
        passed(&t, "  files_changed: [notes.md]\n"),
        [(Kind::FilesChanged, true)]
    );
    assert_eq!(
        passed(&t, "  files_changed: [TODO.md]\n"),
        [(Kind::FilesChanged, false)]
    );
}

#[test]
fn commands_run_matches_part_of_a_command_without_regard_to_case() {
    let t = trace("[]", "[]", r#"["git commit -m x"]"#, r#""""#);
    assert_eq!(
        passed(&t, "  commands_run: [GIT COMMIT]\n"),
        [(Kind::CommandsRun, true)]
    );
    assert_eq!(
        passed(&t, "  commands_run: [npm]\n"),
        [(Kind::CommandsRun, false)]
    );
}

#[test]
fn tool_calls_compare_normalised_names() {
    let t = trace(
        r#"[{"tool":"Read","input":{}},{"tool":"Edit","input":{}}]"#,
        "[]",
        "[]",
        r#""""#,
    );
    assert_eq!(
        passed(&t, "  tool_calls: [read, write]\n"),
        [(Kind::ToolCalls, true), (Kind::ToolCalls, true)]
    );
    assert_eq!(
        passed(&t, "  tool_calls: [bash]\n"),
        [(Kind::ToolCalls, false)]
    );
}

#[test]
fn must_not_fails_when_the_forbidden_thing_happened() {
    let t = trace(
        r#"[{"tool":"Bash","input":{}}]"#,
        r#"["TODO.md"]"#,
        r#"["rm -rf x"]"#,
        r#""""#,
    );
    let all = passed(
        &t,
        "  must_not:\n    files_changed: [TODO.md]\n    commands_run: [rm -rf]\n    tool_calls: [bash]\n",
    );
    assert_eq!(
        all,
        [
            (Kind::MustNot, false),
            (Kind::MustNot, false),
            (Kind::MustNot, false)
        ]
    );
}

#[test]
fn must_not_passes_when_the_forbidden_thing_did_not_happen() {
    let t = trace(
        r#"[{"tool":"Read","input":{}}]"#,
        r#"["NOTES.md"]"#,
        r#"["ls"]"#,
        r#""""#,
    );
    let all = passed(
        &t,
        "  must_not:\n    files_changed: [TODO.md]\n    commands_run: [rm -rf]\n    tool_calls: [bash]\n",
    );
    assert_eq!(
        all,
        [
            (Kind::MustNot, true),
            (Kind::MustNot, true),
            (Kind::MustNot, true)
        ]
    );
}

#[test]
fn must_not_names_what_it_forbids() {
    let t = trace("[]", r#"["TODO.md"]"#, "[]", r#""""#);
    let b = behaviour("  must_not:\n    files_changed: [TODO.md]\n");
    let r = evaluate(&t, &b.expect);
    assert_eq!(r[0].expected, "files_changed does not include TODO.md");
}

#[test]
fn output_contains_matches_without_regard_to_case() {
    let t = trace("[]", "[]", "[]", r#""Done: SPIKE RAN OK""#);
    assert_eq!(
        passed(&t, "  output_contains: [spike ran ok]\n"),
        [(Kind::OutputContains, true)]
    );
    assert_eq!(
        passed(&t, "  output_contains: [missing phrase]\n"),
        [(Kind::OutputContains, false)]
    );
}

#[test]
fn output_not_contains_fails_when_the_text_is_present() {
    let t = trace("[]", "[]", "[]", r#""the password is hunter2""#);
    assert_eq!(
        passed(&t, "  output_not_contains: [PASSWORD]\n"),
        [(Kind::OutputNotContains, false)]
    );
    assert_eq!(
        passed(&t, "  output_not_contains: [token]\n"),
        [(Kind::OutputNotContains, true)]
    );
}

#[test]
fn results_come_in_a_fixed_order() {
    let t = trace(
        r#"[{"tool":"Read","input":{}}]"#,
        r#"["A.md"]"#,
        r#"["ls"]"#,
        r#""ok""#,
    );
    let yaml = "  output_not_contains: [zzz]\n  output_contains: [ok]\n  must_not:\n    files_changed: [B.md]\n  tool_calls: [read]\n  commands_run: [ls]\n  files_changed: [A.md]\n";
    let kinds: Vec<Kind> = passed(&t, yaml).into_iter().map(|(k, _)| k).collect();
    assert_eq!(
        kinds,
        [
            Kind::FilesChanged,
            Kind::CommandsRun,
            Kind::ToolCalls,
            Kind::MustNot,
            Kind::OutputContains,
            Kind::OutputNotContains
        ]
    );
}

// ---- the scenario file: strict, and it reads every kind ----------------------

#[test]
fn the_scenario_reads_output_not_contains() {
    // skilldiff's own loader (src/scenario.ts) never reads this field, so an
    // assertion written in a scenario file was silently dropped. Ours must keep it.
    let b = behaviour("  output_not_contains: [secret, token]\n");
    assert_eq!(b.expect.output_not_contains, ["secret", "token"]);
}

#[test]
fn a_misspelt_expectation_is_an_error_not_a_silent_drop() {
    let err = Behaviour::parse(
        "schema_version: 1\nname: demo\nexpect:\n  output_not_contain: [secret]\n",
    )
    .expect_err("typo must be rejected");
    assert!(err.contains("output_not_contain"), "{err}");
}

#[test]
fn a_scenario_needs_a_supported_schema_version() {
    let missing = Behaviour::parse("name: demo\nexpect:\n  files_changed: [a]\n")
        .expect_err("no schema_version");
    assert!(missing.contains("schema_version"), "{missing}");
    let future = Behaviour::parse("schema_version: 2\nname: demo\nexpect:\n  files_changed: [a]\n")
        .expect_err("future version");
    assert!(future.contains("schema_version"), "{future}");
}

#[test]
fn a_scenario_with_no_expectations_is_an_error() {
    let err = Behaviour::parse("schema_version: 1\nname: demo\nexpect: {}\n")
        .expect_err("nothing to check");
    assert!(err.contains("no assertions"), "{err}");
}

#[test]
fn a_scenario_needs_a_name() {
    let err = Behaviour::parse("schema_version: 1\nname: \"\"\nexpect:\n  files_changed: [a]\n")
        .expect_err("blank name");
    assert!(err.contains("name"), "{err}");
}

#[test]
fn invalid_yaml_is_an_error_that_says_so() {
    let err = Behaviour::parse("schema_version: [unterminated").expect_err("bad yaml");
    assert!(err.contains("invalid YAML"), "{err}");
}

// ---- the trace file: strict --------------------------------------------------

fn trace_text(extra: &str) -> String {
    format!(
        r#"{{ {HEADER}, "toolCalls": [], "filesChanged": [], "commandsRun": [], "output": ""{extra} }}"#
    )
}

#[test]
fn a_valid_trace_parses() {
    let t = Trace::parse(&trace_text("")).expect("valid");
    assert_eq!(t.header.harness, "claude-code");
}

#[test]
fn a_trace_must_carry_its_header() {
    let err = Trace::parse(
        r#"{"schema_version":1,"toolCalls":[],"filesChanged":[],"commandsRun":[],"output":""}"#,
    )
    .expect_err("no header");
    assert!(err.contains("header"), "{err}");
}

#[test]
fn a_trace_rejects_unknown_fields() {
    let err = Trace::parse(&trace_text(r#", "tool_calls": []"#)).expect_err("typo");
    assert!(err.contains("tool_calls"), "{err}");
}

#[test]
fn a_trace_needs_a_supported_schema_version() {
    let text = trace_text("").replace("\"schema_version\": 1", "\"schema_version\": 9");
    let err = Trace::parse(&text).expect_err("future version");
    assert!(err.contains("schema_version"), "{err}");
}

#[test]
fn a_hand_written_trace_says_so_in_its_header() {
    let text = trace_text("").replace("\"recorded\"", "\"hand-written\"");
    let t = Trace::parse(&text).expect("valid");
    assert!(!t.header.source.is_recorded());
}

#[test]
fn a_trace_that_is_not_json_is_an_error() {
    let err = Trace::parse("{ not json").expect_err("bad json");
    assert!(err.contains("invalid JSON"), "{err}");
}
