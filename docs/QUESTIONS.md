# Open questions

Answer inline, then move the outcome to DECISIONS.md.

**Q-001 — Site name.** ~~Domain?~~ → **Domain answered 2026-10-21: zgub.net**,
possibly hunci.sk later (D-019). Still open: what the site calls itself in the
title bar, footer and browser tab. Proposal: just **zgub.net**. It is already
the identity people know, it needs no translation, and it ties the name to the
domain. If the site moves to hunci.sk, the name changes with it.

**Q-002 — Language.** → **Answered: Slovak and English** (D-013).

**Q-003 — Code licence.** ~~Content licence?~~ → **Content answered
2026-10-21: reuse with credit, so CC BY 4.0** (D-010, D-025). Still open: code
under AGPL-3.0-or-later (like Flysch: anyone running a changed copy must share
it) or MIT/Apache (easiest for schools and other developers to reuse)? Public
repo or private?

**Q-004 — Ads at all?** Targeted ads are ruled out (D-024). Do you want to
keep the door open for non-tracking ads, or rule out ads entirely? A "buy me a
coffee" link fits the site's spirit better and needs no consent machinery.

**Q-005 — Operator identity for the privacy page.** GDPR needs the
controller's name and contact once the site processes personal data (server
logs today, comments later). Your name, your živnosť, or something else? This
is the one place where zgub and hunci (D-019) may have to meet. If the živnosť
is involved, check whether that brings its own duty to publish identification
details. Not legal advice.

**Q-006 — Hosting.** Static host (Cloudflare Pages / GitHub Pages / Netlify)
or your own box behind the axum server? Also decides how much crawler blocking
is possible (D-021): your own server can refuse bots itself; Cloudflare has a
ready-made AI-bot setting; GitHub Pages offers neither.

**Q-007 — The gas-in-a-box code you wrote with your son.** Port it into
`sims::gas`, or start fresh to the new structure?

**Q-008 — Mobile.** Target "usable on a phone" from the start (menu collapses
below 720 pt), or desktop/tablet first? The old zgbweb2 menu is a fixed
200 px right panel with no collapse.

**Q-009 — Spherical-cow marker.** Use the same 🐄 "model alert" device across
all topics? Name in English/Slovak?

**Q-010 — Czech visitors.** Browser language `cs` falls back to English. Show
Slovak to Czech browsers instead, or keep English until there is a Czech catalog?

**Q-011 — "Rubik's" in the text?** The page says "puzzle cube" /
"kocka-hlavolam" and draws a generic cube with no logo (D-014). Mention the
name descriptively in the text, or keep it generic?

**Q-012 — Tykanie.** Keep "ty" everywhere, or use "vy" on some pages
(privacy, Projects, Workbench)?

**Q-013 — Fonts and glyphs.** *Deferred, by Peter's choice.* When it comes
up: do egui's default fonts cover all Slovak letters (ľ, ĺ, ŕ, ô, ä, č, š, ž
and capitals), every menu glyph, and the superscript-minus for 10⁻³? Check by
screenshot. Constraint from D-018: appealing, not flashy.

**Q-014 — Learning section name.** → **Answered: Workbench.** Slovak proposal:
**Dielňa** (workshop), shorter than *Pracovný stôl*. OK?

**Q-015 — What is public?** → **Answered: everything; no names of private
people ever** (D-023). Leftover: where does the chemistry/biology course for
8th graders go: Explore as its own strand, Projects, or not at all?

**Q-016 — Energy-ladder numbers.** → **Mostly answered: ladder drafted in
ENERGY-LADDER.md** (D-026). Still open: (a) the household for "household-days
of electricity": Slovak average, EU average, or yours as an example? It needs
a sourced figure; I could not find a reliable Slovak average yet. (b) Several
sources are marked "link to check"; they get checked before the topic ships.
(c) New rung from D-029: the sweater jolt, about 5 mJ if a body charged to
10 kV at 100 pF (½CV²); to be sourced.

**Q-017 — Menu order.** Default for now: Projects, Playground, Workbench,
Explore, History. Alternative: the configured landing section first, History
last.

**Q-018 — Child appeal.** → **Answered: one look for any curious mind** (D-018, D-020).

**Q-019 — zgbweb2 code.** *Deferred, by Peter's choice.*

**Q-020 — Comments, later.** (a) where: everywhere, only Workbench, only
projects; (b) moderation: approve before showing, or remove after; (c) access:
anonymous, name plus email, or an account; (d) children will comment, and
under GDPR a child below the digital consent age needs a parent's consent
(16 in Slovakia, as far as I know; check). Pre-moderation and no public email
display avoid most problems. Note D-023: commenters' names would be private
people's names on the site, so a nickname-only rule may fit better.

**Q-021 — Explore section name.** → **Answered (working): Explore / Objavuj.**
Small language note: *objavuj* means "discover"; the literal Slovak for
"explore" is *preskúmaj*. Objavuj is the more inviting word, so the pair
works even though it is not a literal translation. Keep it?

**Q-022 — Weapons on the energy ladder?** The draft has the Hiroshima bomb
and the largest bomb ever tested. Classic ladder rungs and good anchors
(a fully fuelled Starship stack ≈ 12 kt TNT), but the audience starts at 6.
Keep them, keep them with neutral wording, or drop them?

**Q-023 — Scope of "no private names".** My reading of D-023: no living
private people (family, friends, pupils, colleagues, the school), while
historical figures and credited authors are named. Does it also cover your
own real name (the site speaks as zgub), and living public figures, e.g. a
current scientist whose result a page uses?

**Q-024 — History section shape.** Options: (a) one short piece per person
(Faraday, Galileo, Descartes ...); (b) one piece per idea, with the people in
it ("how we learned the Earth moves"); (c) small "who figured this out?" boxes
inside topics, with History as their index. Also: how much philosophy, and
which way into it (Descartes via coordinates, Galileo via falling bodies)?

**Q-025 — The video channel, far future.** Not urgent; noted so the site
does not block it. (a) **Link or embed?** A plain link out to YouTube keeps
the site free of tracking. An embedded YouTube player loads Google's scripts
and cookies, which conflicts with D-024; even the "privacy-enhanced"
youtube-nocookie.com player is not tracking-free once played. A click-to-load
placeholder is a middle way. (b) **Identity:** does the channel run as zgub,
like the site? (c) **Hands only:** voices, the room, a school logo on a
notebook or a window view can identify a child as surely as a face. Worth a
quick look before each upload. (d) **Being seen:** YouTube is indexed and
recommended to strangers, so the channel works against "spread by word and
links" (D-021), and comments under the videos would need moderation like Q-020.
Unlisted videos linked only from the site are an option.
