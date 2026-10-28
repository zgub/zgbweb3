## zgub.net — English catalog
## Every key here must also exist in sk.ftl (a test checks both ways).
## Style: plain and concrete, for any curious mind (6 or 60). Things that can't
## want don't "want", "try" or "seek". No names of private people (D-023).

## --- Chrome ---------------------------------------------------------------

site-name = zgub.net
nav-home = Home
nav-menu = ☰ Menu
nav-menu-hover = Show or hide the menu
nav-settings = Settings
menu-title = Menu

theme-dark = Dark
theme-light = Light
theme-system = Follow my device
settings-size = Size
settings-size-reset = reset
settings-calm = Calm motion (fewer animations)
settings-language = Language
lang-auto = Automatic (from the browser)
settings-remember-title = Remember on this device
settings-remember = Remember my settings and experiments
settings-forget = 🗑 Forget everything
settings-forget-hover = Delete everything this site saved in your browser

consent-text = Should this page remember your settings and experiments on this device? They stay in your browser and are never sent anywhere.
consent-yes = Yes, remember
consent-no = No thanks

footer-privacy = Privacy
footer-license = License
footer-about = About
footer-powered = Powered by
footer-and = and

where-next = Where next?
grown-ups = For the grown-ups: where the numbers come from
model-alert = Model alert!
draft-note = Draft — to be finished before the site goes public.
sawdust = Nothing but sawdust
    .body = You are at the right place, just at the wrong time. Three of your four coordinates are spot on: x, y and z. Only t is off. Come back later.

status-planned = planned
status-draft = draft

## --- Sections -------------------------------------------------------------

section-projects = Projects
    .intro = Things I build as a hobby: electronics, radio, small computers, software.
section-playground = Playground
    .intro = Games that play with an idea. Each one links to the topic behind it.
section-workbench = Workbench
    .intro = Things I am still learning, written as I learn them. Unfinished, and sometimes wrong.
section-explore = Explore
    .intro = Things I understand and want to explain. Poke them, change the numbers, see what happens.
section-history = History
    .intro = Short pieces about the people behind the ideas.

## --- About, privacy, license ----------------------------------------------

about-body = zgub.net is my personal site: hobby projects, games, things I am learning and things I can explain. Everything runs in your browser.
about-tech = Built with Rust, egui and WebAssembly.
about-source = Source code link: coming later.

privacy-tracking = No tracking, no analytics, no targeted ads and no cookies. Not because a law says so: I don't like being tracked, so I don't track you.
    .title = No tracking
privacy-device = If you choose “Yes, remember”, your settings and the state of the experiments are saved in your browser's local storage on this device. They are never sent to us. Your answer to that question is remembered too, so we don't ask every time. Settings ⚙ → “Forget everything” deletes it all.
    .title = What stays on your device
privacy-logs = The web server that delivers this page may record standard technical data (IP address, time, requested address, browser type) to keep the service running and secure. How long they are kept: to be decided.
    .title = Server logs
privacy-changes = If accounts or comments are ever added, this page will be updated and you will be asked before anything new is stored.
    .title = If this changes
privacy-operator = Operator and contact: to be decided.
    .title = Who runs this site

license-code = Program code: GNU AGPL v3.0 or later (to be confirmed).
license-content = Texts and pictures by zgub: Creative Commons BY 4.0. Reuse them freely with a link or credit. Work by others is credited where it is used, under its own licence.
license-third = Parts made by others:
    .title = Third-party parts

## --- Pages: titles, and a blurb once a page exists -------------------------

page-flysch = Flysch shell journal
page-spatlers = Spatlers
page-electronics = Electronics

page-game-of-life = Conway's Game of Life
page-gas-box = Gas in a box

page-statistics = Statistics
page-probability = Probability
page-music = Music
page-linear-algebra = Linear algebra
page-tensor-calculus = Tensor calculus

page-rule-of-three = Rule of three
    .blurb = If 180 is 30 %, what is 100 %? One question, four pictures, and two cases where the trick lies.
page-percent = Percent tricks
page-linear = Same step
page-quadratic = Twice as fast
page-inverse = More hands
page-exponential = Doubling
page-logarithm = Counting zeros

page-numbers = Numbers
page-thermodynamics = Thermodynamics
page-energy = What is energy
page-chemistry = Chemistry
page-biology = Biology

## --- Cross-link questions ---------------------------------------------------

hook-percent-tricks = Why doesn't +20 % and then −20 % bring you back?
hook-taxi-start-fee = What if the line doesn't start at zero?
hook-more-hands = Why do more hands mean less time?

## --- Rule of three (rule-of-three) ----------------------------------------

r3-examples = Examples:
r3-ex-percent = Percent
    .story = The classic from school and from shops: you know a part, and how many percent it is.
r3-ex-pancakes = Pancakes
    .story = The recipe is for 4 people and needs 250 ml of milk. You are cooking for 6.
r3-ex-fuel = Fuel
    .story = The car uses 6.5 litres per 100 km. The trip is 340 km.
r3-ex-sale = Sale
    .story = After 30 % off, a shirt costs 42 €. So you pay 70 % of the old price. What was it?

r3-q-percent = If { $a } is { $b }, what is { $c }?
r3-q-pancakes = { $a } need { $b } of milk. How much for { $c }?
r3-q-fuel = { $a } take { $b }. How much for { $c }?
r3-q-sale = If { $a } is { $b }, what was { $c }?

r3-unit-percent-a = { $num } %
r3-unit-percent-b = { $num }
r3-unit-pancakes-a = { $num } { $n ->
        [one] person
       *[other] people
    }
r3-unit-pancakes-b = { $num } ml
r3-unit-fuel-a = { $num } km
r3-unit-fuel-b = { $num } l
r3-unit-sale-a = { $num } %
r3-unit-sale-b = { $num } €

r3-we-know = We know:
r3-is = →
r3-we-ask = We ask:
r3-no-answer = Start from something that isn't zero.
r3-your-guess = Your guess:
r3-check = Check
r3-show = Show the answer
r3-verdict-exact = Exactly!
r3-verdict-close = Close: within 10 %. A picture below gets you all the way.
r3-verdict-low = Too little. Look at the picture: how much is one piece?
r3-verdict-high = Too much. Look at the picture: how much is one piece?
r3-answer = { $c } → { $answer }
r3-sale-trap = The usual mistake: 42 € + 30 % = 54.60 €. But the 30 % was taken from the old price, not from 42 €.

r3-picture = Picture:
r3-pic-pieces = Pieces
    .hint = Cut both amounts into equal pieces. Find what one piece is worth, then count the pieces.
r3-pic-rulers = Two rulers
    .hint = The first quantity on top, the second below. Whatever you do on top, you do below.
r3-pic-line = A straight line
    .hint = Twice the one, twice the other: all the points lie on one straight line through zero.
r3-pic-squares = A hundred squares
    .hint = Per cent means "per hundred". One square is 1 %.
r3-squares-only-percent = Only when the first number is a percentage.
r3-too-many-pieces = Too many pieces to draw. Try the two rulers.
r3-is-pair = { $a } → { $b }
r3-one-piece = one piece: { $step } → { $b } ÷ { $down } = { $piece }
r3-all-pieces = { $c } → { $up } × { $piece } = { $answer }
r3-line-step = every { $step } more adds the same { $adds }
r3-sq-known = { $n } { $count ->
        [one] square is
       *[other] squares are
    } { $b }
r3-sq-one = 1 square is { $b } ÷ { $n } = { $per }
r3-sq-asked = { $n } { $count ->
        [one] square is
       *[other] squares are
    } { $n } × { $per } = { $answer }
r3-sq-percent = Percent is just counting squares out of a hundred.

r3-school-title = What school calls this
r3-school-direct = When twice the one means twice the other, school calls it direct proportion. Working out the missing number is the rule of three.
r3-school-k = One thing stays the same here: { $one } is always { $per }. School writes it as y = k · x, with k = { $k }.
r3-school-line = Its graph is a straight line through zero. Further down are a line that doesn't start at zero (the taxi) and a curve (the painters).
r3-school-words = Two more words you will hear: the domain is which numbers make sense to put in, the range is what can come out. Can you cook pancakes for −3 people? For half a person?

r3-inverse-title = More hands, less time
r3-inverse-story = 4 painters paint a house in 6 days. How long do 3 painters need? Fewer hands, so more days: here the rule of three works upside down.
r3-painters = { $n ->
        [one] painter
       *[other] painters
    }
r3-days = { $n ->
        [one] day
       *[other] days
    }
r3-how-long-for = How long for
r3-inverse-answer = The work is { $work } painter-days: the area of the rectangle. { $p } { $pn ->
        [one] painter needs
       *[other] painters need
    } { $work } ÷ { $p } = { $d } { $dn ->
        [one] day
       *[other] days
    }.
r3-grid-label = { $p } × { $d }
r3-inverse-cow = Every painter works equally fast and nobody gets in anyone's way.

r3-lies-title = When it lies
r3-lies-intro = The rule of three only works when twice the one really means twice the other. Two everyday cases where it doesn't:
r3-lie-charging = Charging a phone
    .story = 20 % in 15 minutes. Full in 75?
    .cow = A typical shape, not a measured phone.
r3-lie-charging-rule = rule of three: full at 75 min
r3-lie-charging-real = a real phone slows down near full
r3-lie-taxi = A taxi with a start fee
    .story = 5 km cost 9 €. Do 10 km cost 18 €?
    .cow = 3 € to get in, then 1.20 € per km. A straight line, but not through zero.
r3-lie-taxi-rule = rule of three: 10 km → 18 €
r3-lie-taxi-real = real fare: 10 km → 15 €
r3-rule = The rule of three works when twice the one means twice the other: a straight line through zero. Painters are the opposite: twice the hands, half the time. Phones and taxis follow neither.

r3-grown-formula = Direct: if a is b, then c is b × c ÷ a. Through one: b ÷ a is what one unit is worth, times c.
r3-grown-inverse = Inverse: a × b stays the same, so for c the answer is a × b ÷ c.
r3-grown-numbers = The pictures go through the biggest step that fits both numbers (for 30 % and 100 % it is 10 %), so every step is a whole number.
r3-grown-sources = No outside data on this page. The charging curve is a typical shape, not a measurement.
