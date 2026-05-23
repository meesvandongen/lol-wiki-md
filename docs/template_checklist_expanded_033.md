# Template checklist — expanded

Generated: 2025-10-10T15:54:19.285Z

Batch 33 of 33 — items 641..652

## Portal link

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3APortal%20link%2Fdoc.txt
- Decoded name: Template:Portal link/doc
- Namespace: Template
- Remainder: Portal link/doc
- Path parts: Template:Portal link / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Adds an icon and a link to a game- or title-specific landing page in the {{SITENAME}}.

;Syntax
: {{t|portal link|1:portal|alttext}}
: '''1''': name of game/title/section on the Wiki. Is case-insensitive.

;Supported
* <code><nowiki>{{portal link|league of legends or lol}}</nowiki></code> -> {{portal link|League of Legends}}
* <code><nowiki>{{portal link|teamfight tactics or tft}}</nowiki></code> -> {{portal link|Teamfight Tactics}}
* <code><nowiki>{{portal link|legends of runeterra or lor}}</nowiki></code> -> {{portal link|Legends of Runeterra}}
* <code><nowiki>{{portal link|2xko}}</nowiki></code> -> {{portal link|2XKO}}
* <code><nowiki>{{portal link|wild rift or wr}}</nowiki></code> -> {{portal link|Wild Rift}}
* <code><nowiki>{{portal link|universe or runeterra or lore}}</nowiki></code> -> {{portal link|Universe}}
* <code><nowiki>{{portal link|universe of arcane or arcane lore}}</nowiki></code> -> {{portal link|Arcane}}
* <code><nowiki>{{portal link|editor or community}}</nowiki></code> -> {{portal link|Community}}
* <code><nowiki>{{portal link|todo}}</nowiki></code> -> {{portal link|todo}}
* <code><nowiki>{{portal link|riot or riot games}}</nowiki></code> -> {{portal link|riot games}}
```

### Notes

_No notes specified._

---

## Portal navigation

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3APortal%20navigation%2Fdoc.txt
- Decoded name: Template:Portal navigation/doc
- Namespace: Template
- Remainder: Portal navigation/doc
- Path parts: Template:Portal navigation / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
{{Removed}}
;Description
This template is used to navigate between the League of Legends Wiki's multiple homepages. 

;Syntax
{{t|Portal navigation|id}}

The 'id' handles game-specific modifications to the template (e.g. emphasizing the page you're currently on). 
* LOL
* TFT
* LOR
* WR
* Editor
* Universe

;Example
{{t|Portal navigation|Editor}}
{{Portal navigation|Editor}}
```

### Notes

_No notes specified._

---

## Portal subheader

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3APortal%20subheader%2Fdoc.txt
- Decoded name: Template:Portal subheader/doc
- Namespace: Template
- Remainder: Portal subheader/doc
- Path parts: Template:Portal subheader / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Adds a formatted, small header with a background color.

;Syntax
: {{t|Portal subheader|1}}
: '''1''': subheader title

;Usage
: Add '''Portal subheader''' within a Portal template to make a smaller header than the primary/title one, and create a subsection.

;Example
{{Portal|header=Main title|content=Some text
Some more text
Some icons
{{Portal subheader|Small title}}
Further text and icons
}}

```

### Notes

_No notes specified._

---

## Rework infobox

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ARework%20infobox%2Fdoc.txt
- Decoded name: Template:Rework infobox/doc
- Namespace: Template
- Remainder: Rework infobox/doc
- Path parts: Template:Rework infobox / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
: Infobox that accompanies champion rework information.

;Syntax:
: Attack, Innate, Q, W, E and R use {{t|advanced tooltip}} to generate a pop-up.
: The icon and the description must be separated by <code>//</code>. 
: Icon does not need to be specificed for the champion's Attack as the icon is consistent.
<pre>
{{Rework infobox
| champion    = 
| (GalleryHelper parameters)= 
| version     = 
| description = 
| attack      = 
| innate      = //
| q           = //
| w           = //
```

### Notes

_No notes specified._

---

## Rune path infobox

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ARune%20path%20infobox%2Fdoc.txt
- Decoded name: Template:Rune path infobox/doc
- Namespace: Template
- Remainder: Rune path infobox/doc
- Path parts: Template:Rune path infobox / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Usage
This infobox is used to design the following templates:
* [[Template:Rune path infobox/Domination]]
* [[Template:Rune path infobox/Inspiration]]
* [[Template:Rune path infobox/Precision]]
* [[Template:Rune path infobox/Resolve]]
* [[Template:Rune path infobox/Sorcery]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Infobox templates]]
[[de:Vorlage:Infobox Runenpfad]]
[[ru:Шаблон:Rune path infobox]]
</includeonly>
```

### Notes

_No notes specified._

---

## Section bot

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ASection%20bot%2Fdoc.txt
- Decoded name: Template:Section bot/doc
- Namespace: Template
- Remainder: Section bot/doc
- Path parts: Template:Section bot / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
This template goes at the bottom of articles intended for use in conjunction with Tab-View; namely champion subpages. 

'''Do not use this template as a generic page-bottom.'''

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Section formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Section top

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ASection%20top%2Fdoc.txt
- Decoded name: Template:Section top/doc
- Namespace: Template
- Remainder: Section top/doc
- Path parts: Template:Section top / doc

```
{{Documentation subpage}}

; Description
: This template is used to categorize pages sharing the same base page name together.
: Champion subpages also link to the equivalent [[:de:|German]] and [[:ru:|Russian]] wiki tab.

;Syntax
: {{t|section top}}

;See also

<includeonly>
[[Category:General wiki templates]]

</includeonly>
```

### Notes

_No notes specified._

---

## Skin infobox

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ASkin%20infobox%2Fdoc.txt
- Decoded name: Template:Skin infobox/doc
- Namespace: Template
- Remainder: Skin infobox/doc
- Path parts: Template:Skin infobox / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Infobox that presents features and credits of a champion skin in a standardized format.

;Syntax & Example
: Date format should be: DD-Month abbreviation-YYYY
:: e.g. 14-Mar-2015
{{Skin infobox
  | 1            = Arcanist Kog'Maw
  | legacy       =
  | image        = {{GalleryHelper
      |Kog'Maw ArcanistSkin.jpg¦Splash Art¦
      |Arcanists The Unlikely Companions Concept 04.jpg¦Concept¦Arcanists "The Unlikely Companions" Concept 4 (by Riot Contracted Artist Justice Wong)
      }}
  | model_link   = [https://modelviewer.lol/model-viewer?id=96019 View in 3D]
  | champion     = {{ci|Kog'Maw}}
  | quality      = {{skin tier|Epic}}
  | cost         = {{RP|1350}}
  | release_date = 09-Jul-2020
```

### Notes

_No notes specified._

---

## Thematic Season infobox

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AThematic%20Season%20infobox%2Fdoc.txt
- Decoded name: Template:Thematic Season infobox/doc
- Namespace: Template
- Remainder: Thematic Season infobox/doc
- Path parts: Template:Thematic Season infobox / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Infobox for a {{tip|League of Legends}} thematic [[season]].

;GalleryHelper information
: ''This template uses [[Template:GalleryHelper|GalleryHelper]].''
* Only uses 1 instance: for title art.
** Image parameter(s): <code>image</code> '''or''' <code>gallery</code>
** Additional caption parameter: (none)
** Placeholder image: (none)

== Example syntax ==
{{Thematic Season infobox
  | cycle= [[2025 Annual Cycle]]
  | release= Act 1: January 9th, 2025<hr style="border-color:var(--league-grey-2);">Act 2: March 5th, 2025
  | end= April 28th, 2025
  | patch= Act 1:<br>[[V25.S1.1|S1.1]]{{,}}[[V25.S1.2|S1.2]]{{,}}[[V25.S1.3|S1.3]]{{,}}[[V25.04|25.04]]<hr style="border-color:var(--league-grey-2);">Act 2:<br>[[V25.05|25.05]]{{,}}[[V25.06|25.06]]{{,}}[[V25.07|25.07]]{{,}}[[V25.08|25.08]]
  | image= 2025 Season 1 Promo 01.png
<!--  | description= <center>Noxus-themed season</center> -->
```

### Notes

_No notes specified._

---

## Universe infobox

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AUniverse%20infobox%2Fdoc.txt
- Decoded name: Template:Universe infobox/doc
- Namespace: Template
- Remainder: Universe infobox/doc
- Path parts: Template:Universe infobox / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Infobox for a [[Champion skin|skin]] set/alternate universe.

;Syntax
<pre>
{{Universe infobox
  | name=Example
  | (GalleryHelper parameters)=
  | description=Example
  | lore=Example
  | release=Example
}}
</pre>

;GalleryHelper information
: ''This template uses [[Template:GalleryHelper|GalleryHelper]].''
* Only uses 1 instance: for theme, concept & flavor art.
** Image parameter(s): <code>image</code> '''or''' <code>gallery</code>
```

### Notes

_No notes specified._

---

## Universe navigation

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AUniverse%20navigation%2Fdoc.txt
- Decoded name: Template:Universe navigation/doc
- Namespace: Template
- Remainder: Universe navigation/doc
- Path parts: Template:Universe navigation / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Displays a list of subpages for a lore characters (Universe, Arcane).

;Syntax
: <code>{{t|Universe navigation|1|2}}</code>
: 1 (Optional): Override first link (base Universe page)
: 2 (Optional): Override second link (Arcane page)

;Usage
: Type <nowiki>{{Universe navigation}}</nowiki> at the top of the page.

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Navigation templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Utility tree navigation

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AUtility%20tree%20navigation%2Fdoc.txt
- Decoded name: Template:Utility tree navigation/doc
- Namespace: Template
- Remainder: Utility tree navigation/doc
- Path parts: Template:Utility tree navigation / doc

```
{{Documentation subpage}}

==Usage==
Used on mastery tree pages to display a navigation box between the different version of the tree throughout the seasons.

==Display==
{{Utility tree navigation}}

==See also==
* [[Template:Defense tree navigation|Defense tree navigation]]
* [[Template:Offense tree navigation|Offense tree navigation]]

<includeonly>[[Category:Mastery data templates]]</includeonly>
```

### Notes

_No notes specified._

---
