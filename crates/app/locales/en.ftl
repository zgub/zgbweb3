## zgb — English catalog
## Every key here must also exist in sk.ftl (a test checks both ways).
## Style: short, concrete, for curious 9–14-year-olds. Things that can't want
## don't "want", "try" or "seek".

## --- Chrome ---------------------------------------------------------------

nav-home = Home
nav-topics = Topics
nav-topics-hover = Show or hide the list of topics
nav-settings = Settings

settings-look = Look
theme-dark = 🌙 Dark
theme-light = ☀ Light
theme-system = 💻 Follow my device
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

where-next = Where next?
grown-ups = For the grown-ups: where the numbers come from
model-alert = Model alert!
planned-intro = This one is still on the drawing board. The plan:
draft-note = Draft — to be finished before the site goes public.

maturity-planned = coming soon
maturity-draft = draft
maturity-ready = ready

group-scale = Scale & numbers
group-chance = Chance & statistics
group-physics = Physics
group-maths = Maths

## --- Home -----------------------------------------------------------------

home-title = Big numbers, small numbers, and how the world adds up
home-intro = Little experiments to poke at. Move the sliders, roll the dice, see what breaks. Made for curious kids — grown-ups welcome.

## --- About, privacy, license ----------------------------------------------

about-body = zgb is a small collection of interactive explainers about scale, chance, physics and maths. Everything runs in your browser; nothing you do here is sent anywhere.
about-tech = Built with Rust, egui and WebAssembly.
about-source = Source code link: coming later.

privacy-tracking = No accounts, no analytics, no advertising and no cookies.
    .title = No tracking
privacy-device = If you choose “Yes, remember”, your settings and the state of the experiments are saved in your browser's local storage on this device. They are never sent to us. Your answer to that question is remembered too, so we don't ask every time. Settings ⚙ → “Forget everything” deletes it all.
    .title = What stays on your device
privacy-logs = The web server that delivers this page may record standard technical data (IP address, time, requested address, browser type) to keep the service running and secure. How long they are kept: to be decided.
    .title = Server logs
privacy-changes = If advertising or accounts are ever added, this page will be updated and you will be asked before anything new is stored or shared.
    .title = If this changes
privacy-operator = Operator and contact: to be decided.
    .title = Who runs this site

license-code = Program code: GNU AGPL v3.0 or later.
license-content = Texts and pictures: Creative Commons BY-SA 4.0.
license-third = Parts made by others:
    .title = Third-party parts

## --- Topics: titles, blurbs, plans -----------------------------------------

topic-numbers = What is a number?
    .blurb = Counting, then zero, then minus, then halves, then √2 — and finally a number that turns things by 90°.
    .plan =
        One number line that grows with each idea: counting numbers → zero → negatives (thermometer, lift floors)
        Fractions and decimals: cut a bar into pieces; 0.1 is one slab of the powers-of-ten cube
        √2: the diagonal of a square is not any fraction (the classic proof, told as a story)
        π and friends: the line has no gaps — the real numbers
        i: multiplying by −1 turns around (180°); is there a number that turns only 90°? The line becomes a plane
        Each new kind of number answers a question the old kind could not (take away more than you have, share one cake between three, …)

topic-powers-of-ten = Powers of ten
    .blurb = One puzzle cube, ten in a row, a hundred in a slab, a thousand in a big cube. Then do it again — and again.

topic-energy-ladder = The energy ladder
    .blurb = How much energy has a flying mosquito? A car? The space station? Line them up and count them in sugar cubes.

topic-dice-bell = Why dice make a bell
    .blurb = One die is flat and fair. Add up two, five, twenty dice and a bell curve appears out of nowhere.

topic-gas-box = Gas in a box
    .blurb = Hundreds of bouncing balls make pressure and temperature — and slowly forget where they started.
    .plan =
        2D gas of hard discs, elastic collisions, adjustable number of particles and box size
        Speed histogram that settles into the Maxwell–Boltzmann distribution (in 2D: Rayleigh)
        Pressure from hits on the walls; push a piston and watch the gas warm up
        Two gases behind a wall: remove it, watch them mix (and never un-mix)
        Physics in sims::gas, drawn with the egui painter first, wgpu if we need many particles

topic-linear-maps = Matrices move the plane
    .blurb = A 2×2 matrix is just where the two arrows î and ĵ end up. Drag them and watch the whole grid follow.
    .plan =
        Draggable basis arrows î, ĵ; the grid and a picture (a house, a smiley) change live
        Determinant as the area of the unit square; negative = the picture flips over
        Composition: first A, then B — compare with B·A
        Eigenvectors: the arrows that only stretch, highlighted

## --- "Where next?" hooks ----------------------------------------------------

hook-how-big-is-a-billion = How big is a billion, really?
hook-turning-with-a-matrix = Turning by 90° with a matrix
hook-why-random-makes-a-bell = Why do random things make a bell?
hook-energy-of-one-molecule = How much energy has one jiggling molecule?
hook-numbers-that-turn = Numbers that turn things: i
hook-what-is-1e9 = What does 10⁹ actually mean?
hook-heat-is-jiggling = Where does heat come from? Jiggling molecules
hook-molecules-make-a-bell = Billions of molecules, one bell-shaped speed curve
hook-what-is-1e13-joules = What is 10¹³ joules? Meet the energy ladder
hook-what-kind-of-number = 0.001 — what kind of number is that?

## --- Energy ladder ----------------------------------------------------------

energy-count-in = Count energy in
unit-joule = joules
unit-electronvolt = electronvolts
unit-kilocalorie = kilocalories (food calories)
unit-kilowatt-hour = kilowatt-hours (like on the electricity bill)
unit-sugar-cube = sugar cubes (eaten)
unit-mosquito = flying mosquitoes

# $shown = the number as text, $count = the same number, for picking the plural
amount-sugar-cube = { $count ->
    [one] { $shown } sugar cube
   *[other] { $shown } sugar cubes
}
amount-mosquito = { $count ->
    [one] { $shown } flying mosquito
   *[other] { $shown } flying mosquitoes
}

energy-make-own = Make your own moving thing
energy-own-energy = Moving energy: { $energy }
energy-ex-mine = Your thing

energy-ex-thermal = One jiggling air molecule (25 °C)
    .note = Boltzmann constant × room temperature: the typical energy of thermal jiggling, per direction of motion.
energy-ex-photon = One photon of green light
    .note = Wavelength 530 nm, E = h·c/λ ≈ 2.3 eV.
energy-ex-mosquito = A flying mosquito
    .note = 2 mg at 0.4 m/s (about 1.5 km/h). Roughly one tera-electronvolt.
energy-ex-lhc = One proton in the LHC
    .note = 6.8 TeV per proton (LHC Run 3). A few mosquitoes' worth — in one particle.
energy-ex-apple = Lifting an apple by one metre
    .note = A 102 g apple raised by 1 m: about one joule. A handy picture for “1 J”.
energy-ex-tennis = A fast tennis serve
    .note = 58 g ball at 200 km/h.
energy-ex-aa = An AA battery
    .note = Alkaline AA: about 2.5 Ah at about 1.2 V ≈ 3 Wh.
energy-ex-sugar = One sugar cube (eaten)
    .note = 4 g of sugar × about 16.5 kJ per gram.
energy-ex-car50 = Family car at 50 km/h
    .note = 1500 kg at town speed.
energy-ex-car130 = Family car at 130 km/h
    .note = Same car on the motorway: 2.6× the speed, 6.8× the energy (speed is squared).
energy-ex-food_day = A grown-up's food for one day
    .note = 2000 kcal.
energy-ex-lightning = A lightning bolt
    .note = Very rough: one bolt is often quoted at about a gigajoule; real bolts vary a lot.
energy-ex-b747_fuel = Full fuel tanks of a Boeing 747-400
    .note = 216 840 L of jet fuel × 0.80 kg/L × 43 MJ/kg (chemical energy).
energy-ex-iss = The ISS flying in orbit
    .note = 420 t at 7.66 km/s (moving energy only). More than a full jumbo jet's fuel.

energy-grid = Each grid line is ×10.
energy-compare = Compare two
energy-ratio = { $big } has { $ratio }× the energy of { $small }.
energy-orders = That is { $orders } { $n ->
    [one] order
   *[other] orders
} of magnitude: about { $pow } — multiply by ten, { $times } times in a row.
energy-model = We treat every moving thing as one solid lump at one speed: no spinning wheels, no flapping wings, no wobble.
energy-notes-intro = Moving (kinetic) energy is ½·m·v². Twice the speed means four times the energy.

## --- Dice and the bell curve --------------------------------------------------

dice-n = dice
dice-sides = sides each
dice-roll = Roll ×{ $n }
dice-keep = ▶ Keep rolling
dice-restart = ⟲ Start over
dice-curve = Show the bell (Gauss) curve
dice-empty = Roll some dice!
dice-summary = { $rolls ->
    [one] { $rolls } roll
   *[other] { $rolls } rolls
}. Average sum: { $mean } (theory says { $theory }).
dice-notes = Sum of n fair dice with s sides each: mean n·(s+1)/2 = { $mean }, standard deviation √(n·(s²−1)/12) = { $sd }. The orange curve is the normal (Gauss) curve with that mean and spread, scaled to the number of rolls. The central limit theorem says that adding up many independent random things gives roughly this shape, whatever the shape of each one.

## --- Powers of ten --------------------------------------------------------------

pot-name-p0 = one
pot-name-p1 = ten
pot-name-p2 = one hundred
pot-name-p3 = one thousand
pot-name-p4 = ten thousand
pot-name-p5 = one hundred thousand
pot-name-p6 = one million
pot-name-p7 = ten million
pot-name-p8 = one hundred million
pot-name-p9 = one billion
pot-name-p10 = ten billion
pot-name-p11 = one hundred billion
pot-name-p12 = one trillion
pot-name-p13 = ten trillion
pot-name-p14 = one hundred trillion
pot-name-p15 = one quadrillion
pot-name-p16 = ten quadrillion
pot-name-p17 = one hundred quadrillion
pot-name-p18 = one quintillion
pot-name-m1 = one tenth
pot-name-m2 = one hundredth
pot-name-m3 = one thousandth
pot-name-m4 = one ten-thousandth
pot-name-m5 = one hundred-thousandth
pot-name-m6 = one millionth
pot-name-m7 = one ten-millionth
pot-name-m8 = one hundred-millionth
pot-name-m9 = one billionth
pot-name-m10 = one ten-billionth
pot-name-m11 = one hundred-billionth
pot-name-m12 = one trillionth
pot-name-m13 = one ten-trillionth
pot-name-m14 = one hundred-trillionth
pot-name-m15 = one quadrillionth
pot-name-m16 = one ten-quadrillionth
pot-name-m17 = one hundred-quadrillionth
pot-name-m18 = one quintillionth

si-m30 = quecto
si-m27 = ronto
si-m24 = yocto
si-m21 = zepto
si-m18 = atto
si-m15 = femto
si-m12 = pico
si-m9 = nano
si-m6 = micro
si-m3 = milli
si-m2 = centi
si-m1 = deci
si-p1 = deca
si-p2 = hecto
si-p3 = kilo
si-p6 = mega
si-p9 = giga
si-p12 = tera
si-p15 = peta
si-p18 = exa
si-p21 = zetta
si-p24 = yotta
si-p27 = ronna
si-p30 = quetta

pot-prefix-table = All the SI prefixes, from quecto to quetta
pot-col-power = power
pot-col-prefix = prefix
pot-col-symbol = symbol
pot-legend = Colour key:
pot-legend-cube = 1 puzzle cube ({ $side })
pot-legend-block = { $count } cubes ({ $side })

pot-story-one = One puzzle cube, { $cube } on each side, sitting in the middle of its own invisible { $box } box. Ten boxes in a row make exactly one metre — with { $gap } of air between neighbouring cubes.
pot-story = { $count } puzzle cubes, each in its own 10 cm box, make { $shape ->
    [row] a row { $len } long.
    [slab] a square { $len } × { $len }, one cube thick.
   *[cube] a cube { $len } on each side.
}
pot-hint = That's about { $thing }.
pot-weight = Together they would weigh about { $mass }.
pot-story-neg = Cut a 1 m cube into { $count } equal { $shape ->
    [slab] slabs, each { $big } × { $big } × { $thin }.
    [stick] sticks, each { $big } long and { $thin } × { $thin } thick.
   *[cube] little cubes, each { $thin } on a side.
}

size-hint-k1 = a box just big enough for you to sit inside
size-hint-k2 = a three-storey house
size-hint-k3 = a 30-storey tower block
size-hint-k4 = a big hill — taller than any building ever built
size-hint-k5 = a mountain higher than Mount Everest — airliners fly at about this height
size-hint-k6 = a cube reaching the edge of space (100 km up)
size-hint-neg3 = a puzzle cube's invisible box
size-hint-neg6 = a sugar cube
size-hint-neg9 = a grain of sand
size-hint-neg12 = the thickness of a hair
size-hint-neg15 = a cell in your body
size-hint-neg18 = a small bacterium

pot-model = Real puzzle cubes don't float in the air. We give every cube its own 10 cm box so that ten cubes make exactly one metre — the counting stays honest and the numbers stay round.
pot-kid = kid, { $h }
pot-neg-zoomed = Zoomed in: the faint cube here is { $size } on a side.
pot-notes =
    Every third step the pattern restarts: 1000 blocks join into one new, bigger block (with a new colour).
    A standard 3×3 puzzle cube is 57 mm on a side (speed cubes are 55–56 mm) and weighs 70–100 g; we use 80 g.
    Each cube sits in a 10 cm box, so there are 4.3 cm of air between neighbours and ten boxes are exactly 1 m.
    Big blocks are drawn with their faces showing the blocks one level down, so a face of a 1 m block shows 10 × 10 puzzle cubes.
    Number names use the short scale (billion = 10⁹, trillion = 10¹²). Slovak and most of Europe use the long scale (miliarda = 10⁹, bilión = 10¹²).
