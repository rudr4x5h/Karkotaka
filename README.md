# Kārkotaka
Storytellers love telling **stories**, and this toolkit strives to allow
the cratives more freedom in the pursuit of what sets their :heart: on :fire:.

A story can be long, short, sweet or sour. Whatever it maybe, and whatever be your style,
this toolkit allows you to create promo material to reach the target audience faster.

## Architecture
The main entities of this program are:
1. Story - the main entity.
2. Author - every story is written by someone.
3. Domain - for better categorisation and search.
4. Body - the main body of the story.
5. Headline - the main, catchy line, of the Story.
6. Synopsis - TL;DR; a summary comprising the essence of Story.
7. Headshot - a related or promotional image/graphic for Story.

### The Big Picture
```text
Story                Author       Domain
 ├── id               ├── id       ├── name
 ├── created_at       ├── name     └── [fk] Expert
 ├── updated_at       ├── gender       └── Author
 ├── publish_status   ├── email
 ├── [fk] Author      ├── contact
 ├── [fk] Domain      ├── profile_img
 ├── [fk] Body        └── profile_uri
 ├── [fk] Headline
 ├── [fk] Synopsis    Body
 └── [fk] Headshot    ├── num_chars
                      ├── num_words
 Headline             ├── num_para
 ├── id               └── [fk] Paragraphs
 ├── content              └── Paragraph
 └── kind
                      Synopsis
 Paragraph            ├── kind
 ├── id               └── [fk] Paragraphs
 ├── content              └── Paragraph
 ├── kind
 ├── created_at       Headshot
 ├── updated_at       ├── kind
 ├── num_chars        └── [fk] Image
 └── num_words

 Image                Report
 ├── uri              ├── id
 ├── caption          ├── created_at
 └── prompt           └── [fk] Story

```
