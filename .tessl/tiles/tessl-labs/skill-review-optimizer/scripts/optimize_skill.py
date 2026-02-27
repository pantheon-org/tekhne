#!/usr/bin/env python3
"""
Iteratively optimize a skill using tessl skill review feedback.

This script automates the process of:
1. Running tessl skill review on a skill
2. Analyzing the feedback
3. Making improvements to the skill
4. Re-running review to verify improvements
5. Repeating until score threshold is met or max iterations reached
"""

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Tuple


def check_tessl_installed() -> bool:
    """Check if tessl CLI is installed."""
    try:
        result = subprocess.run(
            ["tessl", "--version"],
            capture_output=True,
            text=True,
            timeout=5
        )
        return result.returncode == 0
    except (subprocess.TimeoutExpired, FileNotFoundError):
        return False


def install_tessl() -> bool:
    """Install tessl CLI using npm or brew."""
    print("\ntessl CLI not found. Installing...")

    # Check if npm is available
    npm_available = False
    try:
        subprocess.run(["npm", "--version"], capture_output=True, timeout=5)
        npm_available = True
    except (subprocess.TimeoutExpired, FileNotFoundError):
        pass

    # Check if brew is available
    brew_available = False
    try:
        subprocess.run(["brew", "--version"], capture_output=True, timeout=5)
        brew_available = True
    except (subprocess.TimeoutExpired, FileNotFoundError):
        pass

    if npm_available:
        print("Installing tessl via npm...")
        result = subprocess.run(
            ["npm", "install", "-g", "@tessl/cli"],
            capture_output=True,
            text=True
        )
        if result.returncode == 0:
            print("tessl installed successfully via npm")
            return True
        else:
            print(f"npm installation failed: {result.stderr}")

    if brew_available:
        print("Installing tessl via Homebrew...")
        result = subprocess.run(
            ["brew", "install", "tesslio/tap/tessl"],
            capture_output=True,
            text=True
        )
        if result.returncode == 0:
            print("tessl installed successfully via Homebrew")
            return True
        else:
            print(f"Homebrew installation failed: {result.stderr}")

    print("Could not install tessl. Please install npm or Homebrew first.")
    return False


def run_tessl_review(skill_path: str) -> Tuple[str, int, Dict]:
    """
    Run tessl skill review and parse output.

    Returns:
        Tuple of (output_text, average_score, parsed_data)
    """
    result = subprocess.run(
        ["tessl", "skill", "review"],
        cwd=skill_path,
        capture_output=True,
        text=True,
        timeout=60
    )

    output = result.stdout

    # Parse average score
    avg_score = 0
    score_match = re.search(r"Average Score:\s*(\d+)%", output)
    if score_match:
        avg_score = int(score_match.group(1))

    # Parse description and content scores
    desc_score = 0
    content_score = 0

    desc_match = re.search(r"Description:\s*(\d+)%", output)
    if desc_match:
        desc_score = int(desc_match.group(1))

    content_match = re.search(r"Content:\s*(\d+)%", output)
    if content_match:
        content_score = int(content_match.group(1))

    # Parse validation errors
    validation_errors = []
    # Look for validation error patterns in the output
    error_patterns = [
        r"Error:.*",
        r"Validation failed:.*",
        r"Invalid.*",
        r"Missing required field.*"
    ]
    for pattern in error_patterns:
        for match in re.finditer(pattern, output, re.IGNORECASE):
            error_text = match.group(0).strip()
            if error_text and error_text not in validation_errors:
                validation_errors.append(error_text)

    # Parse suggestions
    suggestions = []
    suggestion_section = re.search(
        r"Suggestions:(.*?)(?=Average Score:|$)",
        output,
        re.DOTALL
    )
    if suggestion_section:
        suggestion_text = suggestion_section.group(1)
        # Extract bullet points
        for line in suggestion_text.split('\n'):
            line = line.strip()
            if line.startswith('-'):
                suggestions.append(line[1:].strip())

    parsed_data = {
        'average_score': avg_score,
        'description_score': desc_score,
        'content_score': content_score,
        'validation_errors': validation_errors,
        'suggestions': suggestions,
        'full_output': output
    }

    return output, avg_score, parsed_data


def main():
    parser = argparse.ArgumentParser(
        description="Iteratively optimize a skill using tessl skill review"
    )
    parser.add_argument(
        "skill_path",
        help="Path to the skill directory to optimize"
    )
    parser.add_argument(
        "--max-iterations",
        type=int,
        default=10,
        help="Maximum number of optimization iterations (default: 10)"
    )

    args = parser.parse_args()

    # Validate skill path
    skill_path = Path(args.skill_path).resolve()
    if not skill_path.exists():
        print(f"Error: Skill path does not exist: {skill_path}")
        sys.exit(1)

    skill_md = skill_path / "SKILL.md"
    if not skill_md.exists():
        print(f"Error: SKILL.md not found in {skill_path}")
        sys.exit(1)

    # Check/install tessl
    if not check_tessl_installed():
        if not install_tessl():
            sys.exit(1)
        # Verify installation
        if not check_tessl_installed():
            print("tessl installation verification failed")
            sys.exit(1)

    print(f"\nStarting skill optimization")
    print(f"Skill: {skill_path.name}")
    print(f"Target criteria:")
    print(f"  - No validation errors")
    print(f"  - Description score: 100%")
    print(f"  - Content score: ≥ 90%")
    print(f"Max iterations: {args.max_iterations}\n")

    # Run baseline review
    print("=" * 60)
    print("ITERATION 0: Baseline Review")
    print("=" * 60)

    try:
        output, avg_score, data = run_tessl_review(str(skill_path))

        print(f"\nBaseline Scores:")
        print(f"  Average: {data['average_score']}%")
        print(f"  Description: {data['description_score']}%")
        print(f"  Content: {data['content_score']}%")

        if data['validation_errors']:
            print(f"\nValidation Errors:")
            for i, error in enumerate(data['validation_errors'], 1):
                print(f"  {i}. {error}")
        else:
            print(f"\nValidation Errors: None")

        # Check if already meets all target criteria
        no_errors = len(data['validation_errors']) == 0
        desc_perfect = data['description_score'] == 100
        content_good = data['content_score'] >= 90

        print(f"\nTarget Criteria Status:")
        print(f"  No validation errors: {'PASS' if no_errors else 'FAIL'}")
        print(f"  Description score 100%: {'PASS' if desc_perfect else 'FAIL'} (currently {data['description_score']}%)")
        print(f"  Content score ≥ 90%: {'PASS' if content_good else 'FAIL'} (currently {data['content_score']}%)")

        if no_errors and desc_perfect and content_good:
            print(f"\nSkill meets all target criteria!")
            print("No optimization needed!")
            sys.exit(0)

        print(f"\nSuggestions for improvement:")
        for i, suggestion in enumerate(data['suggestions'], 1):
            print(f"  {i}. {suggestion}")

    except subprocess.TimeoutExpired:
        print("Error: tessl skill review timed out")
        sys.exit(1)
    except Exception as e:
        print(f"Error running baseline review: {e}")
        sys.exit(1)

    print(f"\nWARNING: This script identifies improvements but does not automatically apply them.")
    print(f"Claude will need to read the suggestions and make the edits to improve the skill.")
    print(f"\nTo continue optimization after making changes, run this script again.")


if __name__ == "__main__":
    main()
