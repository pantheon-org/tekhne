# Scenario 4: Fresh Checkout, Title-Only Lookup and Bulk Citations

## User Prompt

The skill has just been checked out and has never been run: there is no virtual environment. The user writes:

"I only know the title 'Attention Is All You Need' (arXiv 1706.03762). Get me the full metadata and its list of references, saved to `candidates.json`. Then I want the citing papers for each of 150 papers in my reading list, as fast as possible with a quick loop."

Repo state: the skill directory with `requirements.txt` and `scripts/semantic-scholar-search.py`. No `SEMANTIC_SCHOLAR_API_KEY` is set.
