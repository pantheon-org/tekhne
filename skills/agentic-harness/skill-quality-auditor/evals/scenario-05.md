# Scenario 05: Skills Portfolio Optimization Through Consolidation Analysis

## User Prompt

Your organization has been developing AI agent skills for two years, and the skills collection has grown organically to include 25 different skills. The development team suspects there may be significant overlap and redundancy between some skills, leading to maintenance overhead and confusion about which skill to use in different situations.

The CTO wants to optimize the skills portfolio by identifying redundant content and consolidating related skills where appropriate. However, the team needs to be careful not to merge skills that serve different purposes, as this could harm discoverability and make the skills less targeted.

The analysis should identify which skills have overlapping content, quantify the similarity levels, and make recommendations about which skills should be consolidated and which should remain separate. Any consolidation recommendations should preserve the unique value of each skill while reducing overall maintenance burden.

## Output Specification

Create a comprehensive consolidation analysis that includes:

1. **similarity-analysis.md** - Detailed analysis of content overlap between skills
2. **consolidation-recommendations.md** - Specific recommendations for merging or keeping skills separate
3. **duplication-report.json** - Structured data showing similarity percentages and statistics
4. **analysis-methodology.md** - Document your approach to detecting and analyzing duplication

## Expected Behavior

1. Calculate content similarity percentages between skill pairs
2. Apply the >20% similarity threshold to identify aggregation candidates
3. Flag pairs with >35% similarity as critical duplication requiring immediate action
4. Perform line-by-line or content comparison to detect overlapping text
5. Never aggregate skills with low similarity or skills from different domains
6. Evaluate whether skills serve similar purposes vs. different domains before recommending consolidation
7. Compare each skill against others systematically in a pairwise fashion
8. Consider file organisation, reference patterns, and naming conventions as structural signals

## Success Criteria

- **Similarity percentage calculation**: Calculates content similarity percentages between skill pairs
- **20% threshold application**: Uses >20% similarity as aggregation candidate threshold
- **35% critical threshold**: Identifies >35% similarity as critical duplication requiring immediate action
- **Text similarity analysis**: Performs line-by-line or content comparison to detect overlapping text
- **NEVER aggregate low-similarity**: Follows the rule to never aggregate skills with low similarity or different domains
- **Domain fit evaluation**: Considers whether skills serve similar purposes vs different domains
- **Consolidation recommendations**: Provides specific recommendations for which skills to merge or keep separate
- **Pairwise comparison**: Compares each skill against others systematically
- **Structural analysis**: Considers similar file organization, reference patterns, or naming conventions

## Failure Conditions

- Recommends consolidation without calculating any similarity percentages
- Applies no similarity thresholds, treating all overlapping skills as consolidation candidates
- Does not distinguish between skills that have superficial overlap vs. deep content duplication
- Aggregates skills from entirely different domains based on superficial similarity
- Skips pairwise comparison and analyses only a subset of skill pairs
- Provides no specific recommendations — concludes only with "consider reviewing"
