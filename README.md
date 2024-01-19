## Notice: this route planner has been hacked together and likely has many bugs. Verify all routes before using them.

# YAERP - Yet Another Eve Route Planner
#### (it just rolls off the tongue, doesn't it?)

## Usage

This is a CLI command because I didn't feel like making a UI for it. As such, it is likely not useful for people who aren't familiar with command lines.

While the command's `--help` is exhaustive, here is the full help text, followed by a few examples.

Note that the filtering is unidirectional; excluding a system will allow jumps /out/, but not /in/. Additionally, the route can jump between filtered systems if there is no other option, but these routes are heavily penalized (one filtered jump is effectively a thousand normal jumps in the distance calculation).

A filtered system will never be entered, unless the `--no-filter` flag is added, which will keep the penalty but not remove the jump.

Long flags which accept an argument can be specified several times.

Arguments which accept a system or region name will first look for the exact match (case sensitive), then do a wildcard search (contains substring - case insensitive). You may need to enter the system name exactly, potentially with double quotes if there's a space in the name. Wildcard searches that have multiple matches will return an error.

```
$ yaerp --help
Usage: yaerp [OPTIONS] <WAYPOINTS> <WAYPOINTS>...

Arguments:
  <WAYPOINTS> <WAYPOINTS>...
          The systems to travel through (first is the start, last is the end).
          
          2 or more systems must be specified.
          
          The most optimal route is chosen, if more than 3 are entered (respects the start and end systems).

Options:
  -w, --wormholes <WORMHOLE_BOOKMARKS>
          A file containing copy+pasted bookmarks from the locations window in eve.
          
          Each line must match this regex: ^[A-Z]{3}-\d{3} +[\w\-]+ +(-&gt; +[\w\-]+|\(\w+\))\t([^\t]+\t){2}[^\t]+

  -a, --ansiblexes <ANSIBLEX_FILES>
          A file containing SMT-compatible connections for ansiblexes.
          
          Each line must match this regex: ^(#.*|\d+\s+[\w\-]+\s+-->\s+[\w\-]+)?$

      --no-filter
          Filtered jumps are not removed, but the penalties are still applied.
          
          A filtered jump is counted as 1000 jumps in the distance calculation.

      --no-special
          Routes will never enter Thera, Turnur, Zarzakh, or Pochven, and will try to get out as soon as possible

      --no-jspace
          Routes will never enter j-space, and will try to get out as soon as possible

      --no-nullsec
          Routes will never enter nullsec, and will try to get out as soon as possible

      --no-lowsec
          Routes will never enter lowsec, and will try to get out as soon as possible

      --no-highsec
          Routes will never enter highsec, and will try to get out as soon as possible

  -r, --region-blacklist <REGION_BLACKLIST>
          Routes will never enter this region, and will try to get out as soon as possible

  -n, --ns-region-whitelist <NS_REGION_WHITELIST>
          When set, only these nullsec regions will be enterable.
          
          Does not effect lowsec or highsec.

  -s, --system-blacklist <SYSTEM_BLACKLIST>
          Routes will never enter this system, and will try to get out as soon as possible

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Route through the trade hubs (fastest)

```
$ yaerp Jita Rens Dodixie Amarr Jita     

Best route:

From Jita to Dodixie: (12 jumps)
  Jita -> Ikuchi (0.99, The Forge, via gate)
  Ikuchi -> Tunttaras (0.89, Lonetrek, via gate)
  Tunttaras -> Nourvukaiken (0.82, Lonetrek, via gate)
  Nourvukaiken -> Tama (0.28, The Citadel, via gate)
  Tama -> Sujarento (0.31, The Citadel, via gate)
  Sujarento -> Onatoh (0.30, The Citadel, via gate)
  Onatoh -> Tannolen (0.30, The Citadel, via gate)
  Tannolen -> Tierijev (0.84, Verge Vendor, via gate)
  Tierijev -> Chantrousse (0.64, Verge Vendor, via gate)
  Chantrousse -> Ourapheh (0.86, Genesis, via gate)
  Ourapheh -> Botane (0.88, Sinq Laison, via gate)
  Botane -> Dodixie (0.87, Sinq Laison, via gate)

From Dodixie to Rens: (10 jumps)
  Dodixie -> Meves (0.98, Everyshore, via gate)
  Meves -> Lirsautton (0.84, Everyshore, via gate)
  Lirsautton -> Ardallabier (0.71, Everyshore, via gate)
  Ardallabier -> Jel (0.61, Sinq Laison, via gate)
  Jel -> Egghelende (0.45, Sinq Laison, via gate)
  Egghelende -> Siseide (0.34, Heimatar, via gate)
  Siseide -> Amamake (0.44, Heimatar, via gate)
  Amamake -> Osoggur (0.53, Heimatar, via gate)
  Osoggur -> Abudban (0.73, Heimatar, via gate)
  Abudban -> Rens (0.89, Heimatar, via gate)

From Rens to Amarr: (11 jumps)
  Rens -> Abudban (0.73, Heimatar, via gate)
  Abudban -> Osoggur (0.53, Heimatar, via gate)
  Osoggur -> Amamake (0.44, Heimatar, via gate)
  Amamake -> Auga (0.39, Heimatar, via gate)
  Auga -> Kourmonen (0.36, The Bleak Lands, via gate)
  Kourmonen -> Huola (0.37, The Bleak Lands, via gate)
  Huola -> Otelen (0.78, The Bleak Lands, via gate)
  Otelen -> Mahrokht (0.86, Domain, via gate)
  Mahrokht -> Alkabsi (0.66, Domain, via gate)
  Alkabsi -> Sarum Prime (1.00, Domain, via gate)
  Sarum Prime -> Amarr (1.00, Domain, via gate)

From Amarr to Jita: (11 jumps)
  Amarr -> Ashab (0.91, Domain, via gate)
  Ashab -> Kehour (0.92, Domain, via gate)
  Kehour -> Akhragan (0.80, Domain, via gate)
  Akhragan -> Zororzih (0.74, Kador, via gate)
  Zororzih -> Gensela (0.69, Kador, via gate)
  Gensela -> Shera (0.60, Genesis, via gate)
  Shera -> Ahbazon (0.42, Genesis, via gate)
  Ahbazon -> Hykkota (0.82, The Forge, via gate)
  Hykkota -> Ansila (0.91, The Forge, via gate)
  Ansila -> Ikuchi (0.99, The Forge, via gate)
  Ikuchi -> Jita (0.95, The Forge, via gate)

Shorthand route:
  Jita
  Dodixie
  Rens
  Amarr
  Jita

Total jumps: 44
```

### Route through the trade hubs (without any lowsec)
```
$ yaerp Jita Rens Dodixie Amarr Jita --no-lowsec

Best route:

From Jita to Rens: (25 jumps)
  Jita -> Perimeter (0.95, The Forge, via gate)
  Perimeter -> Urlen (0.96, The Forge, via gate)
  Urlen -> Sirppala (0.88, The Citadel, via gate)
  Sirppala -> Anttiri (0.71, The Citadel, via gate)
  Anttiri -> Juunigaishi (0.62, The Citadel, via gate)
  Juunigaishi -> Uedama (0.51, The Citadel, via gate)
  Uedama -> Sivala (0.55, The Citadel, via gate)
  Sivala -> Hatakani (0.94, The Citadel, via gate)
  Hatakani -> Kassigainen (0.92, The Citadel, via gate)
  Kassigainen -> Synchelle (0.87, Essence, via gate)
  Synchelle -> Pakhshi (0.85, Genesis, via gate)
  Pakhshi -> Irgrus (0.67, Metropolis, via gate)
  Irgrus -> Hakeri (0.72, Metropolis, via gate)
  Hakeri -> Nifflung (0.75, Metropolis, via gate)
  Nifflung -> Josekorn (0.64, Metropolis, via gate)
  Josekorn -> Gedugaud (0.74, Metropolis, via gate)
  Gedugaud -> Tratokard (0.63, Metropolis, via gate)
  Tratokard -> Evettullur (0.79, Metropolis, via gate)
  Evettullur -> Hjortur (0.86, Metropolis, via gate)
  Hjortur -> Illuin (0.91, Metropolis, via gate)
  Illuin -> Leurtmar (0.96, Metropolis, via gate)
  Leurtmar -> Ryddinjorn (1.00, Metropolis, via gate)
  Ryddinjorn -> Meirakulf (0.88, Heimatar, via gate)
  Meirakulf -> Frarn (0.84, Heimatar, via gate)
  Frarn -> Rens (0.89, Heimatar, via gate)

From Rens to Amarr: (20 jumps)
  Rens -> Odatrik (0.82, Heimatar, via gate)
  Odatrik -> Jark (0.82, Derelik, via gate)
  Jark -> Sasta (0.81, Derelik, via gate)
  Sasta -> Lashesih (0.75, Derelik, via gate)
  Lashesih -> Lisudeh (0.76, Devoid, via gate)
  Lisudeh -> Eredan (0.70, Devoid, via gate)
  Eredan -> Mehatoor (0.66, Devoid, via gate)
  Mehatoor -> Gheth (0.60, Devoid, via gate)
  Gheth -> Sasoutikh (0.59, Devoid, via gate)
  Sasoutikh -> Ohide (0.57, Devoid, via gate)
  Ohide -> Odin (0.61, Devoid, via gate)
  Odin -> Esescama (0.61, Devoid, via gate)
  Esescama -> Uadelah (0.83, Devoid, via gate)
  Uadelah -> Hati (0.85, Devoid, via gate)
  Hati -> Yuhelia (0.80, Domain, via gate)
  Yuhelia -> Barira (0.82, Domain, via gate)
  Barira -> Bagodan (0.72, Domain, via gate)
  Bagodan -> Hama (0.84, Domain, via gate)
  Hama -> Sarum Prime (1.00, Domain, via gate)
  Sarum Prime -> Amarr (1.00, Domain, via gate)

From Amarr to Dodixie: (36 jumps)
  Amarr -> Sarum Prime (1.00, Domain, via gate)
  Sarum Prime -> Hama (0.84, Domain, via gate)
  Hama -> Bagodan (0.72, Domain, via gate)
  Bagodan -> Barira (0.82, Domain, via gate)
  Barira -> Yuhelia (0.80, Domain, via gate)
  Yuhelia -> Hati (0.85, Devoid, via gate)
  Hati -> Uadelah (0.83, Devoid, via gate)
  Uadelah -> Esescama (0.61, Devoid, via gate)
  Esescama -> Odin (0.61, Devoid, via gate)
  Odin -> Ohide (0.57, Devoid, via gate)
  Ohide -> Sasoutikh (0.59, Devoid, via gate)
  Sasoutikh -> Gheth (0.60, Devoid, via gate)
  Gheth -> Mehatoor (0.66, Devoid, via gate)
  Mehatoor -> Eredan (0.70, Devoid, via gate)
  Eredan -> Lisudeh (0.76, Devoid, via gate)
  Lisudeh -> Lashesih (0.75, Derelik, via gate)
  Lashesih -> Sasta (0.81, Derelik, via gate)
  Sasta -> Jark (0.82, Derelik, via gate)
  Jark -> Odatrik (0.82, Heimatar, via gate)
  Odatrik -> Rens (0.89, Heimatar, via gate)
  Rens -> Frarn (0.84, Heimatar, via gate)
  Frarn -> Gyng (0.80, Heimatar, via gate)
  Gyng -> Onga (0.95, Heimatar, via gate)
  Onga -> Lustrevik (0.95, Heimatar, via gate)
  Lustrevik -> Eystur (0.95, Heimatar, via gate)
  Eystur -> Hek (0.80, Metropolis, via gate)
  Hek -> Uttindar (0.50, Metropolis, via gate)
  Uttindar -> Bei (0.56, Metropolis, via gate)
  Bei -> Colelie (0.51, Sinq Laison, via gate)
  Colelie -> Deltole (0.54, Sinq Laison, via gate)
  Deltole -> Augnais (0.54, Sinq Laison, via gate)
  Augnais -> Parchanier (0.90, Sinq Laison, via gate)
  Parchanier -> Doussivitte (0.76, Sinq Laison, via gate)
  Doussivitte -> Mattere (0.95, Everyshore, via gate)
  Mattere -> Meves (0.98, Everyshore, via gate)
  Meves -> Dodixie (0.87, Sinq Laison, via gate)

From Dodixie to Jita: (15 jumps)
  Dodixie -> Botane (0.88, Sinq Laison, via gate)
  Botane -> Erme (0.80, Sinq Laison, via gate)
  Erme -> Grinacanne (0.85, Sinq Laison, via gate)
  Grinacanne -> Renyn (0.90, Essence, via gate)
  Renyn -> Algogille (0.93, Essence, via gate)
  Algogille -> Kassigainen (0.92, The Citadel, via gate)
  Kassigainen -> Hatakani (0.94, The Citadel, via gate)
  Hatakani -> Sivala (0.55, The Citadel, via gate)
  Sivala -> Uedama (0.51, The Citadel, via gate)
  Uedama -> Haatomo (0.61, The Citadel, via gate)
  Haatomo -> Suroken (0.73, The Citadel, via gate)
  Suroken -> Kusomonmon (0.85, The Citadel, via gate)
  Kusomonmon -> Urlen (0.96, The Forge, via gate)
  Urlen -> Perimeter (0.95, The Forge, via gate)
  Perimeter -> Jita (0.95, The Forge, via gate)

Shorthand route:
  Jita
  Rens
  Amarr
  Dodixie
  Jita

Total jumps: 96
```

### Jita run (with wormholes and ansiblexes)
```
$ yaerp K7D Jita K7D --wormholes ./wormholes.txt --ansiblexes ./ansiblex.txt 

Best route:

From K7D-II to Jita: (18 jumps)
  K7D-II -> V-LEKM (-0.06, Querious, via gate)
  V-LEKM -> P-ZMZV (-0.14, Querious, via gate)
  P-ZMZV -> UYU-VV (-0.19, Querious, via gate)
  UYU-VV -> 3-FKCZ (-0.28, Querious, via ansiblex)
  3-FKCZ -> Efa (0.43, Khanid, via gate)
  Efa -> Thera (-0.99, G-R00031, via wormhole)
  Thera -> Resbroko (0.44, Metropolis, via wormhole)
  Resbroko -> Hror (0.51, Metropolis, via gate)
  Hror -> Hek (0.80, Metropolis, via gate)
  Hek -> Otou (0.33, Sinq Laison, via gate)
  Otou -> Miroitem (0.31, Sinq Laison, via gate)
  Miroitem -> Rancer (0.36, Sinq Laison, via gate)
  Rancer -> Crielere (0.40, Sinq Laison, via gate)
  Crielere -> Ambeke (0.50, Sinq Laison, via gate)
  Ambeke -> Faurent (0.54, Sinq Laison, via gate)
  Faurent -> Iyen-Oursta (0.78, Sinq Laison, via gate)
  Iyen-Oursta -> Perimeter (0.95, The Forge, via gate)
  Perimeter -> Jita (0.95, The Forge, via gate)

From Jita to K7D-II: (18 jumps)
  Jita -> Perimeter (0.95, The Forge, via gate)
  Perimeter -> Iyen-Oursta (0.78, Sinq Laison, via gate)
  Iyen-Oursta -> Faurent (0.54, Sinq Laison, via gate)
  Faurent -> Ambeke (0.50, Sinq Laison, via gate)
  Ambeke -> Crielere (0.40, Sinq Laison, via gate)
  Crielere -> Rancer (0.36, Sinq Laison, via gate)
  Rancer -> Miroitem (0.31, Sinq Laison, via gate)
  Miroitem -> Otou (0.33, Sinq Laison, via gate)
  Otou -> Hek (0.80, Metropolis, via gate)
  Hek -> Hror (0.51, Metropolis, via gate)
  Hror -> Resbroko (0.44, Metropolis, via gate)
  Resbroko -> Thera (-0.99, G-R00031, via wormhole)
  Thera -> Efa (0.43, Khanid, via wormhole)
  Efa -> 3-FKCZ (-0.28, Querious, via gate)
  3-FKCZ -> UYU-VV (-0.19, Querious, via ansiblex)
  UYU-VV -> P-ZMZV (-0.14, Querious, via gate)
  P-ZMZV -> V-LEKM (-0.06, Querious, via gate)
  V-LEKM -> K7D-II (-0.07, Querious, via gate)

Shorthand route:
  K7D-II
  Jita
  K7D-II

Total jumps: 36
```

### Safe-ish Jita run (with wormholes and ansiblexes, but no J-Space or Thera/Turnur/Zarzakh/Pochven holes)
```
$ yaerp K7D Jita K7D --wormholes ./wormholes.txt --ansiblexes ./ansiblex.txt --no-jspace --no-special

Best route:

From K7D-II to Jita: (28 jumps)
  K7D-II -> A2-V27 (-0.39, Querious, via ansiblex)
  A2-V27 -> Kaira (0.33, Khanid, via gate)
  Kaira -> Ashmarir (0.42, Khanid, via gate)
  Ashmarir -> Arzanni (0.55, Khanid, via gate)
  Arzanni -> Keberz (0.53, Khanid, via gate)
  Keberz -> Lansez (0.73, Khanid, via gate)
  Lansez -> Bukah (0.80, Khanid, via gate)
  Bukah -> Agil (0.85, Khanid, via gate)
  Agil -> Hishai (0.77, Khanid, via gate)
  Hishai -> Geztic (0.73, Khanid, via gate)
  Geztic -> Osis (0.73, Khanid, via gate)
  Osis -> Yezara (0.74, Khanid, via gate)
  Yezara -> Ervekam (0.69, Khanid, via gate)
  Ervekam -> Masanuh (0.70, Kor-Azor, via gate)
  Masanuh -> Leva (0.62, Kor-Azor, via gate)
  Leva -> Kor-Azor Prime (0.91, Kor-Azor, via gate)
  Kor-Azor Prime -> Amarr (1.00, Domain, via gate)
  Amarr -> Bhizheba (0.96, Domain, via gate)
  Bhizheba -> Romi (0.72, Kador, via gate)
  Romi -> Aphend (0.63, Kador, via gate)
  Aphend -> Dresi (0.62, Kador, via gate)
  Dresi -> Gensela (0.69, Kador, via gate)
  Gensela -> Shera (0.60, Genesis, via gate)
  Shera -> Ahbazon (0.42, Genesis, via gate)
  Ahbazon -> Hykkota (0.82, The Forge, via gate)
  Hykkota -> Ansila (0.91, The Forge, via gate)
  Ansila -> Ikuchi (0.99, The Forge, via gate)
  Ikuchi -> Jita (0.95, The Forge, via gate)

From Jita to K7D-II: (28 jumps)
  Jita -> Ikuchi (0.99, The Forge, via gate)
  Ikuchi -> Ansila (0.91, The Forge, via gate)
  Ansila -> Hykkota (0.82, The Forge, via gate)
  Hykkota -> Ahbazon (0.42, Genesis, via gate)
  Ahbazon -> Shera (0.60, Genesis, via gate)
  Shera -> Gensela (0.69, Kador, via gate)
  Gensela -> Zororzih (0.74, Kador, via gate)
  Zororzih -> Akhragan (0.80, Domain, via gate)
  Akhragan -> Kehour (0.92, Domain, via gate)
  Kehour -> Ashab (0.91, Domain, via gate)
  Ashab -> Amarr (1.00, Domain, via gate)
  Amarr -> Kor-Azor Prime (0.91, Kor-Azor, via gate)
  Kor-Azor Prime -> Leva (0.62, Kor-Azor, via gate)
  Leva -> Masanuh (0.70, Kor-Azor, via gate)
  Masanuh -> Ervekam (0.69, Khanid, via gate)
  Ervekam -> Yezara (0.74, Khanid, via gate)
  Yezara -> Osis (0.73, Khanid, via gate)
  Osis -> Geztic (0.73, Khanid, via gate)
  Geztic -> Hishai (0.77, Khanid, via gate)
  Hishai -> Agil (0.85, Khanid, via gate)
  Agil -> Bukah (0.80, Khanid, via gate)
  Bukah -> Lansez (0.73, Khanid, via gate)
  Lansez -> Keberz (0.53, Khanid, via gate)
  Keberz -> Arzanni (0.55, Khanid, via gate)
  Arzanni -> Ashmarir (0.42, Khanid, via gate)
  Ashmarir -> Kaira (0.33, Khanid, via gate)
  Kaira -> A2-V27 (-0.39, Querious, via gate)
  A2-V27 -> K7D-II (-0.07, Querious, via ansiblex)

Shorthand route:
  K7D-II
  Jita
  K7D-II

Total jumps: 56
```

## Installation

### From binary

Just grab an exectuable from the releases and run it, everything is self contained.

### From source

MSRV: 1.70.0

You will need a copy of the SDE (zipped) to build from source because there is a build script that reads the systems from it. The build script uses rayon, so your system will be bogged down until the build finishes.

1. git clone git@github.com:RecursivePineapple/yaerp.git
2. cd yaerp
3. EVE_SDE_ZIP_PATH=.../sde-TRANQUILITY.zip cargo build --release
4. Wait several seconds (shouldn't take more than a minute)
5. Do something with ./target/release/yaerp

You can also specify `EVE_SDE_ZIP_PATH` in the file `.env.build` in the project root, if you don't want to specify it each time. The build script loads this file into the environment automatically.

## Configuration

### Wormholes

1. Connect to the Eve Scout wormhole bookmark folder
2. Select all locations in it
3. Copy + paste them into a text file
4. Run yaerp with the `--wormhole` option, followed by the file

Each line must match one of two formats:
- ABC-123[tab]Jita -&gt; Thera[tab]Coordinate[tab][Anything][tab]Jita [rest is ignored]
Or:
- ABC-123[tab]K7D-II (NS)[tab]Coordinate[tab][Anything][tab]Jita [rest is ignored]

Specifically, it needs to match this regex: `^[A-Z]{3}-\d{3} +[\w\-]+ +(-&gt; +[\w\-]+|\(\w+\))\t([^\t]+\t){2}[^\t]+`

Each line adds both directions (so a line for Jita -> K7D-II will allow the planner to go both Jita -> K7D-II and K7D-II -> Jita).
Duplicates are silently ignored.

### Ansiblexes

1. Find an SMT-compatible ansiblex file and save it (or make it yourself)
2. Run yaerp with the `--ansiblexes` option, followed by the file

Each line must match one of these formats:
- An empty line
- A comment line (starts with a # symbol)
- [Ignored Number] [Start System] --> [End System]

Specifically, it needs to match this regex: `^(#.*|\d+\s+[\w\-]+\s+-->\s+[\w\-]+)?$`

As with wormholes, there does not need to be a line for each direction.
