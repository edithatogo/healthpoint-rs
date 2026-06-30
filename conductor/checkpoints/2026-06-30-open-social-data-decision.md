# open_social_data bridge decision checkpoint

Date: 2026-06-30

## Completed internally

- Added ADR 0009 to keep the bridge string-row based until a concrete consumer or terms-approved provider path exists.
- Confirmed the current view dictionaries already cover the bridge’s stable integration contract.
- Reduced the open_social_data track to the remaining licensing-gated provider integration.

## Remaining external gate

- Actual `open_social_data` provider integration still requires terms review.

## Result

- No repo-local work remains for the bridge-shape decision itself.
- Arrow/Parquet conversion can be revisited later without changing the core Healthpoint crates.
