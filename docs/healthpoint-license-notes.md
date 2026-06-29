# Healthpoint licensing and usage constraints

These notes summarize the Healthpoint API Deed Poll observed on 2026-06-30 and the June 2026 API and Data Licence Agreement variation retrieved from VUW Outlook. They are not legal advice. The contract files are stored under ignored local evidence at `.healthpoint/contracts/outlook/`.

## Status

The portal terms page said it was last updated on 7 May 2026. The dashboard showed a terms acceptance date of 2025-10-13 for the account.

The VUW Outlook thread titled "Important - Updated Terms for Your Healthpoint API Data Licence Agreement" included a June 2026 licence variation PDF. It names Dylan Mordaunt/VUW as licensee, covers academic non-commercial research, and records a 1 November 2025 commencement date.

## Key constraints observed

The license is non-exclusive, non-transferable, and revocable. It allows use of the licensed material only for the permitted purpose and subject to API restrictions.

The contract-backed permitted purpose is academic, non-commercial research into health system reform effects and geographic distribution/access to care. The directory scope covers Community Pharmacy, General Practitioners, and Health Services national service information.

The contract identifies production API limits for that permitted use. This does not make the open-source repository a public Healthpoint data product, public proxy, or sublicensing mechanism.

The terms prohibit or restrict:

- Competing with Healthpoint by building or commercializing a health directory, provider database, or similar data product.
- Marketing, advertising, solicitation, lead generation, insurance, underwriting, claims processing, and financial services use cases.
- Resale, sublicensing, or redistribution of Directory Information in raw, aggregated, derived, or transformed form.
- Systematic extraction, scraping, or bulk copying for AI training, dataset creation, or similar purposes.
- Using the licensed material or confidential information to train, develop, evaluate, or commercialize AI systems or competing tools.
- Making sandbox-derived output, data, or content available to third parties.
- Production, commercial, or live operational use outside the executed licence and permitted purpose.
- Circumventing API access controls or technical restrictions.

The licence variation specifies production limits for the licensed research use: 100,000 API calls per day and 350 requests per second. The portal also showed UAT usage-plan values of 100,000 quota, 10 rate, and 10 burst.

The first annual production licence fee is waived on the basis that publications using Healthpoint API data clearly attribute Healthpoint as the data source and method. Future production use has a stated annual fee unless separately varied.

The sandbox term is stated not to exceed 90 days from commencement unless Healthpoint agrees in writing to an extension. On expiry, sandbox access is suspended unless a production API licence is entered.

## Project implications

For this repository:

- Keep live validation opt-in and small.
- Keep real API responses out of git.
- Keep exports local-only unless the executed licence or written Healthpoint approval permits broader use.
- Do not use Healthpoint data for AI training, public datasets, or open_social_data publication without explicit written approval.
- Preserve attribution requirements in any UI, report, export, or derived display.
- Treat production use as allowed only for Dylan/VUW within the executed academic non-commercial research purpose and operational limits.
- Treat public tools, public caches, public datasets, redistribution, sublicensing, and open_social_data publication as blocked without additional written approval.
