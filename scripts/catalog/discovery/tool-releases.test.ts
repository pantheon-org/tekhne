import { describe, expect, test } from "bun:test";
import {
  latestVersion,
  versionFromTag,
} from "../../../docs/src/lib/tool-releases";

describe("versionFromTag", () => {
  test("strips the leading v", () => {
    expect(versionFromTag("v0.3.1")).toBe("0.3.1");
  });

  test("accepts a tag without a v and a pre-release suffix", () => {
    expect(versionFromTag("0.2.5")).toBe("0.2.5");
    expect(versionFromTag("v0.1.0-rc.1")).toBe("0.1.0-rc.1");
  });

  test("rejects anything that is not a version", () => {
    expect(versionFromTag("latest")).toBeNull();
    expect(versionFromTag(undefined)).toBeNull();
    expect(versionFromTag(42)).toBeNull();
  });
});

describe("latestVersion", () => {
  test("reads the tag of the latest release", async () => {
    const urls: string[] = [];
    const version = await latestVersion("pantheon-adr", {
      fetchImpl: async (url) => {
        urls.push(url);
        return { ok: true, json: async () => ({ tag_name: "v0.2.5" }) };
      },
    });
    expect(version).toBe("0.2.5");
    expect(urls).toEqual([
      "https://api.github.com/repos/pantheon-org/pantheon-adr/releases/latest",
    ]);
  });

  test("sends the token when one is given", async () => {
    let auth: string | undefined;
    await latestVersion("pantheon-adr", {
      token: "t0ken",
      fetchImpl: async (_url, init) => {
        auth = init?.headers?.Authorization;
        return { ok: true, json: async () => ({ tag_name: "v1.0.0" }) };
      },
    });
    expect(auth).toBe("Bearer t0ken");
  });

  test("returns null on an error response", async () => {
    const version = await latestVersion("pantheon-adr", {
      fetchImpl: async () => ({ ok: false, json: async () => ({}) }),
    });
    expect(version).toBeNull();
  });

  test("returns null when the lookup throws", async () => {
    const version = await latestVersion("pantheon-adr", {
      fetchImpl: async () => {
        throw new Error("offline");
      },
    });
    expect(version).toBeNull();
  });
});
