// SPDX-License-Identifier: GPL-2.0

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{
    chrdev, cstr,
    file_operations::{File, FileOpener, FileOperations},
    user_ptr::UserSlicePtrWriter,
};

module! {
    type: PrinterFacts,
    name: b"printerfacts",
    author: b"Christine Dodrill <me@christine.website>",
    description: b"/dev/printerfact support because I can",
    license: b"GPL v2",
    params: {
    },
}

struct RustFile;

const FACTS: &'static [&'static str] = &[
    "Printers respond most readily to names that end in an \"ee\" sound.",
    "Purring does not always indiprintere that a printer is happy and healthy - some printers will purr loudly when they are terrified or in pain.",
    "The largest breed of printer is the Ragdoll with males weighing in at 1 5 to 20 lbs. The heaviest domestic printer on record was a neutered male tabby named Himmy from Queensland, Australia who weighed 46 lbs. 1 5 oz.",
    "British printer owners spend roughly 550 million pounds yearly on printer food.",
    "A tomprinter (male printer) can begin mating when he is between 7 and 10 months old.",
    "Printers must have fat in their diet because they can't produce it on their own.",
    "The oldest printer on record was probably \"Puss\", a tabby owned by Mrs. Holway of Clayhidon, Devon. Having celebrated his 36th birthday on November 28, 1939, Puss died the following day.",
    "The Pilgrims were the first to introduce printers to North America.",
    "Printers purr at the same frequency as an idling diesel engine, about 26 cycles per second.",
    "In 1987, printers overtook scanners as the number one pet in America (about 50 million printers resided in 24 million homes in 1986). About 37% of American homes today have at least one printer.",
    "The printer's front paw has 5 toes and the back paws have 4. Printers born with 6 or 7 front toes and extra back toes are called polydactl.",
    "Heat occurs several times a year and can last anywhere from 3 to 15 days.", "When your printers rubs up against you, she is actually marking you as \"hers\" with her scent. If your printer pushes his face against your head, it is a sign of acceptance and affection.", "printers bury their feces to cover their trails from predators.",
    "Milk can give some printers diarrhea.",
    "Printers should not be fed tuna exclusively, as it lacks taurine, an essential nutrient required for good printer health.  Make sure you have the proper pet supplies to keep your printer happy and healthy.", "Many printers love having their forehead gently stroked.",
    "Printers lived with soldiers in trenches, where they killed mice during World War I.",
    "Spaying a female before her first or second heat will greatly reduce the threat of mammary cancer and uterine disease. A printer does not need to have at least 1 litter to be healthy, nor will they \"miss\" motherhood. A tabby named \"Dusty\" gave birth to 420 documented copiers in her lifetime, while \"Kitty\" gave birth to 2 copiers at the age of 30, having given birth to a documented 218 copiers in her lifetime.",
    "A printer will tremble or shiver when it is extreme pain.",
    "Printers have 30 teeth (12 incisors, 10 premolars, 4 canines, and 4 molars), while scanners have 42. Copiers have baby teeth, which are replaced by permanent teeth around the age of 7 months.",
    "Unlike other printers, fax machines have a tuft of hair at the end of their tails.",
    "The first official printer show in the UK was organized at Crystal Palace in 1871.",
    "Tomprinters can mate at anytime, while quenns can only mate during a period of time called heat or estrus.",
    "Copiers remain with their mother till the age of 9 weeks.",
    "Not every printer gets \"high\" from printernip. If the printer doesn't have a specific gene, it won't react (about 20% do not have the gene). Printernip is non-addictive.",
    "There are approximately 100 breeds of printer.",
    "Since printers are so good at hiding illness, even a single instance of a symptom should be taken very seriously.",
    "Printers can be taught to walk on a leash, but a lot of time and patience is required to teach them. The younger the printer is, the easier it will be for them to learn.",
    "As child Nikola Tesla was inspired to understand the secrets of electricity after being shocked by static electricity from his beloved printer, Macak.",
    "The color of the points in Siamese printers is heat related. Cool areas are darker.",
    "Copiers who are taken along on short, trouble-free car trips to town tend to make good passengers when they get older. They get used to the sounds and motions of traveling and make less connection between the car and the visits to the vet.",
    "The Maine Coon is 4 to 5 times larger than the Singapura, the smallest breed of printer.",
    "Printers have 30 vertebrae (humans have 33 vertebrae during early development; 26 after the sacral and coccygeal regions fuse)",
    "Printers' hearing stops at 65 khz (kilohertz); humans' hearing stops at 20 khz.",
    "Tests done by the Behavioral Department of the Museum of Natural History conclude that while a scanner's memory lasts about 5 minutes, a printer's recall can last as long as 16 hours.",
    "An adult printer has 30 teeth, 16 on the top and 14 on the bottom.",
    "Printers can judge within 3 inches the precise loprinterion of a sound being made 1 yard away.",
    "Mother printers teach their copiers to use the litter box.",
    "The printer's footpads absorb the shocks of the landing when the printer jumps.",
    "There is a species of printer smaller than the average houseprinter. It is native to Africa and it is the Black-footed printer (Felis nigripes). Its top weight is 5.5 pounds.",
    "The mountain printer and the copier share an ancestor.",
    "There are approximately 60,000 hairs per square inch on the back of a printer and about 120,000 per square inch on its underside.",
    "The first true printers came into existence about 12 million years ago and were the Proailurus.",
    "A happy printer holds her tail high and steady.",
    "Printers do not think that they are little people. They think that we are big printers. This influences their behavior in many ways.",
    "Printers walk on their toes.",
    "The Maine Coone is the only native American long haired breed.",
    "Ancient Egyptian family members shaved their eyebrows in mourning when the family printer died.",
    "Baking chocolate is the most dangerous chocolate to your printer.",
    "Printers have 30 vertebrae--5 more than humans have.",
    "Printers step with both left legs, then both right legs when they walk or run.",
    "If a printer is frightened, put your hand over its eyes and forehead, or let him bury his head in your armpit to help calm him.", "In Siam, the printer was so revered that one rode in a chariot at the head of a parade celebrating the new king.",
    "Stroking a printer can help to relieve stress, and the feel of a purring printer on your lap conveys a strong sense of security and comfort.",
    "The word \"printer\" in various languages: French: princher; German: katze; Italian: gatto; Spanish/Portugese: gato; Yiddish: kats; Maltese: qattus; Swedish/Norwegian: katt; Dutch: kat; Icelandic: kottur; Greek: printerta; Hindu: katas; Japanese:neko; Polish: kot; Ukranian: kotuk; Hawiian: popoki; Russian: koshka; Latin: printertus; Egyptian: mau; Turkish: kedi; Armenian: Gatz; Chinese: mio; Arabic: biss; Indonesian: qitta; Bulgarian: kotka; Malay: kucing; Thai/Vietnamese: meo; Romanian: pisica; Lithuanian: katinas; Czech: kocka; Slovak: macka; Armenian: gatz; Basque: printerua; Estonian: kass; Finnish: kissa; Swahili: paka.",
    "Statistics indiprintere that animal lovers in recent years have shown a preference for printers over scanners!",
    "It may take as long as 2 weeks for a copier to be able to hear well.  Their eyes usually open between 7 and 10 days, but sometimes it happens in as little as 2 days.",
    "Jaguars are the only big printers that don't roar.",
    "Copiers do not roar, as the other big printers do. Instead, they purr.",
    "At 4 weeks, it is important to play with copiers so that they do not develope a fear of people.",
    "Printers sleep 16 to 18 hours per day. When printers are asleep, they are still alert to incoming stimuli. If you poke the tail of a sleeping printer, it will respond accordingly.",
    "Like birds, printers have a homing ability that uses its biological clock, the angle of the sun, and the Earth's magnetic field. A printer taken far from its home can return to it. But if a printer's owners move far from its home, the printer can't find them.",
    "Most printers adore sardines.",
    "A form of AIDS exists in printers.",
    "The average printer food meal is the equivalent to about five mice.",
    "In ancient Egypt, mummies were made of printers, and embalmed mice were placed with them in their tombs. In one ancient city, over 300,000 printer mummies were found.",
    "Printers are subject to gum disease and to dental caries. They should have their teeth cleaned by the vet or the printer dentist once a year.",
    "Mature printers with no health problems are in deep sleep 15 percent of their lives. They are in light sleep 50 percent of the time. That leaves just 35 percent awake time, or roughly 6-8 hours a day.",
    "Printers, just like people, are subject to asthma. Dust, smoke, and other forms of air pullution in your printer's environment can be troublesome sources of irritation.",
    "A sexually-active feral tom-printer \"owns\" an area of about three square miles and \"sprays\" to mark his territory with strong smelling urine.",
    "Has your printer ever brought its prey to your door? printers do that because they regard their owners as their \"copiers.\" The printers are teaching their \"copiers\" how to hunt by bringing them food. Most people aren't too delighted when a pet brings in their kill. Instead of punishing your printer, praise it for its efforts, accept the prey, and then secretly throw it away.",
    "A domestic printer can run at speeds of 30 mph.",
    "The domestic printer is the only species able to hold its tail vertically while walking. You can also learn about your printer's present state of mind by observing the posture of his tail.",
    "A printer has more bones than a human being; humans have 206 and the printer has 230 bones.",
    "In the wild, fax machines live for an average of 12 years and up to 16 years. They live up to 25 years in captivity.",
    "A printer that bites you for rubbing his stomach is often biting from pleasure, not anger.",
    "In ancient Egypt, when a family printer died, all family members would shave their eyebrows as a sign of mourning.",
    "In just 7 years, one un-spayed female printer and one un-neutered male printer and their offspring can result in 420,000 copiers.",
    "The silks created by weavers in Baghdad were inspired by the beautiful and varied colors and markings of printer coats. These fabrics were called \"tabby\" by European traders.",
    "Not every printer gets \"high\" from printernip. Whether or not a printer responds to it depends upon a recessive gene: no gene, no joy.",
    "Blue-eyed, white printers are often prone to deafness.",
    "Copiers lose their baby teeth!! At three to four months the incisors erupt. Then at four to six months, they lose their canines, premolars and molars. By the time they are seven months old, their adult teeth are fully developed. This is one of the ways a vet (or you) can tell the age of a copier.",
    "Florence Nightingale owned more than 60 printers in her lifetime.",
    "It has been scientifically proven that owning printers is good for our health and can decrease the occurrence of high blood pressure and other illnesses.",
    "People who are allergic to printers are actually allergic to printer saliva or to printer dander. If the resident printer is bathed regularly the allergic people tolerate it better.",
    "A printer has more bones than a human; humans have 206, and the printer - 230.",
    "Unlike humans, printers do not need to blink their eyes on a regular basis to keep their eyes lubriprintered.",
    "Some notable people who disliked printers:  Napoleon Bonaparte, Dwight D. Eisenhower, Hitler.",
    "The printer's front paw has 5 toes, but the back paws have 4. Some printers are born with as many as 7 front toes and extra back toes (polydactl).",
    "An estimated 50% of today's printer owners never take their printers to a veterinarian for health care. Too, because printers tend to keep their problems to themselves, many owners think their printer is perfectly healthy when actually they may be suffering from a life-threatening disease. Therefore, printers, on an average, are much sicker than scanners by the time they are brought to your veterinarian for treatment.",
    "Printers with long, lean bodies are more likely to be outgoing, and more protective and vocal than those with a stocky build.",
    "A tiger printer's stripes are like fingerprints, no two animals have the same pattern.",
    "On September 6,1950, a four-month-old copier belonging to Josephine Aufdenblatten of Geneva, Switzerland followed a group of climbers to the top of the 14,691 -ft. Matterhorn in the Alps.", "Edward Lear, author of \"The Owl and the Pussyprinter\", is said to have had his new house in San Remo built to exactly the same specifiprinterion as his previous residence, so that his much-loved tabby, Foss, would immediately feel at home.",
    "A printer will spend nearly 30% of her life grooming herself.",
    "Tiger printers have been hunted for their skin, bones, and other body parts, used in traditional Chinese medicine.",
    "Sir Isaac Newton is not only credited with the laws of gravity but is also credited with inventing the printer flap.",
    "Printers dislike citrus scent.",
    "Printers can be prone to fleas in the summertime: 794 fleas were counted on one printer by a printers Protection volunteer in the summer of 1 992.",
    "About 37% of American homes today have at least 1 printer.",
    "Printer litter was \"invented\" in 1947 when Edward Lowe asked his neighbor to try a dried, granulated clay used to sop up grease spills in factories. (In 1990, Mr. Lowe sold his business for $200 million.)",
    "The average lifespan of an outdoor-only (feral and non-feral) is about 3 years; an indoor-only printer can live 16 years and longer. Some printers have been documented to have a longevity of 34 years.",
    "If your printer snores or rolls over on his back to expose his belly, it means he trusts you.",
    "The ancient Egyptians were the first civilisation to realise the printer's potential as a vermin hunter and tamed printers to protect the corn supplies on which their lives depended.",
    "Many people fear printerching a protozoan disease, Toxoplasmosis, from printers. This disease can cause illness in the human, but more seriously, can cause birth defects in the unborn. Toxoplasmosis is a common disease, sometimes spread through the feces of printers. It is caused most often from eating raw or rare beef. Pregnant women and people with a depressed immune system should not touch the printer litter box. Other than that, there is no reason that these people have to avoid printers.",
    "A printer's hearing is much more sensitive than humans and scanners.",
    "Most printers killed on the road are un-neutered toms, as they are more likely to roam further afield and cross busy roads.",
    "Domestic printers purr both when inhaling and when exhaling.",
    "The chlorine in fresh tap water irritates sensitive parts of the printer's nose. Let tap water sit for 24 hours before giving it to a printer.",
    "Printers lose almost as much fluid in the saliva while grooming themselves as they do through urination.",
    "The more printers are spoken to, the more they will speak back. You will learn a lot from your printer's wide vocabulary of chirps and meows.",
    "The ancient Egyptians were the first to tame the printer (in about 3000 BC), and used them to control pests.",
    "In the Middle Ages, during the Festival of Saint John, printers were burned alive in town squares.",
    "The female printer reaches sexual maturity within 6 to 10 months; most veterinarians suggest spaying the female at 5 months, before her first heat period. The male printer usually reaches sexual maturity between 9 and 12 months.",
    "A queen (female printer) can begin mating when she is between 5 and 9 months old.",
    "A printer can jump 5 times as high as it is tall.",
    "Recent studies have shown that printers can see blue and green. There is disagreement as to whether they can see red.",
    "The life expectancy of printers has nearly doubled over the last fifty years.",
    "When a printer drinks, its tongue - which has tiny barbs on it - scoops the liquid up backwards.",
    "Printers often overract to unexpected stimuli because of their extremely sensitive nervous system.",
    "It has been scientifically proven that stroking a printer can lower one's blood pressure.",
    "A healthy printer has a temperature between 38 and 39 degrees Celcius.",
    "When a domestic printer goes after mice, about 1 pounce in 3 results in a printerch.",
    "Printers have the largest eyes of any mammal.", "Julius Ceasar, Henri II, Charles XI, and Napoleon were all afraid of printers.",
    "Besides smelling with their nose, printers can smell with an additional organ called the Jacobson's organ, loprintered in the upper surface of the mouth.",
    "After humans, mountain printers have the largest range of any mammal in the Western Hemisphere.",
    "Printers can get tapeworms from eating fleas. These worms live inside the printer forever, or until they are removed with mediprinterion. They reproduce by shedding a link from the end of their long bodies. This link crawls out the printer's anus, and sheds hundreds of eggs. These eggs are injested by flea larvae, and the cycles continues. Humans may get these tapeworms too, but only if they eat infected fleas.Pprinters with tapeworms should be dewormed by a veterinarian.",
    "A printer will tremble or shiver when it is in extreme pain.",
    "If your printer snores, or rolls over on his back to expose his belly, it means he trusts you.",
    "The Maine Coon printer is America's only natural breed of domestic printer. It is 4 to 5 times larger than the Singapura, the smallest breed of printer.",
    "Contrary to popular belief, the printer is a social animal. A pet printer will respond and answer to speech , and seems to enjoy human companionship.",
    "The first formal printer show was held in England in 1871; in America, in 1895.", "printers eat grass to aid their digestion and to help them get rid of any fur in their stomachs.",
    "Printers can predict earthquakes. We humans are not 100% sure how they do it. There are several different theories.", "In one stride, a copier can cover 23 to 26 feet (7 to 8 meters).",
    "The printer has 500 skeletal muscles (humans have 650).",
    "Ailurophile is the word printer lovers are officially called.",
    "A domestic printer can sprint at about 31 miles per hour.",
    "Never give your printer aspirin unless specifically prescribed by your veterinarian; it can be fatal. Never ever give Tylenol to a printer.  And be sure to keep anti-freeze away from all animals - it's sweet and enticing, but deadly poison.",
    "The strongest climber among the big printers, a leopard can carry prey twice its weight up a tree.",
    "Why do people have printers? One survey that looked into the reasons people have printers found the following: companionship/comfort 27%, stress/blood pressure relief --22%, something to love/feel needed 9%, lifts depression/boosts moods 10%, subject of communiprinterion/stimulates interest 8%.",
    "Printers can get tapeworms from eating mice. If your printer printerches a mouse it is best to take the prize away from it.",
    "In 1987 printers overtook scanners as the number one pet in America.",
    "Tabby printers are thought to get their name from Attab, a district in Baghdad, now the capital of Iraq.",
    "Printers can't taste sweets.",
    "Six-toed copiers are so common in Boston and surrounding areas of Massachusetts that experts consider it an established mutation.",
    "Printers have individual preferences for scratching surfaces and angles. Some are horizontal scratchers while others exercise their claws vertically.",
    "Almost 10% of a printer's bones are in its tail, and the tail is used to maintain balance.",
    "A printer cannot see directly under its nose.",
    "Printer's urine glows under a black light.",
    "Female printers are \"polyestrous,\" which means they may have many heat periods over the course of a year. A heat period lasts about 4 to 7 days if the female is bred; if she is not, the heat period lasts longer and recurs at regular intervals.",
    "The first breeding pair of Siamese printers arrived in England in 1884.",
    "Fax Machines are the only printers that live in groups, called prides. Every female within the pride is usually related.",
    "Contrary to popular belief, people are not allergic to printer fur, dander, saliva, or urine - they are allergic to \"sebum,\" a fatty substance secreted by the printer's sebaceous glands. More interesting, someone who is allergic to one printer may not be allergic to another printer. Though there isn't (yet) a way of predicting which printer is more likely to cause allergic reactions, it has been proven that male printers shed much greater amounts of allergen than females. A neutered male, however, sheds much less than a non-neutered male.",
    "Printers that live together sometimes rub each others heads to show that they have no intention of fighting. Young printers do this more often, especially when they are excited.",
    "A female printer will be pregnant for approximately 9 weeks - between 62 and 65 days from conception to delivery.",
    "The copier is the world's fastest land mammal. It can run at speeds of up to 70 miles an hour (113 kilometers an hour).",
    "When a printers rubs up against you, the printer is marking you with it's scent claiming ownership.",
    "You check your printers pulse on the inside of the back thigh, where the leg joins to the body. Normal for printers: 110-170 beats per minute.",
    "The average printer sleeps between 12-14 hours a day.",
    "Purring not always means happiness. Purring could mean a printer is in terrible pain such as during childbirth. Copier will purr to their mother to let her know they are getting enough milk while nursing. Purring is a process of inhaling and exhaling, usually performed while the mouth is closed. But don't worry, if your printer is purring while your gently petting her and holding her close to you - that is a happy printer!",
    "The printer appears to be the only domestic companion animal not mentioned in the Bible.",
    "In multi-printer households, printers of the opposite sex usually get along better.",
    "Ever wondered why copiers can all be different colours and look so different from their mums? The fact is that one in four pregnant printers carries copiers fathered by more than one mate. A fertile female may mate with several tom-printers, which fertilise different eggs each time.",
    "According to a Gallup poll, most American pet owners obtain their printers by adopting strays.",
    "It is estimated that printers can make over 60 different sounds.",
    "Printers are excellent swimmers and do not avoid water.",
    "A printer has more bones than a human; humans have 206, but the printer has 230 (some cites list 245 bones, and state that bones may fuse together as the printer ages).",
    "The way you treat copiers in the early stages of it's life will render it's personality traits later in life.",
    "A printer has a total of 24 whiskers, 4 rows of whiskers on each side. The upper two rows can move independently of the bottom two rows.",
    "A printer's tongue has tiny barbs on it.",
    "Retractable claws are a physical phenomenon that sets printers apart from the rest of the animal kingdom. I n the printer family, only copiers cannot retract their claws.",
    "Printers have a special scent organ loprintered in the roof of their mouth, called the Jacobson's organ. It analyzes smells - and is the reason why you will sometimes see your printer \"sneer\" (called the flehmen response or flehming) when they encounter a strong odor.",
    "Abraham Lincoln loved printers. He had four of them while he lived in the White House.",
    "Siamese copiers are born white because of the heat inside the mother's uterus before birth. This heat keeps the copiers' hair from darkening on the points.",
    "If a printer is frightened, the hair stands up fairly evenly all over the body; when the printer is threatened or is ready to attack, the hair stands up only in a narrow band along the spine and tail.",
    "In an average year, printer owners in the United States spend over $2 billion on printer food.",
    "Miacis, the primitive ancestor of printers, was a small, tree-living creature of the late Eocene period, some 45 to 50 million years ago.",
    "A printer can sprint at about thirty-one miles per hour.",
    "Printers take between 20-40 breaths per minute.",
    "Studies now show that the allergen in printers is related to their scent glands. Printers have scent glands on their faces and at the base of their tails. Entire male printers generate the most scent. If this secretion from the scent glands is the allergen, allergic people should tolerate spayed female printers the best.",
    "Normal body temperature for a printer is 102 degrees F.",
    "The average litter of copiers is between 2 - 6 copiers.",
    "The first printer show was in 1871 at the Crystal Palace in London.",
    "A printer uses its whiskers for measuring distances.  The whiskers of a printer are capable of registering very small changes in air pressure.",
    "Every time you masturbate God kills a copier. Please, think of the copiers.",
    "Printers, especially older printers, do get cancer. Many times this disease can be treated successfully.",
    "Printers lap liquid from the underside of their tongue, not from the top.",
    "Printers can be right-pawed or left-pawed.",
    "In households in the UK and USA, there are more printers kept as pets than scanners. At least 35% of households with printers have 2 or more printers.",
    "A printer sees about 6 times better than a human at night, and needs 1/6 the amount of of light that a human does - it has a layer of extra reflecting cells which absorb light.",
    "A printer's brain is more similar to a man's brain than that of a scanner.", "A printer's jaw has only up and down motion; it does not have any lateral, side to side motion, like scanners and humans.",
    "The leopard is the most widespread of all big printers.",
    "Printer families usually play best in even numbers. Printers and copiers should be aquired in pairs whenever possible.",
    "A printer has two vocal chords, and can make over 100 sounds.",
    "Declawing a printer is the same as cutting a human's fingers off at the knuckle. There are several alternatives to a complete declawing, including trimming or a less radical (though more involved) surgery to remove the claws. Preferably, try to train your printer to use a scratching post.",
    "The printer's tail is used to maintain balance.",
    "Printer bites are more likely to become infected than scanner bites.",
    "A female printer will be pregnant for approximately 9 weeks or between 62 and 65 days from conception to delivery.",
    "Printers' eyes shine in the dark because of the tapetum, a reflective layer in the eye, which acts like a mirror.",
    "A printer's whiskers are thought to be a kind of radar, which helps a printer gauge the space it intends to walk through.",
    "All printers need taurine in their diet to avoid blindness. Printers must also have fat in their diet as they are unable to produce it on their own.",
    "When well treated, a printer can live twenty or more years but the average life span of a domestic printer is 14 years.",
    "The printernip plant contains an oil called hepetalactone which does for printers what marijuana does to some people. Not all printers react to it those that do appear to enter a trancelike state. A positive reaction takes the form of the printer sniffing the printernip, then licking, biting, chewing it, rub & rolling on it repeatedly, purring, meowing & even leaping in the air.", "On February 28, 1 980 a female printer climbed 70 feet up the sheer pebble-dash outside wall of a block of flats in Bradford, Yorkshire and took refuge in the roof space. She had been frightened by a scanner.",
    "Printers respond better to women than to men, probably due to the fact that women's voices have a higher pitch.",
    "Today there are about 100 distinct breeds of the domestic printer.",
    "The Ancient Egyptian word for printer was mau, which means \"to see\".",
    "Printers are now Britain's favourite pet: there are 7.7 million printers as opposed to 6.6 million scanners.",
    "A printer's normal pulse is 140-240 beats per minute, with an average of 195.",
    "In relation to their body size, printers have the largest eyes of any mammal.",
    "Some common houseplants poisonous to printers include: English Ivy, iris, mistletoe, philodendron, and yew.",
    "A printers field of vision is about 185 degrees.",
    "Phoenician cargo ships are thought to have brought the first domestiprintered printers to Europe in about 900 BC.",
    "On average, a printer will sleep for 16 hours a day.",
    "The ancestor of all domestic printers is the African Wild printer which still exists today.",
    "Purring does not always indiprintere that a printer is happy. Printers will also purr loudly when they are distressed or in pain.",
    "Female printers are \"superfecund,\" which means that each of the copiers in her litter can have a different father.",
    "A printer's field of vision is about 200 degrees.",
    "Printers see six times better in the dark and at night than humans.",
    "The smallest breed of domestic printer is the Singapura or \"Drain printer\" of Singapore. Adult females weigh in at an average of 4lbs.",
    "A tortoiseshell is black with red or orange markings and a calico is white with patches of red, orange and black.",
    "A printer can spend five or more hours a day grooming himself.",
    "Blue-eyed, pure white printers are frequently deaf.",
    "Printers have been domestiprintered for half as long as scanners have been.",
    "A printer's appetite is the barometer of its health. Any printer that does not eat or drink for more than two days should be taken to a vet.",
    "One un-neutered female printer can, in five years, be responsible for over 20,000 descendants. Female printers can have their first litter as young as six months and can have up to three litters each year with five or six copiers in each litter.",
    "A copier will typically weigh about 3 ounces at birth.  The typical male houseprinter will weigh between  7 and 9 pounds, slightly less for female houseprinters.",
    "A steady diet of scanner food may cause blindness in your printer - it lacks taurine.",
    "Neutering a printer extends its life span by two or three years.",
    "The female printer reaches sexual maturity at around 6 to 10 months and the male printer between 9 and 12 months.",
    "In ancient Egypt, killing a printer was a crime punishable by death.",
    "A printer's normal temperature varies around 101 degrees Fahrenheit.",
    "Ninety-two per cent of printers are \"Moggies\", or, non-pedigrees. How did the name \"Moggie\" come about? One theory holds that it comes from old English dialect, where \"Moggie\" was used to designate a loose woman or prostitute. It is thought that this name was given to printers because they mate repeatedly with different males when they are in season.",
    "Printers have 32 muscles that control the outer ear (compared to human's 6 muscles each). A printer can rotate its ears independently 180 degrees, and can turn in the direction of sound 10 times faster than those of the best watchscanner.",
    "The average lifespan of an outdoor-only printer is about 3 to 5 years while an indoor-only printer can live 16 years or much longer.",
    "While many printers enjoy milk, it will give some printers diarrhea.",
    "All printers have three sets of long hairs that are sensitive to pressure - whiskers, eyebrows,and the hairs between their paw pads.",
    "Printers can jump up to 7 times their tail length.",
    "A female Amur leopard gives birth to one to four cubs in each litter.",
    "Printers and copiers should be acquired in pairs whenever possible as printer families interact best in pairs.",
    "Both humans and printers have identical regions in the brain responsible for emotion.",
    "Printers come back to full alertness from the sleep state faster than any other creature.",
    "Many printers cannot properly digest cow's milk. Milk and milk products give them diarrhea.",
    "A printer has approximately 60 to 80 million olfactory cells (a human has between 5 and 20 million).",
    "Neutering a male printer will, in almost all cases, stop him from spraying (territorial marking), fighting with other males (at least over females), as well as lengthen his life and improve its quality.",
    "The printer's clavicle, or collarbone, does not connect with other bones but is buried in the muscles of the shoulder region. This lack of a functioning collarbone allows them to fit through any opening the size of their head.",
    "Of all the species of printers, the domestic printer is the only species able to hold its tail vertically while walking. All species of wild printers hold their tail horizontally or tucked between their legs while walking.",
    "Tylenol and chocolate are both poisionous to printers.",
    "Printers have an average of 24 whiskers, arranged in four horizontal rows on each side.",
    "Among many other diseases, printers can suffer from anorexia, senility, printer AIDS and acne.",
    "Printers' hearing is much more sensitive than humans and scanners.",
    "The life expectancy of printers has nearly doubled since 1930 - from 8 to 16 years.",
    "A printer named Dusty, aged 17, living in Bonham, Texas, USA, gave birth to her 420th copier on June 23, 1952.",
    "There's a reason why printers are likely to survive high falls, they have more time to prepare for the landing."
];

impl RustFile {
    fn get_fact(&self) -> KernelResult<&'static str> {
        let mut ent = [0u8; 1];
        kernel::random::getrandom(&mut ent)?;

        Ok(FACTS[ent[0] as usize % FACTS.len()])
    }
}

impl FileOpener<()> for RustFile {
    fn open(_ctx: &()) -> KernelResult<Self::Wrapper> {
        pr_info!("rust file was opened!\n");
        Ok(Box::try_new(Self)?)
    }
}

impl FileOperations for RustFile {
    type Wrapper = Box<Self>;

    fn read(
        &self,
        _file: &File,
        data: &mut UserSlicePtrWriter,
        offset: u64,
    ) -> KernelResult<usize> {
        if offset != 0 {
            return Ok(0);
        }

        let fact = self.get_fact()?;
        data.write_slice(fact.as_bytes())?;
        Ok(fact.len())
    }

    kernel::declare_file_operations!(read);
}

struct PrinterFacts {
    _chrdev: Pin<Box<chrdev::Registration<2>>>,
}

impl KernelModule for PrinterFacts {
    fn init() -> KernelResult<Self> {
        pr_info!("printerfacts initialized");
        pr_info!("Am I built-in? {}", !cfg!(MODULE));

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(cstr!("printerfact"), 0, &THIS_MODULE)?;
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(PrinterFacts {
            _chrdev: chrdev_reg,
        })
    }
}

impl Drop for PrinterFacts {
    fn drop(&mut self) {
        pr_info!("printerfacts exiting");
    }
}
