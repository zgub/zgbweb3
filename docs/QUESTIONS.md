# Open questions

Answer inline, then move the outcome to DECISIONS.md.

**Q-001 — Name and domain.** "zgbweb" is the project name from the claude.ai
Project. Public name? Domain? (Affects title, footer, storage origin.)

**Q-002 — Language.** ~~English only, Slovak only, or both?~~ → **Answered
2026-10-11: Slovak and English, localisable, texts later served by the server.**
See D-013.

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
desktop/tablet first? *(Checked at 400 pt wide on 2026-10-11: usable; cards
stack, cube scenes scale, breadcrumb hides.)*

**Q-009 — Spherical-cow marker.** Use the same 🐄 "model alert" device as the
Sunflower class across all topics? Name in English/Slovak?

**Q-010 — Czech visitors.** Browser language `cs` currently falls back to
English. Show Slovak to Czech browsers instead (most Czech kids read Slovak
fine), or keep English until there is a Czech catalog?

**Q-011 — "Rubik's" in the text?** The page says "puzzle cube" /
"kocka-hlavolam" and draws a generic scrambled 3×3 cube with no logo (D-014).
Slovak kids say "Rubikova kocka". Mentioning the name descriptively in the
text ("a puzzle cube, like a Rubik's Cube") is normal use of a trade mark,
but keeping it out of titles, the logo and marketing is the safe line. Want
the name mentioned in the story text, or keep it fully generic?

**Q-012 — Tykanie.** Slovak texts address the reader as "ty" (kids' site).
The privacy page is read by parents too — keep "ty" there, or switch that
page to "vy"?

**Q-013 — Font.** Nunito (rounded, friendly, full Slovak) for everything,
two weights. Happy with it, or try Lexend / Atkinson Hyperlegible Next for
body text (both designed for reading ease)?
