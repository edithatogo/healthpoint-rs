# Healthpoint licensing and usage constraints

These notes summarize the Healthpoint API Deed Poll observed on 2026-06-30. They are not legal advice.

## Status

The terms page said it was last updated on 7 May 2026. The dashboard showed a terms acceptance date of 2025-10-13 for the account.

## Key constraints observed

The license is non-exclusive, non-transferable, and revocable. It allows use of the licensed material only for the permitted purpose and subject to API restrictions.

The permitted purpose is limited to non-production, sandbox testing and evaluation of the listed use cases. The observed terms explicitly exclude live, commercial, or operational deployment unless Healthpoint gives prior written approval.

The terms prohibit or restrict:

- Competing with Healthpoint by building or commercializing a health directory, provider database, or similar data product.
- Marketing, advertising, solicitation, lead generation, insurance, underwriting, claims processing, and financial services use cases.
- Resale, sublicensing, or redistribution of Directory Information in raw, aggregated, derived, or transformed form.
- Systematic extraction, scraping, or bulk copying for AI training, dataset creation, or similar purposes.
- Making sandbox-derived output, data, or content available to third parties.
- Production, commercial, or live operational use of sandbox data.
- Circumventing API access controls or technical restrictions.

The sandbox term is stated not to exceed 90 days from commencement unless Healthpoint agrees in writing to an extension. On expiry, sandbox access is suspended unless a production API licence is entered.

## Project implications

For this repository:

- Keep live validation opt-in and small.
- Keep real API responses out of git.
- Keep exports local-only unless written Healthpoint approval permits broader use.
- Do not use Healthpoint data for AI training, public datasets, or open_social_data publication without explicit written approval.
- Preserve attribution requirements in any UI, report, export, or derived display.
- Treat production deployment as blocked until Healthpoint grants written approval.
