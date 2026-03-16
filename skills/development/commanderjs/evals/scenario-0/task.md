# Build a File Conversion CLI Tool

## Problem Description

A data engineering team needs a small command-line tool to convert JSON files to CSV format. The tool will be used in shell scripts and automated pipelines, so correct exit codes are important. When the conversion succeeds the tool should exit with code 0; on any error it should exit with a non-zero code and print a useful message.

The tool should accept a required input file path and an optional output file path (defaulting to the input filename with a `.csv` extension). It should also support a `--verbose` flag for debug output during development.

The conversion logic itself can be a stub — just write the structure of the tool with correct argument handling, error management, and program entry point. The team will fill in the actual JSON-to-CSV logic later.

Write the complete TypeScript source for the tool in a file called `cli.ts`. The tool should be runnable with `bun cli.ts <input> [output]`.

## Output Specification

Create `cli.ts` with the full CLI implementation. The file should be executable as a bun script.
