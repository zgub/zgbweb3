# Open questions

Answer inline, then move the outcome to DECISIONS.md.

**Q-001 — Name and domain.** "zgbweb" is the project name from the claude.ai
Project. Public name? Domain? (Affects title, footer, storage origin.)

**Q-002 — Language.** English only, Slovak only, or both with a switch?
Slovak matters if the Sunflower class uses it. Also short vs long scale
(billion vs miliarda) — the powers-of-ten page would follow the language.
If both: decide early, because every string then goes through a lookup.

**Q-003 — License.** Code AGPL-3.0-or-later (like Flysch) or MIT/Apache
(easier for schools and other devs to reuse)? Content CC BY-SA 4.0 or CC BY 4.0?
Public repo or private?

**Q-004 — Ads or donations?** Ads mean a consent platform, a privacy-policy
rewrite and kid-directed-content rules (ad networks restrict personalised ads
to children; "made for kids" settings). A "buy me a coffee" link costs none of
that. Which do you want to plan for?

**Q-005 — Operator identity for the privacy page.** GDPR needs the
controller's name and contact. Your name, your živnosť, or something else?

**Q-006 — Hosting.** Static host (Cloudflare Pages / GitHub Pages / Netlify)
is enough until accounts exist. Or your own box behind the axum server?

**Q-007 — The gas-in-a-box code you wrote with Viliam.** Port it into
`sims::gas`, or start fresh to the new structure?

**Q-008 — Mobile.** Phones are a big share of kids' browsing. Target
"usable on a phone" from the start (menu collapses below 720 pt today), or
desktop/tablet first?

**Q-009 — Spherical-cow marker.** Use the same 🐄 "model alert" device as the
Sunflower class across all topics? Name in English/Slovak?
