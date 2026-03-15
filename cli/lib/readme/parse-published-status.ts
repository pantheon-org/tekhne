import type { PublishedStatus } from "./tile-types";

export const parsePublishedStatus = (
  tileData: Record<string, unknown>,
): PublishedStatus =>
  tileData.private === false
    ? "public"
    : tileData.private === true
      ? "private"
      : "unpublished";
