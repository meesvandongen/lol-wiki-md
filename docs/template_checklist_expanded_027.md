# Template checklist — expanded

Generated: 2025-10-10T15:54:19.255Z

Batch 27 of 33 — items 521..540

## T

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AT%2Fdoc.txt
- Decoded name: Template:T/doc
- Namespace: Template
- Remainder: T/doc
- Path parts: Template:T / doc

```
{{Documentation subpage}}
;Description
:A template link with a variable number of example parameters (0-20), which can be used to show example inputs.
:Utilises [[Template:T/piece]].
;Syntax
:{{t|t|parameter1|parameter2|parameter3|parameter4|...|parameter20}}
;Sample code
:<code><nowiki>{{t|t}}</nowiki></code> gives...
:{{t|t}}
:<code><nowiki>{{t|t|Item1|Item2|Item3|Item4|Item5|...}}</nowiki></code> gives...
:{{t|t|Item1|Item2|Item3|Item4|Item5|...}}
;See also
:[[w:c:wow:Template:T|Template:T]] on WoWWiki

<includeonly>[[Category:Documentation templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---

## T/piece

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AT%2Fpiece%2Fdoc.txt
- Decoded name: Template:T/piece/doc
- Namespace: Template
- Remainder: T/piece/doc
- Path parts: Template:T / piece / doc

```
{{Documentation subpage}}
;Description
:Parameter piece for {{t|t}}, controls styling/showing of parameter fragments.
;See also
:[[w:c:wow:Template:T/piece|Template:T/piece]] on WoWWiki

<includeonly>[[Category:Documentation templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Tabview

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATabview%2Fdoc.txt
- Decoded name: Template:Tabview/doc
- Namespace: Template
- Remainder: Tabview/doc
- Path parts: Template:Tabview / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
: Creates a tabber where each tab transcludes one page under it.
: More convenient and readable replacement for tabber syntax. It is preferred in [[Universe:Universe|Universe]] skin lore pages the most due to their page and category structure.

;Syntax
: {{t|tabview|content1|tab1|content2|tab2|…}}
* Where "content(x)" is the [[League_of_Legends_Wiki:Tutorial#Wiki_(or_'Internal')_links|internal link]] to the page that will be transcluded and "tab(x)" is the title of its respective tab.

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Formatting templates]]
</includeonly>
```

### Notes

_No notes specified._

---

## Template link

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATemplate%20link%2Fdoc.txt
- Decoded name: Template:Template link/doc
- Namespace: Template
- Remainder: Template link/doc
- Path parts: Template:Template link / doc

```
{{Documentation subpage}}
The '''template link''' template is a simple [[w:Macro (computer science)|macro]] [[m:Help:template|template]] used to display a template name as a link surrounded by braces, thus showing how the template name would be used in code. Its primary use is in instruction and documentation.

==Example usage==
<pre>{{Tl|t}}</pre>
{{Tl|t}}

<pre>{{Tl|t|foo|bar|baz}}</pre>
{{Tl|t|foo|bar|baz}}

If more than ten parameters are necessary, manually append more to the last parameter using <code>&amp;#124;</code>. Otherwise, any parameters beyond the tenth will be replaced with "...".

<pre>{{Tl|Tl|t|1|2|3|4|5|6|7|8|9&#124;10&#124;11&#124;12}}</pre>
{{Tl|t|1|2|3|4|5|6|7|8|9|10|11|12}}

<pre>{{Tl|Tl|t|1|2|3|4|5|6|7|8|9&#124;10&amp;#124;11&amp;#124;12}}</pre>

{{Tl|t|1|2|3|4|5|6|7|8|9|10&#124;11&#124;12}}

<includeonly>
```

### Notes

_No notes specified._

---

## Template other

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATemplate%20other%2Fdoc.txt
- Decoded name: Template:Template other/doc
- Namespace: Template
- Remainder: Template other/doc
- Path parts: Template:Template other / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

This is the {{tl|template other}} meta-template.

It helps other templates detect if they are on a "Template:" page or some "other" type of page.

=== Usage ===

This template usually takes two parameters, like this:

<pre>
{{template other | Template page text | Other pages text }}
</pre>

If the template is on a "Template:" page, it will return this:

:{{template other | Template page text | Other pages text }}

If the template is on any other page, it will return this:
```

### Notes

_No notes specified._

---

## Templatecategory

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATemplatecategory%2Fdoc.txt
- Decoded name: Template:Templatecategory/doc
- Namespace: Template
- Remainder: Templatecategory/doc
- Path parts: Template:Templatecategory / doc

```
{{Documentation subpage}}
;Description
:This template is used in subcategories of [[:Category:Templates]].
;Syntax
:To use the template, type a short description of the template category followed by <code>{{t|templatecategory}}</code>.

<includeonly>[[Category:Category templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---

## Term

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATerm%2Fdoc.txt
- Decoded name: Template:Term/doc
- Namespace: Template
- Remainder: Term/doc
- Path parts: Template:Term / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->__NOTOC__
==Description==
This template defines multiple '''terms''' in Terminology pages. It supports up to five term names and up to six definitions, and also applies [[Template:Anchor]] to the terms' name so that they can be linked to easily.

==Syntax==
===Usage===
<pre>
{{Term
|1=Term 1 |link1=Some link 1
|2=Term 2 |link2=Some link 2
|d1=Definition 1
|d2=Definition 2
}}
</pre>

===Parameters===
;''<sub>n</sub>''
:(Unnamed parameters) This is used for the ''fragment id'' of the definition term ("scheme://domain:port/path?query_string#fragment_id") and the displayed text.
;link''<sub>n</sub>''
```

### Notes

_No notes specified._

---

## Term table

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATerm%20table%2Fdoc.txt
- Decoded name: Template:Term table/doc
- Namespace: Template
- Remainder: Term table/doc
- Path parts: Template:Term table / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
This template defines the start of [[Template:Term]].

;See also
* {{t|term}}
* {{t|term table end}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Formatting templates|{{PAGENAME}}]]
</includeonly>
```

### Notes

_No notes specified._

---

## Term table end

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATerm%20table%20end%2Fdoc.txt
- Decoded name: Template:Term table end/doc
- Namespace: Template
- Remainder: Term table end/doc
- Path parts: Template:Term table end / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
This template closes [[Template:Term]].

;See also
* {{t|term table}}
* {{t|term}}

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Formatting templates|{{PAGENAME}}]]
</includeonly>
```

### Notes

_No notes specified._

---

## Texttip

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATexttip%2Fdoc.txt
- Decoded name: Template:Texttip/doc
- Namespace: Template
- Remainder: Texttip/doc
- Path parts: Template:Texttip / doc

```
{{Documentation subpage}}
;Description
:Adds a simple text tooltip to text.

;Syntax
:<code>{{t|Texttip|text|tiptext}}</code>
:<code>{{t|tt|text|tiptext}}</code>

;Parameters
* '''1/text''' = Text to show. Can include links, but be careful if you don't supply tip text.
* '''2/tiptext''' = Text for tooltip. Don't use double quotes, links or HTML tags.
* '''noline=''' = Don't show a dotted underline (no value needed).

;Example
<code>{{tl|tt|Some text|A tooltip for the text.}}</code>
* {{tt|Some text|A tooltip for the text.}}

<code>{{tl|tt|No dotted underline|A tooltip for the text.|4=noline=}}</code>
* {{tt|No dotted underline|A tooltip for the text.|noline=}}

```

### Notes

_No notes specified._

---

## The Mageseeker

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AThe%20Mageseeker%2Fdoc.txt
- Decoded name: Template:The Mageseeker/doc
- Namespace: Template
- Remainder: The Mageseeker/doc
- Path parts: Template:The Mageseeker / doc

```
{{Documentation subpage}}
This is a navigation box for [[The Mageseeker]] game. The list of characters is automatically aggregated from [[:Category:The Mageseeker]]. The icon-template used is [[Template:Unit icon]]. Expand as necessary or desired.

;Usage
{{t|The Mageseeker}}

;Customization
* <code>|hide=true</code> collapses the box, which is otherwise expanded by default

;Example
{{The Mageseeker}}

[[Category:Navigation templates]]
```

### Notes

_No notes specified._

---

## Tiles

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATiles%2Fdoc.txt
- Decoded name: Template:Tiles/doc
- Namespace: Template
- Remainder: Tiles/doc
- Path parts: Template:Tiles / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
This template uses CSS to create a tiled list, or a column layout that reads left-to-right.

== Usage ==
:See also: [[Template:Column]].
<code>{{t|tiles|type:type|1:columns|2:content}}</code>
;type (optional)
*fluid (default)
*fixed

;columns (required)
*2
*3
*4
*5
*6
*8

Fixed refers to a fixed number of columns where each column occupies a percentage of the total space. Fluid will look approximately the same under FANDOM's default browsing experience but the number of columns will increase or decrease depending on the size of your viewport. 
```

### Notes

_No notes specified._

---

## Tip

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATip%2Fdoc.txt
- Decoded name: Template:Tip/doc
- Namespace: Template
- Remainder: Tip/doc
- Path parts: Template:Tip / doc

```
__NOTOC__{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
== Usage ==
Prefixes the subject with the corresponding icon and makes a link to the relevant page. Mouse-over will generate a tooltip.

== Syntax ==
{| class="article-table"
! Name
! Accepted parameters
! Notes
|- 
| {{{1}}}
| custom input
| Indicates the subject.
|-
| {{{2}}}
| custom input
| Changes display name.
|-
| size
```

### Notes

_No notes specified._

---

## Tip data

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATip%20data%2Fdoc.txt
- Decoded name: Template:Tip data/doc
- Namespace: Template
- Remainder: Tip data/doc
- Path parts: Template:Tip data / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
;Description
* {{t|Tip data}} defines the format of [[:Category:Tip data templates|Keyword/"Tip" data templates]] and generates their documentation.
* ''Data templates'' store data centrally, which can then be retrieved for any purpose on the Wiki. '''Tip data''' templates are most commonly used for presenting information about a specific keyword via [[Template:Tip]].
* The documentation produces a table on the data template's page that lists all possible parameters and their given value. Certain parameters are related to [[crowd control]] specifically. Instructions for retrieving and formatting values are then provided.

;Creation of Tip data templates
* Create new '''Tip data''' templates using the following title scheme. Only the first letter in the ''subpage's'' title should be capitalized.
** <code><nowiki>Template:Tip data/<Name of keyword></nowiki></code> for general and League of Legends-specific content
** <code><nowiki>Template:Tip data/Tft <name of keyword></nowiki></code> for Teamfight Tactics-specific content
** <code><nowiki>Template:Tip data/Lor <name of keyword></nowiki></code> for Legends of Runeterra-specific content
** <code><nowiki>Template:Tip data/Wr <name of keyword></nowiki></code> for Wild Rift-specific content
* All '''Tip data''' templates begin with:
<pre><nowiki>{{{{{1<noinclude>|Tip data</noinclude>}}}|
}}
</nowiki></pre>
: The <nowiki><noinclude> </noinclude></nowiki> tags are necessary to prevent transclusion of the entire documentation page whenever the data template is invoked.
* Data is then stored in the data template as parameter inputs. Inputs are separated from each other by a <code>|</code> vertical bar.
** The first input must be the same as the subpage's title.
```

### Notes

_No notes specified._

---

## Tip info

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATip%20info%2Fdoc.txt
- Decoded name: Template:Tip info/doc
- Namespace: Template
- Remainder: Tip info/doc
- Path parts: Template:Tip info / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
This template converts [[Template:Tip data]] information into a format that can be inserted into articles, such as [[Types of Crowd Control]].

;Basic Example
This:
<pre>{{tip info|airborne}}</pre>

Creates this:

{{tip info|airborne}}

Another example:
{{tip info|bandit}}

Note that the '''footer''' variable is omitted by default. To restore it, set footer=true.
<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:Tooltip templates]][[de:Vorlage:Tip info]]
</includeonly>
```

### Notes

_No notes specified._

---

## TipLoR

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATipLoR%2Fdoc.txt
- Decoded name: Template:TipLoR/doc
- Namespace: Template
- Remainder: TipLoR/doc
- Path parts: Template:TipLoR / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
:''See [[Template:Tip]]''
;Description
* [[Template:Tip]] in the context of [[Legends of Runeterra]]. It automatically prefixes '''Lor''' to the keyword's name based on the link scheme for [[Template:Tip data|tip data templates]] in Legends of Runeterra, so that it does not need to be typed. For example:
<code><nowiki>{{tip|Lor obliterate|Obliterate}}</nowiki></code> and <code><nowiki>{{tipLoR|Obliterate}}</nowiki></code>
both give:

{{tip|Lor obliterate|Obliterate}}

<code><nowiki>{{tip|Lor obliterate|Reduced to atoms}}</nowiki></code> and <code><nowiki>{{tipLoR|Obliterate|Reduced to atoms}}</nowiki></code>
both give:

{{tip|Lor obliterate|Reduced to atoms}}

Currently supports the following parameters of Template:Tip:
* ''style''
* ''icononly''
* ''noimg''
* ''size''
```

### Notes

_No notes specified._

---

## TOC

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATOC%2Fdoc.txt
- Decoded name: Template:TOC/doc
- Namespace: Template
- Remainder: TOC/doc
- Path parts: Template:TOC / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->

== Usage ==

== Syntax ==

== See also ==

<includeonly>
<!-- Categories and interwikis go here: -->
[[Category:General wiki templates|{{PAGENAME}}]]
</includeonly>
```

### Notes

_No notes specified._

---

## Tocright

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATocright%2Fdoc.txt
- Decoded name: Template:Tocright/doc
- Namespace: Template
- Remainder: Tocright/doc
- Path parts: Template:Tocright / doc

```
{{Documentation subpage}}
;Description
:This template will float the article's table of contents to the right. It will also force a TOC when/where it would not normally be found.
:Use it only when needed.
;Syntax
:Type <code>{{t|tocright}}</code>

<includeonly>[[Category:General wiki templates|{{PAGENAME}}]]</includeonly><noinclude></noinclude>
```

### Notes

_No notes specified._

---

## ToDo

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3AToDo%2Fdoc.txt
- Decoded name: Template:ToDo/doc
- Namespace: Template
- Remainder: ToDo/doc
- Path parts: Template:ToDo / doc

```
#REDIRECT [[Template:Ambox documentation]][[Category:Template documentation]]
```

### Notes

_No notes specified._

---

## Tooltip

- [ ] Status: todo
- [ ] Include decision: unknown

### Source (first 20 lines)

- Source filename: Template%3ATooltip%2Fdoc.txt
- Decoded name: Template:Tooltip/doc
- Namespace: Template
- Remainder: Tooltip/doc
- Path parts: Template:Tooltip / doc

```
{{Documentation subpage}}
<!-- Categories and interwikis go at the bottom of this page. -->
== Description ==
The box design which is used to display Tooltips inside of it. Subtemplates add additional structures, which are populated by data templates.

== Associated Templates ==
This list is not complete.

* [[Template:Champion icon]] / [[Template:Champion skin link]]
** [[Template:Tooltip/Champion]]
*** [[Template:Tooltip/Champion/Data]]
* [[Template:Tip]]
** [[Template:Infotip]]
* [[Template:Runetip]]
** [[Template:Tooltip/Rune]]
*** [[Template:Tooltip/Rune/Data]]
* [[Template:Mi7]]
** [[Template:Tooltip/Mastery]]
*** [[Template:Tooltip/Mastery/Data]]

```

### Notes

_No notes specified._

---
