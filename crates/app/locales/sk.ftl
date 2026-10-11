## zgb — slovenský katalóg
## Každý kľúč musí byť aj v en.ftl (test to kontroluje oboma smermi).
## Štýl: tykáme, krátko a konkrétne, pre zvedavé deti 9–14 rokov. Veci, ktoré
## nemôžu nič chcieť, nič „nechcú“, „neskúšajú“ ani „nehľadajú“.

## --- Rám stránky -------------------------------------------------------------

nav-home = Domov
nav-topics = Témy
nav-topics-hover = Zobraziť alebo skryť zoznam tém
nav-settings = Nastavenia

settings-look = Vzhľad
theme-dark = 🌙 Tmavý
theme-light = ☀ Svetlý
theme-system = 💻 Podľa zariadenia
settings-size = Veľkosť
settings-size-reset = pôvodná
settings-calm = Pokojný pohyb (menej animácií)
settings-language = Jazyk
lang-auto = Automaticky (podľa prehliadača)
settings-remember-title = Pamätať si na tomto zariadení
settings-remember = Pamätaj si moje nastavenia a pokusy
settings-forget = 🗑 Zabudni všetko
settings-forget-hover = Vymaže všetko, čo si táto stránka uložila v tvojom prehliadači

consent-text = Má si táto stránka pamätať tvoje nastavenia a pokusy na tomto zariadení? Zostanú len v tvojom prehliadači a nikam sa neposielajú.
consent-yes = Áno, pamätaj si
consent-no = Nie, ďakujem

footer-privacy = Súkromie
footer-license = Licencia
footer-about = O stránke

where-next = Kam ďalej?
grown-ups = Pre dospelých: odkiaľ sú tie čísla
model-alert = Pozor, model!
planned-intro = Táto téma je zatiaľ len na rysovacej doske. Plán:
draft-note = Návrh — pred zverejnením stránky ho ešte dokončíme.

maturity-planned = čoskoro
maturity-draft = rozpracované
maturity-ready = hotové

group-scale = Veľkosti a čísla
group-chance = Náhoda a štatistika
group-physics = Fyzika
group-maths = Matematika

## --- Domov -------------------------------------------------------------------

home-title = Veľké čísla, malé čísla a ako to celé do seba zapadá
home-intro = Malé pokusy, do ktorých môžeš šťuchať. Hýb posuvníkmi, hádž kockami, skúšaj, čo sa pokazí. Pre zvedavé deti — dospelí sú tiež vítaní.

## --- O stránke, súkromie, licencia ----------------------------------------------

about-body = zgb je malá zbierka interaktívnych vysvetlení o veľkostiach, náhode, fyzike a matematike. Všetko beží v tvojom prehliadači; nič z toho, čo tu robíš, sa nikam neposiela.
about-tech = Vyrobené v jazyku Rust s knižnicou egui a technológiou WebAssembly.
about-source = Odkaz na zdrojový kód: neskôr.

privacy-tracking = Žiadne účty, žiadna analytika, žiadne reklamy a žiadne cookies.
    .title = Žiadne sledovanie
privacy-device = Ak zvolíš „Áno, pamätaj si“, tvoje nastavenia a stav pokusov sa uložia do lokálneho úložiska prehliadača na tomto zariadení. Nikdy sa neposielajú nám. Zapamätá sa aj tvoja odpoveď na túto otázku, aby sme sa nepýtali zakaždým. Nastavenia ⚙ → „Zabudni všetko“ to celé vymaže.
    .title = Čo zostáva v tvojom zariadení
privacy-logs = Webový server, ktorý túto stránku doručuje, môže zaznamenávať bežné technické údaje (IP adresu, čas, požadovanú adresu, typ prehliadača), aby služba fungovala a bola bezpečná. Ako dlho sa uchovávajú: zatiaľ nerozhodnuté.
    .title = Záznamy servera
privacy-changes = Ak by sa niekedy pridali reklamy alebo účty, táto stránka sa aktualizuje a pred uložením alebo zdieľaním čohokoľvek nového sa ťa opýtame.
    .title = Ak sa niečo zmení
privacy-operator = Prevádzkovateľ a kontakt: zatiaľ nerozhodnuté.
    .title = Kto prevádzkuje túto stránku

license-code = Programový kód: GNU AGPL v3.0 alebo novšia.
license-content = Texty a obrázky: Creative Commons BY-SA 4.0.
license-third = Časti, ktoré vyrobili iní:
    .title = Časti tretích strán

## --- Témy: názvy, popisy, plány -----------------------------------------------

topic-numbers = Čo je to číslo?
    .blurb = Počítanie, potom nula, potom mínus, potom polovice, potom √2 — a nakoniec číslo, ktoré otáča veci o 90°.
    .plan =
        Jedna číselná os, ktorá rastie s každým nápadom: prirodzené čísla → nula → záporné čísla (teplomer, poschodia vo výťahu)
        Zlomky a desatinné čísla: rozkrájaj tyčku na kúsky; 0,1 je jeden plát kocky z mocnín desiatky
        √2: uhlopriečka štvorca sa nedá zapísať ako zlomok (klasický dôkaz, vyrozprávaný ako príbeh)
        π a spol.: na osi nie sú žiadne diery — reálne čísla
        i: násobenie −1 otáča o 180°; existuje číslo, ktoré otočí len o 90°? Z osi sa stane rovina
        Každý nový druh čísel odpovedá na otázku, na ktorú starý druh nestačil (zober viac, ako máš; rozdeľ jednu tortu medzi troch; …)

topic-powers-of-ten = Mocniny desiatky
    .blurb = Jedna kocka-hlavolam, desať v rade, sto v pláte, tisíc vo veľkej kocke. A potom znova — a znova.

topic-energy-ladder = Energetický rebrík
    .blurb = Koľko energie má letiaci komár? Auto? Vesmírna stanica? Zoraď ich vedľa seba a počítaj v kockách cukru.

topic-dice-bell = Prečo kocky robia zvon
    .blurb = Jedna kocka je plochá a férová. Sčítaj dve, päť, dvadsať kociek a zrazu sa objaví zvonová krivka.

topic-gas-box = Plyn v krabici
    .blurb = Stovky poskakujúcich guličiek vytvoria tlak a teplotu — a postupne zabudnú, kde začínali.
    .plan =
        2D plyn z tvrdých diskov, pružné zrážky, nastaviteľný počet častíc a veľkosť krabice
        Histogram rýchlostí, ktorý sa ustáli na Maxwellovom–Boltzmannovom rozdelení (v 2D: Rayleighovom)
        Tlak z nárazov na steny; zatlač piest a sleduj, ako sa plyn zohreje
        Dva plyny oddelené stenou: odstráň ju a sleduj, ako sa zmiešajú (a už nikdy neodmiešajú)
        Fyzika v sims::gas, kreslenie najprv cez egui painter, wgpu ak bude treba veľa častíc

topic-linear-maps = Matice hýbu rovinou
    .blurb = Matica 2×2 je len to, kam doletia dve šípky î a ĵ. Ťahaj ich a sleduj, ako sa s nimi pohne celá mriežka.
    .plan =
        Šípky î, ĵ, ktoré sa dajú ťahať; mriežka a obrázok (domček, smajlík) sa menia naživo
        Determinant ako plocha jednotkového štvorca; záporný = obrázok sa preklopí
        Skladanie: najprv A, potom B — porovnaj s B·A
        Vlastné vektory: zvýraznené šípky, ktoré sa iba natiahnu

## --- „Kam ďalej?“ otázky ------------------------------------------------------

hook-how-big-is-a-billion = Aká veľká je vlastne miliarda?
hook-turning-with-a-matrix = Otáčanie o 90° maticou
hook-why-random-makes-a-bell = Prečo náhoda robí zvon?
hook-energy-of-one-molecule = Koľko energie má jedna roztrasená molekula?
hook-numbers-that-turn = Čísla, ktoré otáčajú: i
hook-what-is-1e9 = Čo vlastne znamená 10⁹?
hook-heat-is-jiggling = Odkiaľ sa berie teplo? Z trasúcich sa molekúl
hook-molecules-make-a-bell = Miliardy molekúl, jedna zvonová krivka rýchlostí
hook-what-is-1e13-joules = Čo je 10¹³ joulov? Pozri energetický rebrík
hook-what-kind-of-number = 0,001 — aké je to vlastne číslo?

## --- Energetický rebrík --------------------------------------------------------

energy-count-in = Energiu počítaj v
unit-joule = jouloch
unit-electronvolt = elektrónvoltoch
unit-kilocalorie = kilokalóriách (kalórie v jedle)
unit-kilowatt-hour = kilowatthodinách (ako na účte za elektrinu)
unit-sugar-cube = kockách cukru (zjedených)
unit-mosquito = letiacich komároch

# $shown = číslo ako text, $count = to isté číslo na výber tvaru (1 / 2–4 / desatinné / 5+)
amount-sugar-cube = { $count ->
    [one] { $shown } kocka cukru
    [few] { $shown } kocky cukru
    [many] { $shown } kocky cukru
   *[other] { $shown } kociek cukru
}
amount-mosquito = { $count ->
    [one] { $shown } letiaci komár
    [few] { $shown } letiace komáre
    [many] { $shown } letiaceho komára
   *[other] { $shown } letiacich komárov
}

energy-make-own = Vyrob si vlastnú vec v pohybe
energy-own-energy = Pohybová energia: { $energy }
energy-ex-mine = Tvoja vec

energy-ex-thermal = Jedna roztrasená molekula vzduchu (25 °C)
    .note = Boltzmannova konštanta × izbová teplota: typická energia tepelného trasenia na jeden smer pohybu.
energy-ex-photon = Jeden fotón zeleného svetla
    .note = Vlnová dĺžka 530 nm, E = h·c/λ ≈ 2,3 eV.
energy-ex-mosquito = Letiaci komár
    .note = 2 mg pri 0,4 m/s (asi 1,5 km/h). Zhruba jeden teraelektrónvolt.
energy-ex-lhc = Jeden protón v LHC
    .note = 6,8 TeV na protón (LHC, Run 3). Energia niekoľkých komárov — v jedinej častici.
energy-ex-apple = Zdvihnúť jablko o meter
    .note = Jablko s hmotnosťou 102 g zdvihnuté o 1 m: asi jeden joule. Šikovná predstava pre „1 J“.
energy-ex-tennis = Rýchle tenisové podanie
    .note = Loptička 58 g pri 200 km/h.
energy-ex-aa = Tužková batéria AA
    .note = Alkalická AA: asi 2,5 Ah pri asi 1,2 V ≈ 3 Wh.
energy-ex-sugar = Jedna kocka cukru (zjedená)
    .note = 4 g cukru × asi 16,5 kJ na gram.
energy-ex-car50 = Rodinné auto pri 50 km/h
    .note = 1500 kg, rýchlosť v meste.
energy-ex-car130 = Rodinné auto pri 130 km/h
    .note = To isté auto na diaľnici: 2,6× vyššia rýchlosť, 6,8× viac energie (rýchlosť sa umocňuje na druhú).
energy-ex-food_day = Jedlo dospelého na jeden deň
    .note = 2000 kcal.
energy-ex-lightning = Blesk
    .note = Veľmi zhruba: pri jednom blesku sa často uvádza okolo jedného gigajoulu; skutočné blesky sa veľmi líšia.
energy-ex-b747_fuel = Plné nádrže Boeingu 747-400
    .note = 216 840 l leteckého paliva × 0,80 kg/l × 43 MJ/kg (chemická energia).
energy-ex-iss = ISS letiaca na obežnej dráhe
    .note = 420 t pri 7,66 km/s (iba pohybová energia). Viac ako plné nádrže jumbo jetu.

energy-grid = Každá čiara mriežky je ×10.
energy-compare = Porovnaj dve veci
energy-ratio = { $big } má { $ratio }× toľko energie ako { $small }.
energy-orders = To je { $orders } { $n ->
    [one] rád
    [few] rády
    [many] rádu
   *[other] rádov
} veľkosti: zhruba { $pow } — vynásob desiatimi { $times }-krát za sebou.
energy-model = Každú vec v pohybe berieme ako jeden pevný kus, ktorý letí jednou rýchlosťou: žiadne točiace sa kolesá, žiadne mávajúce krídla, žiadne kmitanie.
energy-notes-intro = Pohybová (kinetická) energia je ½·m·v². Dvakrát vyššia rýchlosť znamená štyrikrát viac energie.

## --- Kocky a zvonová krivka -------------------------------------------------------

dice-n = kocky
dice-sides = stien na kocke
dice-roll = Hoď ×{ $n }
dice-keep = ▶ Hádž ďalej
dice-restart = ⟲ Odznova
dice-curve = Ukáž zvonovú (Gaussovu) krivku
dice-empty = Hoď si kockami!
dice-summary = { $rolls ->
    [one] { $rolls } hod
    [few] { $rolls } hody
   *[other] { $rolls } hodov
}. Priemerný súčet: { $mean } (teória hovorí { $theory }).
dice-notes = Súčet n férových kociek, každá má s stien: priemer n·(s+1)/2 = { $mean }, smerodajná odchýlka √(n·(s²−1)/12) = { $sd }. Oranžová krivka je normálna (Gaussova) krivka s týmto priemerom a odchýlkou, zväčšená podľa počtu hodov. Centrálna limitná veta hovorí, že súčet mnohých nezávislých náhodných vecí dáva približne tento tvar — bez ohľadu na to, aký tvar má každá z nich.

## --- Mocniny desiatky ----------------------------------------------------------------
## Mená čísel v dlhej škále: miliarda = 10⁹, bilión = 10¹², biliarda = 10¹⁵, trilión = 10¹⁸.

pot-name-p0 = jeden
pot-name-p1 = desať
pot-name-p2 = sto
pot-name-p3 = tisíc
pot-name-p4 = desaťtisíc
pot-name-p5 = stotisíc
pot-name-p6 = milión
pot-name-p7 = desať miliónov
pot-name-p8 = sto miliónov
pot-name-p9 = miliarda
pot-name-p10 = desať miliárd
pot-name-p11 = sto miliárd
pot-name-p12 = bilión
pot-name-p13 = desať biliónov
pot-name-p14 = sto biliónov
pot-name-p15 = biliarda
pot-name-p16 = desať biliárd
pot-name-p17 = sto biliárd
pot-name-p18 = trilión
pot-name-m1 = jedna desatina
pot-name-m2 = jedna stotina
pot-name-m3 = jedna tisícina
pot-name-m4 = jedna desaťtisícina
pot-name-m5 = jedna stotisícina
pot-name-m6 = jedna milióntina
pot-name-m7 = jedna desaťmilióntina
pot-name-m8 = jedna stomilióntina
pot-name-m9 = jedna miliardtina
pot-name-m10 = jedna desaťmiliardtina
pot-name-m11 = jedna stomiliardtina
pot-name-m12 = jedna bilióntina
pot-name-m13 = jedna desaťbilióntina
pot-name-m14 = jedna stobilióntina
pot-name-m15 = jedna biliardtina
pot-name-m16 = jedna desaťbiliardtina
pot-name-m17 = jedna stobiliardtina
pot-name-m18 = jedna trilióntina

si-m30 = quekto
si-m27 = ronto
si-m24 = yokto
si-m21 = zepto
si-m18 = atto
si-m15 = femto
si-m12 = piko
si-m9 = nano
si-m6 = mikro
si-m3 = mili
si-m2 = centi
si-m1 = deci
si-p1 = deka
si-p2 = hekto
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

pot-prefix-table = Všetky predpony SI, od quekto po quetta
pot-col-power = mocnina
pot-col-prefix = predpona
pot-col-symbol = značka
pot-legend = Farby:
pot-legend-cube = 1 kocka-hlavolam ({ $side })
pot-legend-block = { $count } kociek ({ $side })

pot-story-one = Jedna kocka-hlavolam s hranou { $cube } sedí uprostred svojej neviditeľnej škatuľky s hranou { $box }. Desať škatuliek v rade má presne jeden meter — a medzi susednými kockami ostane { $gap } vzduchu.
pot-story = { $count } kociek-hlavolamov, každá vo svojej 10 cm škatuľke, tvorí { $shape ->
    [row] rad dlhý { $len }.
    [slab] štvorec { $len } × { $len }, hrubý jednu kocku.
   *[cube] kocku s hranou { $len }.
}
pot-hint = To je zhruba { $thing }.
pot-weight = Spolu by vážili asi { $mass }.
pot-story-neg = Rozkrájaj kocku s hranou 1 m na { $count } rovnakých { $shape ->
    [slab] plátov, každý { $big } × { $big } × { $thin }.
    [stick] tyčiniek, každá dlhá { $big } a hrubá { $thin } × { $thin }.
   *[cube] kociek, každá s hranou { $thin }.
}

size-hint-k1 = škatuľa, do ktorej sa akurát zmestíš
size-hint-k2 = trojposchodový dom
size-hint-k3 = 30-poschodový panelák
size-hint-k4 = veľký kopec — vyšší ako ktorákoľvek budova na svete
size-hint-k5 = hora vyššia ako Mount Everest — dopravné lietadlá lietajú zhruba v tejto výške
size-hint-k6 = kocka siahajúca až na okraj vesmíru (100 km nad zemou)
size-hint-neg3 = neviditeľná škatuľka jednej kocky-hlavolamu
size-hint-neg6 = kocka cukru
size-hint-neg9 = zrnko piesku
size-hint-neg12 = hrúbka vlasu
size-hint-neg15 = bunka v tvojom tele
size-hint-neg18 = malá baktéria

pot-model = Skutočné kocky-hlavolamy sa vo vzduchu nevznášajú. Každej kocke dávame vlastnú 10 cm škatuľku, aby desať kociek malo presne jeden meter — počítanie tak zostane poctivé a čísla okrúhle.
pot-kid = dieťa, { $h }
pot-neg-zoomed = Priblížené: bledá kocka tu má hranu { $size }.
pot-notes =
    Každý tretí krok sa vzor opakuje: 1000 blokov sa spojí do jedného nového, väčšieho bloku (s novou farbou).
    Štandardná kocka-hlavolam 3×3 má hranu 57 mm (rýchlostné kocky 55–56 mm) a váži 70–100 g; počítame s 80 g.
    Každá kocka sedí v 10 cm škatuľke, takže medzi susedmi je 4,3 cm vzduchu a desať škatuliek má presne 1 m.
    Veľké bloky kreslíme tak, že na ich stenách vidno bloky o úroveň menšie — stena 1 m bloku ukazuje 10 × 10 kociek-hlavolamov.
    Mená čísel sú v dlhej škále (miliarda = 10⁹, bilión = 10¹²). Angličtina používa krátku škálu (billion = 10⁹, trillion = 10¹²).
