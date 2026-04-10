#!/usr/bin/env python3
"""
PubMed-Search: Biomedical literature search and analysis tool.
Supports searching, metadata retrieval, deep analysis, and PDF download from PubMed.
"""

import os
import sys
import json
import argparse
import xml.etree.ElementTree as ET
from urllib.parse import quote
from pathlib import Path
from typing import Optional, Dict, List, Any

try:
    import requests
except ImportError:
    print("Error: requests is required.")
    print("Install with: pip install requests")
    sys.exit(1)

try:
    from dotenv import load_dotenv
    load_dotenv()
except ImportError:
    pass


# ============================================
# Configuration
# ============================================

class Config:
    """Configuration manager."""

    def __init__(self):
        self.api_key = os.getenv('PUBMED_API_KEY', '')
        self.email = os.getenv('PUBMED_EMAIL', '')
        self.tool = os.getenv('PUBMED_TOOL', 'pubmed-search-skill')
        self.base_url = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils"

    def get_params(self, extra_params: dict = None) -> dict:
        """Build API request parameters."""
        params = {
            'tool': self.tool,
            'retmode': 'xml'
        }
        if self.api_key:
            params['api_key'] = self.api_key
        if self.email:
            params['email'] = self.email
        if extra_params:
            params.update(extra_params)
        return params


# ============================================
# PubMed Search
# ============================================

class PubMedSearch:
    """PubMed search and retrieval client."""

    def __init__(self, config: Config):
        self.config = config
        self.session = requests.Session()
        self.session.headers.update({
            'User-Agent': f'{config.tool}/1.0 ({config.email or "anonymous"})'
        })

    def generate_search_url(self, term=None, title=None, author=None, journal=None,
                            start_date=None, end_date=None, num_results=10) -> str:
        """Build a PubMed esearch URL from filter parameters."""
        base_url = f"{self.config.base_url}/esearch.fcgi"
        query_parts = []

        if term:
            query_parts.append(quote(term))
        if title:
            query_parts.append(f"{quote(title)}[Title]")
        if author:
            query_parts.append(f"{quote(author)}[Author]")
        if journal:
            query_parts.append(f"{quote(journal)}[Journal]")
        if start_date and end_date:
            query_parts.append(f"{start_date}:{end_date}[Date - Publication]")

        query = " AND ".join(query_parts)

        params = self.config.get_params({
            "db": "pubmed",
            "term": query,
            "retmax": str(num_results)
        })

        return f"{base_url}?{'&'.join([f'{k}={v}' for k, v in params.items()])}"

    def search_articles(self, search_url: str) -> List[str]:
        """Parse PMID list from a PubMed esearch URL."""
        try:
            response = self.session.get(search_url)
            if response.status_code != 200:
                print(f"Error: could not fetch search results (status {response.status_code})")
                return []

            root = ET.fromstring(response.content)
            id_list = root.find("IdList")

            if id_list is not None:
                return [id_elem.text for id_elem in id_list.findall("Id")]
            else:
                print("No search results found.")
                return []

        except Exception as e:
            print(f"Error searching articles: {e}")
            return []

    def get_metadata(self, pmid: str) -> Optional[Dict[str, Any]]:
        """Fetch detailed metadata for a single PMID via efetch."""
        try:
            url = f"{self.config.base_url}/efetch.fcgi"
            params = self.config.get_params({
                "db": "pubmed",
                "id": pmid
            })

            response = self.session.get(url, params=params)

            if response.status_code != 200:
                print(f"Error: could not fetch metadata (status {response.status_code})")
                return None

            root = ET.fromstring(response.content)
            article = root.find(".//Article")

            if article is None:
                print(f"No article data found for PMID: {pmid}")
                return None

            title_elem = article.find(".//ArticleTitle")
            title = title_elem.text if title_elem is not None else "No title"

            abstract_elem = article.find(".//Abstract/AbstractText")
            abstract = abstract_elem.text if abstract_elem is not None else "No abstract"

            authors = []
            for author_elem in article.findall(".//Author"):
                last_name = author_elem.find(".//LastName")
                if last_name is not None and last_name.text:
                    authors.append(last_name.text)
            authors_str = ", ".join(authors) if authors else "No author information"

            journal_elem = article.find(".//Journal/Title")
            journal = journal_elem.text if journal_elem is not None else "No journal information"

            pub_date_elem = article.find(".//PubDate/Year")
            pub_date = pub_date_elem.text if pub_date_elem is not None else "No date"

            doi = ""
            doi_elem = article.find(".//ArticleId[@IdType='doi']")
            if doi_elem is not None:
                doi = doi_elem.text

            return {
                "PMID": pmid,
                "Title": title,
                "Authors": authors_str,
                "Journal": journal,
                "Publication Date": pub_date,
                "Abstract": abstract,
                "DOI": doi
            }

        except Exception as e:
            print(f"Error fetching metadata: {e}")
            return None

    def search_by_keywords(self, keywords: str, num_results: int = 10) -> List[Dict[str, Any]]:
        """Search PubMed by keywords and return metadata list."""
        search_url = self.generate_search_url(term=keywords, num_results=num_results)
        print(f"Search URL: {search_url}")

        pmids = self.search_articles(search_url)
        articles = []

        for pmid in pmids:
            metadata = self.get_metadata(pmid)
            if metadata:
                articles.append(metadata)

        return articles

    def search_advanced(self, term=None, title=None, author=None, journal=None,
                        start_date=None, end_date=None, num_results=10) -> List[Dict[str, Any]]:
        """Advanced multi-filter PubMed search."""
        search_url = self.generate_search_url(
            term=term, title=title, author=author, journal=journal,
            start_date=start_date, end_date=end_date, num_results=num_results
        )
        print(f"Search URL: {search_url}")

        pmids = self.search_articles(search_url)
        articles = []

        for pmid in pmids:
            metadata = self.get_metadata(pmid)
            if metadata:
                articles.append(metadata)

        return articles

    def download_pdf(self, pmid: str, output_dir: str = ".") -> str:
        """Attempt to download an open-access full-text PDF from PMC."""
        try:
            print(f"Attempting to retrieve full text for PMID: {pmid}")

            metadata = self.get_metadata(pmid)
            if not metadata:
                return f"Error: could not fetch info for PMID {pmid}"

            efetch_url = f"{self.config.base_url}/efetch.fcgi"
            params = self.config.get_params({
                "db": "pubmed",
                "id": pmid
            })

            response = self.session.get(efetch_url, params=params)
            if response.status_code != 200:
                return f"Error: could not fetch article data (status {response.status_code})"

            root = ET.fromstring(response.content)
            pmc_id_elem = root.find(".//ArticleId[@IdType='pmc']")

            if pmc_id_elem is None:
                pubmed_url = f"https://pubmed.ncbi.nlm.nih.gov/{pmid}/"
                return f"No PMC ID found.\nView article at: {pubmed_url}"

            pmc_id = pmc_id_elem.text
            pmc_url = f"https://www.ncbi.nlm.nih.gov/pmc/articles/{pmc_id}/"

            pmc_response = self.session.get(pmc_url)
            if pmc_response.status_code != 200:
                return f"Cannot access PMC article page (status {pmc_response.status_code})\n{pmc_url}"

            if "This article is available under a" not in pmc_response.text:
                return f"Article is not fully open access.\nView at: {pmc_url}"

            pdf_url = f"https://www.ncbi.nlm.nih.gov/pmc/articles/{pmc_id}/pdf"
            pdf_response = self.session.get(pdf_url)

            if pdf_response.status_code != 200:
                return f"Cannot download PDF (status {pdf_response.status_code})\nView at: {pmc_url}"

            output_path = Path(output_dir)
            output_path.mkdir(parents=True, exist_ok=True)

            filename = output_path / f"PMID_{pmid}_PMC_{pmc_id}.pdf"
            with open(filename, 'wb') as f:
                f.write(pdf_response.content)

            return f"PDF downloaded: {filename}"

        except Exception as e:
            return f"Error downloading PDF: {e}"

    def generate_analysis(self, metadata: Dict[str, Any]) -> str:
        """Generate a deep-analysis prompt from article metadata."""
        title = metadata.get('Title', 'No title')
        authors = metadata.get('Authors', 'No authors')
        journal = metadata.get('Journal', 'No journal')
        pub_date = metadata.get('Publication Date', 'No date')
        abstract = metadata.get('Abstract', 'No abstract')
        pmid = metadata.get('PMID', '')
        doi = metadata.get('DOI', '')

        prompt = f"""# Deep Paper Analysis (abstract-based)

## Metadata
- **Title**: {title}
- **Authors**: {authors}
- **Journal**: {journal}
- **Publication Date**: {pub_date}
- **PMID**: {pmid}
- **DOI**: {doi}

## Abstract
{abstract}

---

## Analysis

### 1. Research Background and Significance
- What is the context of this research area?
- Why is this study important and what is its novelty?

### 2. Research Question or Hypothesis
- What problem does the study address?
- What is the central hypothesis?

### 3. Methodology Overview
- What research methods were used?
- Is the study design appropriate?

### 4. Key Findings
- What are the main results?
- Does the data support the conclusions?

### 5. Conclusions and Impact
- What are the primary conclusions?
- What is the impact on the field?

### 6. Limitations
- What are the study's limitations?
- How could the research be improved?

### 7. Future Directions
- What directions does this work open for future research?

### 8. Relation to Existing Research
- How does this work relate to other studies in the field?
- Is it consistent with known results?

### 9. Overall Assessment
- Overall evaluation of the research quality
- Applicability and reliability assessment

Note: This analysis is based solely on the abstract and metadata. Obtain the full text for production-level review.
"""
        return prompt


# ============================================
# Output Handling
# ============================================

class OutputHandler:
    """Handles formatting and saving of search results."""

    @staticmethod
    def format_console(articles: List[Dict[str, Any]], show_abstract: bool = False) -> str:
        """Human-readable console output."""
        if not articles:
            return "No articles found."

        lines = []
        for i, article in enumerate(articles, 1):
            lines.append(f"\n{'='*80}")
            lines.append(f"[{i}] {article.get('Title', 'No title')}")
            lines.append(f"Authors: {article.get('Authors', 'N/A')}")
            lines.append(f"Journal: {article.get('Journal', 'N/A')} ({article.get('Publication Date', 'N/A')})")
            lines.append(f"PMID: {article.get('PMID', 'N/A')}")
            if article.get('DOI'):
                lines.append(f"DOI: {article['DOI']}")
            if show_abstract:
                abstract = article.get('Abstract', 'No abstract')
                if len(abstract) > 500:
                    abstract = abstract[:500] + "..."
                lines.append(f"\nAbstract:\n{abstract}")

        return "\n".join(lines)

    @staticmethod
    def format_json(articles: List[Dict[str, Any]]) -> str:
        """JSON output for machine processing."""
        return json.dumps(articles, ensure_ascii=False, indent=2)

    @staticmethod
    def format_markdown(articles: List[Dict[str, Any]]) -> str:
        """Markdown output for documentation."""
        if not articles:
            return "# Search Results\n\nNo articles found."

        lines = ["# PubMed Search Results\n"]
        lines.append(f"Found {len(articles)} articles.\n")

        for i, article in enumerate(articles, 1):
            lines.append(f"## {i}. {article.get('Title', 'No title')}\n")
            lines.append(f"- **Authors**: {article.get('Authors', 'N/A')}")
            lines.append(f"- **Journal**: {article.get('Journal', 'N/A')} ({article.get('Publication Date', 'N/A')})")
            lines.append(f"- **PMID**: {article.get('PMID', 'N/A')}")
            if article.get('DOI'):
                lines.append(f"- **DOI**: {article['DOI']}")
            lines.append(f"\n**Abstract**:\n{article.get('Abstract', 'No abstract')}\n")
            lines.append("---\n")

        return "\n".join(lines)

    @staticmethod
    def save_output(content: str, output_path: str) -> bool:
        """Save output content to a file."""
        try:
            output_file = Path(output_path)
            output_file.parent.mkdir(parents=True, exist_ok=True)

            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(content)

            print(f"Output saved to: {output_file}")
            return True

        except Exception as e:
            print(f"Error saving file: {e}")
            return False


# ============================================
# Main
# ============================================

def main():
    parser = argparse.ArgumentParser(
        description='PubMed-Search: Biomedical literature search and analysis tool',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Keyword search
  %(prog)s search --keywords "COVID-19 vaccine" --results 10

  # Advanced search
  %(prog)s search --term "cancer" --author "Smith" --journal "Nature" --start-date "2020" --end-date "2023"

  # Get article metadata
  %(prog)s metadata --pmid "12345678"

  # Deep analysis
  %(prog)s analyze --pmid "12345678" --output analysis.md

  # Download open-access PDF
  %(prog)s download --pmid "12345678" --output-dir ./papers/
        """
    )

    subparsers = parser.add_subparsers(dest='command', help='Available commands')

    # search subcommand
    search_parser = subparsers.add_parser('search', help='Search articles')
    search_parser.add_argument('--keywords', help='Search keywords')
    search_parser.add_argument('--term', help='General search term')
    search_parser.add_argument('--title', help='Title search term')
    search_parser.add_argument('--author', help='Author name')
    search_parser.add_argument('--journal', help='Journal name')
    search_parser.add_argument('--start-date', help='Start date (YYYY/MM/DD)')
    search_parser.add_argument('--end-date', help='End date (YYYY/MM/DD)')
    search_parser.add_argument('--results', type=int, default=10, help='Number of results (default: 10)')
    search_parser.add_argument('--output', help='Output file path')
    search_parser.add_argument('--format', choices=['console', 'json', 'markdown'],
                               default='console', help='Output format (default: console)')
    search_parser.add_argument('--show-abstract', action='store_true', help='Include abstract in output')

    # metadata subcommand
    metadata_parser = subparsers.add_parser('metadata', help='Get article metadata')
    metadata_parser.add_argument('--pmid', required=True, help='PubMed ID')
    metadata_parser.add_argument('--output', help='Output file path')
    metadata_parser.add_argument('--format', choices=['console', 'json', 'markdown'],
                                 default='console', help='Output format (default: console)')

    # analyze subcommand
    analyze_parser = subparsers.add_parser('analyze', help='Deep-analyze an article')
    analyze_parser.add_argument('--pmid', required=True, help='PubMed ID')
    analyze_parser.add_argument('--output', help='Output file path')

    # download subcommand
    download_parser = subparsers.add_parser('download', help='Download open-access PDF')
    download_parser.add_argument('--pmid', required=True, help='PubMed ID')
    download_parser.add_argument('--output-dir', default='.', help='Output directory (default: current dir)')

    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        return 1

    config = Config()
    searcher = PubMedSearch(config)

    if args.command == 'search':
        if args.keywords:
            articles = searcher.search_by_keywords(args.keywords, args.results)
        else:
            articles = searcher.search_advanced(
                term=args.term, title=args.title, author=args.author,
                journal=args.journal, start_date=args.start_date,
                end_date=args.end_date, num_results=args.results
            )

        if not articles:
            print("No matching articles found.")
            return 0

        if args.format == 'json':
            output = OutputHandler.format_json(articles)
        elif args.format == 'markdown':
            output = OutputHandler.format_markdown(articles)
        else:
            output = OutputHandler.format_console(articles, args.show_abstract)

        print(output)

        if args.output:
            OutputHandler.save_output(output, args.output)

    elif args.command == 'metadata':
        metadata = searcher.get_metadata(args.pmid)

        if not metadata:
            print(f"No metadata found for PMID: {args.pmid}")
            return 1

        if args.format == 'json':
            output = OutputHandler.format_json([metadata])
        elif args.format == 'markdown':
            output = OutputHandler.format_markdown([metadata])
        else:
            output = OutputHandler.format_console([metadata], show_abstract=True)

        print(output)

        if args.output:
            OutputHandler.save_output(output, args.output)

    elif args.command == 'analyze':
        metadata = searcher.get_metadata(args.pmid)

        if not metadata:
            print(f"No metadata found for PMID: {args.pmid}")
            return 1

        analysis = searcher.generate_analysis(metadata)
        print(analysis)

        if args.output:
            OutputHandler.save_output(analysis, args.output)

    elif args.command == 'download':
        result = searcher.download_pdf(args.pmid, args.output_dir)
        print(result)

    return 0


if __name__ == '__main__':
    sys.exit(main())
