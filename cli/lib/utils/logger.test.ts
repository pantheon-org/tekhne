import { beforeEach, describe, expect, spyOn, test } from "bun:test";
import { logger } from "./logger";

describe("logger", () => {
  let consoleLogSpy: ReturnType<typeof spyOn>;

  beforeEach(() => {
    consoleLogSpy = spyOn(console, "log").mockImplementation(() => {});
  });

  test("info should log with blue icon", () => {
    logger.info("test message");
    expect(consoleLogSpy).toHaveBeenCalledWith(
      expect.stringContaining("\x1b[34m"),
    );
    expect(consoleLogSpy).toHaveBeenCalledWith(
      expect.stringContaining("test message"),
    );
  });

  test("success should log with green checkmark", () => {
    logger.success("operation complete");
    expect(consoleLogSpy).toHaveBeenCalledWith(
      expect.stringContaining("\x1b[32m"),
    );
    expect(consoleLogSpy).toHaveBeenCalledWith(
      expect.stringContaining("operation complete"),
    );
  });

  test("warning should log with yellow icon", () => {
    logger.warning("potential issue");
    expect(consoleLogSpy).toHaveBeenCalledWith(
      expect.stringContaining("\x1b[33m"),
    );
    expect(consoleLogSpy).toHaveBeenCalledWith(
      expect.stringContaining("potential issue"),
    );
  });

  test("error should log with red X", () => {
    logger.error("something failed");
    expect(consoleLogSpy).toHaveBeenCalledWith(
      expect.stringContaining("\x1b[31m"),
    );
    expect(consoleLogSpy).toHaveBeenCalledWith(
      expect.stringContaining("something failed"),
    );
  });

  test("debug should log with gray text", () => {
    logger.debug("debug info");
    expect(consoleLogSpy).toHaveBeenCalledWith(
      expect.stringContaining("\x1b[90m"),
    );
    expect(consoleLogSpy).toHaveBeenCalledWith(
      expect.stringContaining("debug info"),
    );
  });

  test("header should log with cyan and newlines", () => {
    consoleLogSpy.mockClear();
    logger.header("Section Title");
    expect(consoleLogSpy).toHaveBeenCalledTimes(1);
    const logCall = consoleLogSpy.mock.calls[0][0];
    expect(logCall).toContain("\x1b[36m");
    expect(logCall).toContain("Section Title");
    expect(logCall).toContain("\n");
  });
});
