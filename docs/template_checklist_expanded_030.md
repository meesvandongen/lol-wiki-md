# Template checklist — expanded

Generated: 2025-10-10T15:54:19.271Z

Batch 30 of 33 — items 581..600

## Wiki

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AWiki%2Fdoc.txt
- Decoded name: Template:Wiki/doc
- Namespace: Template
- Remainder: Wiki/doc
- Path parts: Template:Wiki / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: This is a shorthand template for linking to articles on another wiki soportado por [https://weirdgloop.org/ Weird Gloop] beyond of the League of Legends Wiki. Full URLs affect web tracking in a way that wiki doesn't want and so you're discouraged from using full URLs to link between wikis.

;Syntax
 You may use either {{t|wiki}} to access this template.
To link to a wiki from Weird Gloop, simply use:
 {{t|wiki|1|2|3|4|5|lang|wiki}}
*<code>1</code> - the wiki name, or wiki if it starts with that domain name
*<code>2</code> - the wiki name if the first one starts with <code>wiki</code>
*<code>3</code> - the domain if they exist on the site
*<code>4</code> - the name of the page
*<code>5</code> - the alternative of the name to be displayed in the text
*<code>lang</code> - a validator if the wiki has different languages in the wiki domain
*<code>w</code> - a validator if the prefix of <code>wiki</code> has a different name

;Valid Site Web
{| class="wikitable article-table hover-row hover-column" style="position:relative; z-index:0; margin:auto;"
!Website domain
```

### Notes

_No notes specified._

---

## Wikipedia

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AWikipedia%2Fdoc.txt
- Decoded name: Template:Wikipedia/doc
- Namespace: Template
- Remainder: Wikipedia/doc
- Path parts: Template:Wikipedia / doc

```
{{Documentation subpage}}
;Description
:Use this on pages which directly use [[wikipedia:|Wikipedia]] content.
;Syntax
:Type <code>{{t|wikipedia|page name}}</code> as the last item of the page text.
:Note that "<code>page name</code>" should be the title of the page ''on Wikipedia''.

<includeonly>[[Category:General wiki templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Wikipedia-deleted

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AWikipedia-deleted%2Fdoc.txt
- Decoded name: Template:Wikipedia-deleted/doc
- Namespace: Template
- Remainder: Wikipedia-deleted/doc
- Path parts: Template:Wikipedia-deleted / doc

```
{{Documentation subpage}}
;Description
:Use this on pages which directly use [[wikipedia:|Wikipedia]] content (that has since been deleted from Wikipedia).
;Syntax
:Type <code>{{t|wikipedia-deleted|usernames}}</code> as the last item of the page text.
:Note that "<code>usernames</code>" should be the known user names of the main people who edited the article ''on Wikipedia''.

<includeonly>[[Category:General wiki templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Wildcard

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AWildcard%2Fdoc.txt
- Decoded name: Template:Wildcard/doc
- Namespace: Template
- Remainder: Wildcard/doc
- Path parts: Template:Wildcard / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

;Description
:This template will produce an icon for [[LoR:Wildcard|Wildcard]] in {{tip|Legends of Runeterra}} and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|Wildcard|cost|3=type=Champion, Epic, Rare or Common}}</code> at any part of the article.

;Example
:<code>{{tl|Wildcard|1|type=Champion}}</code>
:* {{Wildcard|1|type=Champion}}

:<code>{{tl|Wildcard|1|type=Champion|4=size=30}}</code>
:* {{Wildcard|1|type=Champion|size=30}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Icon templates]]

```

### Notes

_No notes specified._

---

## WP

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AWP%2Fdoc.txt
- Decoded name: Template:WP/doc
- Namespace: Template
- Remainder: WP/doc
- Path parts: Template:WP / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
:This template will produce an icon for Weekly Points in Wild Rift and the link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|WP|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|WP}}</code>
:* {{WP}}

:<code>{{tl|WP|100}}</code>
:* {{WP|100}}

:<code>{{tl|WP|100|3=size=30}}</code>
:* {{WP|100|size=30}}

<includeonly>
<!-- Categories and interwikis go here: -->
```

### Notes

_No notes specified._

---

## WRa

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AWRa%2Fdoc.txt
- Decoded name: Template:WRa/doc
- Namespace: Template
- Remainder: WRa/doc
- Path parts: Template:WRa / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: A duplicate of [[Template:Ability icon]] for [[Wild Rift]].

;See also
* [[Template:Ability]]

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Icon templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## WRca

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AWRca%2Fdoc.txt
- Decoded name: Template:WRca/doc
- Namespace: Template
- Remainder: WRca/doc
- Path parts: Template:WRca / doc

```
{{Documentation subpage}}
;Description
:This template makes a link to an ability and its respective champion out of their images and names.
:When using code (see syntax below), 1st parameter is the ability name, 2nd parameter is for the champion name.
:When using an inserted template, Parameter #1 is for the champion's name, Parameter #2 is for the ability name (flipped version of code syntax). Parameter #3 can be left blank.
;Syntax
:Type <code>{{tl|WRca|''Ability''|''Champion''}}</code> at any part of the article.
;Examples
*<code>{{tl|WRca|Noxious Trap|Teemo}}</code>
** {{WRca|Noxious Trap|Teemo}}
*<code>{{tl|WRca|Ranger's Focus|Ashe}}</code>
** {{WRca|Ranger's Focus|Ashe}}

<includeonly>
[[Category:Icon templates]]

</includeonly>
```

### Notes

_No notes specified._

---

## WRis

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AWRis%2Fdoc.txt
- Decoded name: Template:WRis/doc
- Namespace: Template
- Remainder: WRis/doc
- Path parts: Template:WRis / doc

```
{{Documentation subpage}}
:This template shortcuts usage of {{t|WR item}} when using possessive apostrophes.

;Syntax
:Type <code>{{t|WRis|Item}}</code> instead of <code>{{t|WRi|Item|Item's}}</code> at any part of the article.

:Correct:
:<code>{{tl|WRis|Sheen}}</code>
:{{WRis|Sheen}}
:<code>{{tl|WRis|Rabadon's Deathcap}}</code>
:{{WRis|Rabadon's Deathcap}}

:Wrong:
:<code>{{tl|WRi|Sheen}}'s</code>
:{{WRi|Sheen}}'s
:<code>{{tl|WRi|Rabadon's Deathcap}}'</code>
:{{WRi|Rabadon's Deathcap}}'s

<includeonly>[[Category:Icon templates|{{PAGENAME}}]]</includeonly>
<noinclude></noinclude>
```

### Notes

_No notes specified._

---

## WRtip

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AWRtip%2Fdoc.txt
- Decoded name: Template:WRtip/doc
- Namespace: Template
- Remainder: WRtip/doc
- Path parts: Template:WRtip / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
* [[Template:Tip]] in the context of [[Wild Rift]]. It automatically prefixes '''Wr''' to the keyword's name based on the link scheme for [[Template:Tip data|tip data templates]] in Wild Rift, so that it does not need to be typed. If no template exists for WR, the LoL version is used instead.

<code><nowiki>{{tip|WR Slow|Slow}}</nowiki></code> and <code><nowiki>{{WRtip|Slow}}</nowiki></code>
both give:

{{WRtip|Slow}}

<code><nowiki>{{tip|WR Slow|handicapped}}</nowiki></code> and <code><nowiki>{{WRtip|Slow|handicapped}}</nowiki></code>
both give:

{{tip|WR Slow|handicapped}}

Currently supports the following parameters of Template:Tip:
* ''style''
* ''icononly''
* ''noimg''
* ''size''
```

### Notes

_No notes specified._

---

## WRui

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AWRui%2Fdoc.txt
- Decoded name: Template:WRui/doc
- Namespace: Template
- Remainder: WRui/doc
- Path parts: Template:WRui / doc

```
{{Documentation subpage}}
== Usage ==
Icon template used for Wild Rift units that are not champions.

== Syntax ==
<code><nowiki>{{WRui|unitname|displayname|link=pagelink|file=File:filename.png}}</nowiki></code>

== See also ==
* [[Template:WR champion icon]]

<includeonly>
[[Category:Icon templates]]

[[de:Vorlage:Unit icon]]
</includeonly>
```

### Notes

_No notes specified._

---

## XP

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AXP%2Fdoc.txt
- Decoded name: Template:XP/doc
- Namespace: Template
- Remainder: XP/doc
- Path parts: Template:XP / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
:This template will produce an icon for [[Experience (summoner)|account experience]] and link to its page. You can also adjust the size of the icon and add its cost.

;Syntax
:Type <code>{{t|XP|cost|3=size=icon size}}</code> at any part of the article.

;Example
:<code>{{tl|XP}}</code>
:* {{XP}}

:<code>{{tl|XP|100}}</code>
:* {{XP|100}}

:<code>{{tl|XP|100|3=size=30}}</code>
:* {{XP|100|size=30}}

<includeonly>
<!-- Categories and interwikis go here: -->
```

### Notes

_No notes specified._

---

## Annual cycle navigation

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AAnnual%20cycle%20navigation%2Fdoc.txt
- Decoded name: Template:Annual cycle navigation/doc
- Namespace: Template
- Remainder: Annual cycle navigation/doc
- Path parts: Template:Annual cycle navigation / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Navigation between annual cycle articles. To be used at the top of the page.

;Syntax
: {{t|Annual cycle navigation|prev|next|float|style}}
* ''All parameters are optional.''
* <code>prev</code> = Preceding annual cycle. Defaults to "??".
* <code>next</code> = Succeeding annual cycle. Defaults to "??".
* <code>float</code> = Float behavior of the entire template section. Defaults to <code>right</code>.
* <code>style</code> = Additional style parameters for the entire template section.

;Example
: <code><nowiki>{{Annual cycle navigation|prev=Season 2022|next=Season 2024}}</nowiki></code>
{{Annual cycle navigation|prev=Season 2022|next=Season 2024}}

;See also
* [[Template:Release history]]
* [[Template:Thematic Season infobox]]
```

### Notes

_No notes specified._

---

## Article game navigation

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AArticle%20game%20navigation%2Fdoc.txt
- Decoded name: Template:Article game navigation/doc
- Namespace: Template
- Remainder: Article game navigation/doc
- Path parts: Template:Article game navigation / doc

```
#REDIRECT [[Template:Game navigation/doc]]
```

### Notes

_No notes specified._

---

## Banner navigation

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ABanner%20navigation%2Fdoc.txt
- Decoded name: Template:Banner navigation/doc
- Namespace: Template
- Remainder: Banner navigation/doc
- Path parts: Template:Banner navigation / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Displays and links two related pages in a banner format.

;Syntax
: <code>{{t|Banner navigation|text1|link1|text2|link2}}</code>

Obligatory parameters:
*'''text1''' or '''1''': Sets the display text for the first article.
*'''link1''' or '''2''': Sets the linked page for the first article.
*'''text2''' or '''3''': Sets the display text for the second article.
*'''link2''' or '''4''': Sets the linked page for the second article.

Optional parameters:
*'''icon1''': Sets an image directly before the first article's display text.
*'''size1''': Sets the size of ''icon1''. (default: 18px)
*'''icon2''': Sets an image directly before the second article's display text.
*'''size2''': Sets the size of ''icon2''. (default: 18px)

```

### Notes

_No notes specified._

---

## Champion categories

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AChampion%20categories%2Fdoc.txt
- Decoded name: Template:Champion categories/doc
- Namespace: Template
- Remainder: Champion categories/doc
- Path parts: Template:Champion categories / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
This template automatically generates categories for League of Legends champion articles.

;Usage
{{t|Champion categories|name}}

;Automated categories
{| class="article-table"
! Category
! Error handling
|-
| Category:<code>Name</code> || —
|-
| Category:Champion gameplay article || ''Won't draw if the subpage isn't a root page in the Main space''
|-
| Category:<code>Resource</code> champion || 
* Mana champion
* Energy champion AND Manaless champion
* Fury champions AND Manaless champion
```

### Notes

_No notes specified._

---

## Champion info

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AChampion%20info%2Fdoc.txt
- Decoded name: Template:Champion info/doc
- Namespace: Template
- Remainder: Champion info/doc
- Path parts: Template:Champion info / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Main template to present a League of Legends champion's info and stats at the top of the article. Combines {{tl|Infobox champion}} and {{tl|Infobox stats}} and includes a lead paragraph with a reference to their main site entry.

;Syntax
: {{t|Champion info|Champion name}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Section formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Champions

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AChampions%2Fdoc.txt
- Decoded name: Template:Champions/doc
- Namespace: Template
- Remainder: Champions/doc
- Path parts: Template:Champions / doc

```
{{Documentation subpage}}
{{Champions}}


{{Champions|wr}}

;Description
:This navbox is used for Champion Articles, to link them to other Champions.
;Syntax
:Type <code>{{t|champions}}</code> at the end of the article. For WR, write <code>wr</code> in the first parameter.

<includeonly>
[[Category:Navigation templates]]

[[de:Vorlage:Navigation Champion]]
</includeonly>
```

### Notes

_No notes specified._

---

## Class infobox

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AClass%20infobox%2Fdoc.txt
- Decoded name: Template:Class infobox/doc
- Namespace: Template
- Remainder: Class infobox/doc
- Path parts: Template:Class infobox / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Presents the general attributes of a [[Champion classes|champion class]] in League of Legends articles that describe each class ([[Special:PrefixIndex/Champion_classes]]).
: Wheel is created via [[Template:Champion attributes]], which only accepts valid class names. The graphs are custom-made images.

;Syntax
<pre>
{{Class infobox
|title= Override the name provided by the page title
|(GalleryHelper parameters)=
|description= (A description)
|strengths= (Examples of strengths)
|weaknesses= (Examples of weaknesses)
|average= Class name
}}
</pre>

;GalleryHelper information
: ''This template uses [[Template:GalleryHelper|GalleryHelper]].''
```

### Notes

_No notes specified._

---

## Defense tree navigation

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3ADefense%20tree%20navigation%2Fdoc.txt
- Decoded name: Template:Defense tree navigation/doc
- Namespace: Template
- Remainder: Defense tree navigation/doc
- Path parts: Template:Defense tree navigation / doc

```
{{Documentation subpage}}

==Usage==
Used on mastery tree pages to display a navigation box between the different version of the tree throughout the seasons.

==Display==
{{Defense tree navigation}}

==See also==
* [[Template:Offense tree navigation|Offense tree navigation]]
* [[Template:Utility tree navigation|Utility tree navigation]]

<includeonly>[[Category:Mastery data templates]]</includeonly>
```

### Notes

_No notes specified._

---

## Event infobox

- [x] Status: done
- [ ] Include decision: exclude

### Source (first 20 lines)

- Source filename: Template%3AEvent%20infobox%2Fdoc.txt
- Decoded name: Template:Event infobox/doc
- Namespace: Template
- Remainder: Event infobox/doc
- Path parts: Template:Event infobox / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Infobox for [[:Category:Events|Events]] in any game.
: Can automatically add the article into certain event-related categories based on the categorization parameters provided.

<!-- Should "series" be renamed to "universe"? -->
;Syntax
: Some parameters depend on the presence of other parameters. Each set of parameters is entirely optional. If all three sets of parameters are left blank, the entire chronology section will be ommited. All categories can be disabled by adding "categories = false".
* <code>calendar-event</code> relates to annual celebrations and is required for the <code>prev-calendar</code>/<code>next-calendar</code> timeline.
* <code>series</code> relates to the same event and is required for the <code>prev-time</code>/<code>next-time</code> timeline.
* <code>year</code> relates to all events that have been released in chronological order and is required for the <code>prev-event</code>/<code>next-event</code> timeline.
: Categorization parameters. If these parameters are set to "true", they will add the article to the following categories:
# ''calendar-event'' adds it to the "<code>(Year) events</code>" category
# ''{{tip|League of Legends|LOL}} / {{tip|Teamfight Tactics|TFT}} / {{tip|Legends of Runeterra|LOR}} / {{tip|Wild Rift|WR}}''
#* [[:Category:League of Legends events]] or [[:Category:Teamfight Tactics events]] or [[:Category:Legends of Runeterra events]] or [[:Category:Wild Rift events]] (respectively)
# ''missions'': If the event has missions; adds it to [[:Category:Event with missions]]
# and ''currency'': If the event awards currency; adds it to [[:Category:Event with currency]]
<pre>
{{Event infobox
```

### Notes

_No notes specified._

---
