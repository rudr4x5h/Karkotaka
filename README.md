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
 Story                            Author          
 ├── id                           ├── id          
 ├── created_at                   ├── name        
 ├── updated_at                   ├── gender      
 ├── [fk] Author                  ├── email       
 ├── [fk] Domain                  ├── contact     
 ├── [fk] Body                    ├── profile_img 
 ├── [fk] Headline                └── profile_uri 
 ├── [fk] Synopsis                                
 └── [fk] Headshot                Domain          
                                  ├── name        
                                  └── [fk] Expert 
  Headshot                            └── Author  
  ├── kind                                        
  └── [fk] Image              Body                
                              ├── num_chars       
  Paragraph      Headline     ├── num_words       
  ├── id         ├── id       ├── num_para        
  ├── content    ├── content  └── [fk] Paragraphs 
  ├── kind       └── kind         └── Paragraph   
  ├── created_at                                  
  ├── updated_at              Synopsis            
  ├── num_chars               ├── kind            
  └── num_words               └── [fk] Paragraphs 
                                  └── Paragraph   
  Image             Report                        
  ├── uri           ├── id                        
  ├── caption       ├── created_at                
  └── prompt        └── [fk] Story                

```


