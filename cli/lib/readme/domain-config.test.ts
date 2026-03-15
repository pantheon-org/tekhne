import { describe, expect, test } from "bun:test";
import { DOMAINS, getDomainInfo } from "./domain-config";

describe("getDomainInfo", () => {
  test("returns the matching domain for a known key", () => {
    const info = getDomainInfo("ci-cd");
    expect(info).toBeDefined();
    expect(info?.key).toBe("ci-cd");
  });

  test("returns undefined for an unknown key", () => {
    expect(getDomainInfo("nonexistent")).toBeUndefined();
  });

  test("every entry in DOMAINS is findable by key", () => {
    for (const domain of DOMAINS) {
      expect(getDomainInfo(domain.key)?.key).toBe(domain.key);
    }
  });
});
