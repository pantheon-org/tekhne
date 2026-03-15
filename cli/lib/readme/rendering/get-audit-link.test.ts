import { describe, expect, test } from "bun:test";
import { getAuditLink } from "./get-audit-link";

describe("getAuditLink", () => {
  test("returns markdown link with date as label", () => {
    expect(
      getAuditLink("2026-03-15", ".context/audits/foo/2026-03-15/audit.json"),
    ).toBe("[2026-03-15](.context/audits/foo/2026-03-15/audit.json)");
  });

  test("uses first argument as display text and second as href", () => {
    const result = getAuditLink("label", "path/to/file");
    expect(result).toBe("[label](path/to/file)");
  });
});
