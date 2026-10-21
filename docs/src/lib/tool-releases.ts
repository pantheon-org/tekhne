const OWNER = "pantheon-org";

/** The version in a release tag, or null when the tag is not a version. */
export const versionFromTag = (tag: unknown): string | null => {
  if (typeof tag !== "string") return null;
  return tag.match(/^v?(\d+\.\d+\.\d+\S*)$/)?.[1] ?? null;
};

type Fetch = (
  url: string,
  init?: { headers?: Record<string, string>; signal?: AbortSignal },
) => Promise<{ ok: boolean; json: () => Promise<unknown> }>;

/** The longest a single release lookup may take before it counts as failed. */
const DEFAULT_TIMEOUT_MS = 10_000;

/**
 * The version of a tool repository's latest GitHub release, read from the API at
 * build time. Every tool now ships from its own repository, so a release no
 * longer touches this one; the docs deploy rebuilds weekly to pick it up.
 *
 * Returns null when the lookup fails (offline build, rate limit, no release yet),
 * so the page falls back to the `latest` download link and a build never fails
 * on the network.
 */
export const latestVersion = async (
  repo: string,
  options: { token?: string; fetchImpl?: Fetch; timeoutMs?: number } = {},
): Promise<string | null> => {
  const fetchImpl = options.fetchImpl ?? (fetch as unknown as Fetch);
  const headers: Record<string, string> = {
    Accept: "application/vnd.github+json",
  };
  if (options.token) headers.Authorization = `Bearer ${options.token}`;

  try {
    const response = await fetchImpl(
      `https://api.github.com/repos/${OWNER}/${repo}/releases/latest`,
      {
        headers,
        // A whole-request deadline: a slow response would otherwise stall the
        // whole static build, because the failure fallback only applies once the
        // request has failed.
        signal: AbortSignal.timeout(options.timeoutMs ?? DEFAULT_TIMEOUT_MS),
      },
    );
    if (!response.ok) return null;
    const body = (await response.json()) as { tag_name?: unknown };
    return versionFromTag(body.tag_name);
  } catch {
    return null;
  }
};
