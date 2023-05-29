@echo off
color 0b
title Keegan's Game

set pocketboy = 0
set questaccept = 0

:Scriptstart
:: Dang this is taking too long to make. I wish I could code faster.
echo Welcome to Keegan's Quest: The Story of the Time Travel War
pause
echo.
echo Are you ready to start? If so, select "New Game". Or, select "Load Game" to go to the chapter that you are on. But, if you don't want to play, select "Leave Game".
echo.
:: cmdMenuSel made by Nave GCT.
cmdMenuSel f870 "New Game" "Load Game" "Leave Game" "Test"
if %ERRORLEVEL% == 1 goto wakeupboi
if %ERRORLEVEL% == 2 goto Loadgame
if %ERRORLEVEL% == 3 exit
if %ERRORLEVEL% == 4 goto c4usekeypocketboy
goto wakeupboi

:Loadgame
cls
echo Type in your code here.
set /p chaptercode=Code:
if "%chaptercode%" == "15201" goto Chapter1
if "%chaptercode%" == "97109" goto Chapter2
if "%chaptercode%" == "61042" goto c2housemap
if "%chaptercode%" == "47728" goto Chapter3
if "%chaptercode%" == "09315" goto c4start

:invalidcode
echo.
echo That was an invalid chapter code. Try again.
pause
goto Loadgame


:wakeupboi
cls
echo Chapter 0: Wake Up!
echo.
echo Kameron: Keegan, you have to wake up!
pause
echo.
echo Keegan gets up with a annoyed tone.
pause


:timetotalklol
echo.
echo Responces:
echo.
cmdMenuSel f870 "Why did you wake me up?!" "What's wrong?" "Let me sleep..."
if %ERRORLEVEL% == 1 goto res1
if %ERRORLEVEL% == 2 goto res2
if %ERRORLEVEL% == 3 goto res3
goto wakeupboi

:res1
cls
echo Kameron: Dude! Chill...
pause
echo.
echo Keegan: What is it then?!
pause
echo.
echo Kameron: Elias tasked me and you with a mission.
pause
echo.
echo Keegan: What mission?
pause
echo.
echo Kameron: Elias wants me to work on some time traveling machine, and you have to test it.
pause
echo.
echo Keegan: Got it. Ready when you are.
pause
echo.
echo Kameron: Ok, head to the VR Room, and when you are done, head to me.
pause
echo.
echo Keegan: Got it.
pause
goto howtoplay

:res2
cls
echo Kameron: Elias tasked me and you with a mission.
pause
echo.
echo Keegan: What mission?
pause
echo.
echo Kameron: Elias wants me to work on some time traveling machine, and you have to test it.
pause
echo.
echo Keegan: Got it. Ready when you are.
pause
echo.
echo Kameron: Ok, head to the VR Room, and when you are done, head to me.
pause
echo.
echo Keegan: Got it.
pause
goto howtoplay


:res3
cls
echo Kameron: There is no time to sleep!
pause
goto timetotalklol

:howtoplay
cls
echo Chapter 0.5: The Tutorial
echo.
echo Kameron: Hello, it seems that you want to play right?
pause
echo.
echo Kameron: Alright.
pause
echo.
echo Kameron: So, you will be taking the role of my brother, the owner of the base Fort Bradley, which was created by the team me and my brother is a part of, called "The Ghosts".
pause
echo.
echo Kameron: Why am I telling you this?
pause
echo.
echo Kameron: So you know what my game is about.
pause
echo.
echo Kameron: And now, time for the tutorial!
pause
echo.
echo Kameron: You ready? Let's go.
pause
echo.
echo Kameron: Go ahead, Game.
pause
echo.
echo Ok, first off, there is commands that you will use thoughout your adventure.
pause
echo.
echo There are different commands that you will need to select.
pause
echo.
echo There will also be a map. Let me show what the symbols mean. "[" means that there is a wall there. "." means you can move there. "A" means that is a interactable area. "K" is you. "E" is a examinable area. "T" means you can talk to it.
pause
echo.
echo Here's a map.
pause
echo -----
echo [K.A]
echo -----
pause
echo.
echo Do you see that "K"? That is you! That is what you will be moving around.
pause
echo.
echo Alright, now for the commands.
pause
echo.
echo If you saw on the map, you can go east. So, let's go east.
pause
goto howtoplaycommands

:howtoplaycommands
echo.
echo To move east, select east. Simple as that.
cmdMenuSel f870 "East"
if %ERRORLEVEL% == 1 goto howtoplay2

:howtoplayic
echo.
echo You type invalid code! Try again.
pause
goto howtoplaycommands

:howtoplay2
cls
echo -----
echo [.KA]
echo -----
echo.
echo You have now move next to the interactive object, nice job!
pause
echo.
echo Now, time to interact with the object, which will end the tutorial, let's go!
echo Select "use" to use the object.
cmdMenuSel f870 "Use"
if %ERRORLEVEL% == 1 goto howtoplay3


:howtoplay3
cls
echo Kameron: So, since you hit the button, now we're going to head to the game. So, let's go!
pause
goto Chapter1code

:Chapter1code
cls
echo.
echo Before continuing, save this code somewhere so that you can come back to this: 15201
pause
goto Chapter1

:Chapter1
cls
echo Chapter 1: The Device
echo.
echo Kameron: Alright, Keegan. I'm almost done with the machine.
pause
echo.
echo Keegan: Could you please hurry up?
pause
echo.
echo Kameron: Yeah, I'm almost done. Why don't you go to the computer over there and play some games while you wait.
pause
echo.
echo Keegan: You got it.
pause
goto movecomputermap

:movecomputermap
cls
echo ------
echo [A.KT]
echo ------
echo.
cmdMenuSel f870 "Talk" "West"
if %ERRORLEVEL% == 1 goto talkcomputermap
if %ERRORLEVEL% == 2 goto movecomputermap2

:talkcomputermap
cls
echo.
echo You are chatting with: Kameron
pause
echo.
echo Keegan: Hey.
pause
echo.
echo Kameron: I'm not done yet, go on the computer, or something.
pause
echo.
echo Keegan: Ok.
pause
goto movecomputermap

:movecomputermap2
cls
echo ------
echo [AK.T]
echo ------
echo.
cmdMenuSel f870 "Use" "East"
if %ERRORLEVEL% == 1 goto Chapter1aftercomputer
if %ERRORLEVEL% == 2 goto movecomputermap

:Chapter1aftercomputer
echo Keegan heads towards the computer and turns it on.
pause
echo.
echo Kameron: Oh yeah, I got your favorite game on there.
pause
echo.
echo Keegan: YES!
pause
echo.
echo Keegan jumps on the computer chair and starts playing the game on the computer.
pause
echo.
echo Keegan makes it through the first level, and then............
pause
echo.
echo Kameron: It's done!
pause
echo.
echo Keegan: Alright, I'm coming!
pause
goto acmap

:acmap
cls
echo ------
echo [AK.T]
echo ------
echo.
cmdMenuSel f870 "Use" "East"
if %ERRORLEVEL% == 1 goto acuse
if %ERRORLEVEL% == 2 goto acmap2

:acuse
cls
echo Keegan: I already used that. Kameron's calling me to him.
pause
goto acmap

:acmap2
cls
echo ------
echo [A.KT]
echo ------
echo.
cmdMenuSel f870 "Talk" "East"
if %ERRORLEVEL% == 1 goto Chapter1kameron
if %ERRORLEVEL% == 2 goto acmap

:Chapter1kameron
cls
echo You are chatting with: Kameron
pause
echo.
echo Keegan: I'm here.
pause
echo.
echo Kameron: Alright, now, let me install this chip in your cyber glove.
pause
echo.
echo Keegan: Alright.
pause
echo.
echo Kameron installs the chip into Keegan's glove, and it turns on.
pause
echo.
echo Kameron: Now you will be able to time travel!
pause
echo.
echo Keegan: Neat.
pause
echo.
echo Kameron: Do you want to test it?
pause
echo.
echo Keegan: Sure.
pause
echo.
echo Kameron: Where do you want to go?
pause
echo.
echo Keegan: Normandy, June 6, 1944.
pause
echo.
echo Kameron: Ok, let's go.
pause
echo.
echo Kameron: 3.
pause
echo.
echo Kameron: 2.
pause
echo.
echo Kameron: 1.
pause
echo.
echo Kameron pulls the switch, and Keegan teleports to Normandy.
pause
cls
echo End of Chapter 1.
pause
goto Chapter2Code


:Chapter2Code
cls
echo Load Chapter Code: 97109
pause
echo.
echo When you are ready,
pause
goto Chapter2

:Chapter2
cls
echo Chapter 2: Normandy
echo.
echo Keegan is blinded by the dark, until he sees the light. He enters the light, and he is now in Normandy.
pause
echo.
echo Keegan: Nice, I made it to Normandy.
pause
echo.
echo ???: Schnell! Suche Deckung! (Quick! Find cover!)
pause
echo.
echo Keegan: Crap! It's the Germans! I shouldn't get in their line of sight.
pause
goto Chapter2map

:Chapter2map
cls
echo ---D---
echo [  .  ]
echo [?    ]
echo ---   ]
echo K .   ]
echo -------
echo.
echo commands: east
echo.
echo New Symbols: "B" is a "bad guy". You want to try and kill them. "D" is a door. "?" is an unknown object.
cmdMenuSel f870 "East" "Earpiece"
if %ERRORLEVEL% == 1 goto Chapter2map2
if %ERRORLEVEL% == 2 goto Chapter2mapchat1


:c2mapic
echo.
echo That was an invalid command, try again.
pause
goto Chapter2map

:Chapter2mapchat1
cls
echo Keegan: What should I do?

:Chapter2map2
cls
echo ---D---
echo [  .  ]
echo [?    ]
echo ---   ]
echo . K   ]
echo -------
echo.
echo commands: examine, north
cmdMenuSel f870 "Examine" "North"
if %ERRORLEVEL% == 1 goto Echapter2map2
if %ERRORLEVEL% == 2 goto chapter2death1


:c2map2ic
echo.
echo That was an invalid command, try again.
pause
goto Chapter2map2

:chapter2death1
cls
echo Keegan was caught, and was thrown into Prison for trespassing.
pause
echo.
echo Want to goto the nearest checkpoint?
echo.
echo commands: yes, no
cmdMenuSel f870 "Yes" "No"
if %ERRORLEVEL% == 1 goto Chapter2map
if %ERRORLEVEL% == 2 goto c2leave

:c2leave
cls
echo When you are ready to come back, use this code: 97109
pause
exit

:Echapter2map2
cls
echo Keegan: Looks like there is a Nazi there. That's not good.
pause
cls
echo ---D---
echo [  .  ]
echo [B    ]
echo ---   ]
echo . K   ]
echo -------
echo.
echo commands: call, north
cmdMenuSel f870 "Call" "North"
if %ERRORLEVEL% == 1 goto c2cmap
if %ERRORLEVEL% == 2 goto chapter2death1

:ec2map
echo.
echo That was an invalid command, try again.
pause
goto Echapter2map2

:c2cmap
cls
echo ---D---
echo [  .  ]
echo [  B  ]
echo ---   ]
echo . K   ]
echo -------
echo.
echo commands: wait
cmdMenuSel f870 "Wait"
if %ERRORLEVEL% == 1 goto c2cmap2


:c2cmapic
echo.
echo That was an invalid command, try again.
pause
goto c2cmap

:c2cmap2
cls
echo ---D---
echo [  .  ]
echo [     ]
echo ---B  ]
echo   K   ]
echo -------
echo.
echo commands: ko
cmdMenuSel f870 "KO" " "
if %ERRORLEVEL% == 1 goto c2mapko
if %ERRORLEVEL% == 2 goto c2smapko

:c2cmap2ic
echo.
echo That was an invalid command, try again.
pause
goto c2cmap2

:c2mapko
cls
echo Keegan chokes the Nazi til the guy is knocked out.
pause
echo.
echo Keegan: Well, now I can get to somewhere safer.
pause
goto c2mapko0

:c2smapko
echo Keegan jumps out and yells "Boo!" at the Nazi.
pause
echo.
echo The German yells, and falls dead from getting so scared.
pause
echo.
echo Keegan: Well, that was interesting
pause
goto c2mapko0

:c2mapko0
cls
echo ---D---
echo [  .  ]
echo [     ]
echo ---   ]
echo   K   ]
echo -------
echo.
echo commands: north
cmdMenuSel f870 "North"
if %ERRORLEVEL% == 1 goto komapdoor

:komapdoor
cls
echo ---D---
echo [  K  ]
echo [     ]
echo ---   ]
echo   .   ]
echo -------
echo.
echo commands: open, south
cmdMenuSel f870 "Open" "South"
if %ERRORLEVEL% == 1 goto hc2
if %ERRORLEVEL% == 2 goto c2mapko0


:komapdooric
echo.
echo That was an invalid command, try again.
pause
goto komapdoor

:hc2
cls
echo Keegan enters the house and closes the door.
pause
echo.
echo Keegan: Alright, now, I'm safe.
pause
echo.
echo German: �berpr�fen Sie die H�user! (Check the houses!)
pause
echo.
echo Keegan: Uhggg, really? Fine. Guess I will have to hide.
pause
goto c2housemap

:c2housemap
cls
echo Checkpoint Code: 61042
echo.
echo -------------------
echo [  H          H   ]  Plant (Left) Stove (Right)
echo [                 ]
echo [        -----D----
echo [   S    [        ] Stairs (Left)
echo D K      [    H   ] Toilet
echo -------------------
echo.
echo "H" means you can hide there.
echo.
echo commands: stairs, toilet, plant, stove
cmdMenuSel f870 "Stairs" "Toilet" "Plant" "Stove"
if %ERRORLEVEL% == 1 goto c2trystairs
if %ERRORLEVEL% == 2 goto c2hidetoilet
if %ERRORLEVEL% == 3 goto c2hideplant
if %ERRORLEVEL% == 4 goto c2hidestove

:c2housemapic
echo.
echo That was an invalid command, try again.
pause
goto c2housemap

:c2trystairs
cls
echo Keegan: They would catch me quickly, considering how tall this house is.
pause
goto c2housemap

:c2hidetoilet
cls
echo Keegan goes into the bathroom and hides in the toilet, then he realises that he can't fit in the toilet. A german goes in and laughs at Keegan.
pause
echo.
echo Keegan gets captured.
pause
echo.
echo Want to try again?
echo.
echo commands: yes, no
cmdMenuSel f870 "Yes" "No"
if %ERRORLEVEL% == 1 goto c2housemap
if %ERRORLEVEL% == 2 goto c2deathmessage

:c2deathmessage
cls
echo Goodbye! If you want to come back, use this code at the title screen: 61042
pause
exit

:c2hideplant
cls
echo Keegan hides behind the tall plant.
pause
echo.
echo A german walks in and sees him instantly
pause
echo.
echo Keegan gets captured.
pause
echo.
echo Want to try again?
echo.
echo commands: yes, no
cmdMenuSel f870 "Yes" "No"
if %ERRORLEVEL% == 1 goto c2housemap
if %ERRORLEVEL% == 2 goto c2deathmessage

:c2hidestove
echo Keegan opens the stove door and crawls in.
pause
echo.
echo A german walks in and looks around the whole house.
pause
echo.
echo The german walks out.
pause
echo.
echo Keegan: Few. That was close. I should explore the house more.
pause
cls
echo End of Chapter 2.
pause
cls
echo Chapter 3 Code: 47728
pause
goto Chapter3

:Chapter3
cls
echo Chapter 3: The House
echo.
echo Keegan: Well, now I can explore this house without having to worry about any Nazi's catching me.
pause
echo.
echo Keegan: I'm going to check this house out.
pause
goto c3map

:c3map
cls
echo -------------------
echo [  A          A   ]  Plant (Left) Stove (Right)
echo [                 ]
echo [        -----D----
echo [   S    [        ] Stairs (Left)
echo D K      [    A   ] Toilet
echo -------------------
echo.
echo commands: stairs, toilet, plant, stove
cmdMenuSel f870 "Stairs" "Toilet" "Plant" "Stove"
if %ERRORLEVEL% == 1 goto c3uphouse
if %ERRORLEVEL% == 2 goto c3toilet
if %ERRORLEVEL% == 3 goto c3plant
if %ERRORLEVEL% == 4 goto c3stove


:commandinvalidc3
echo.
echo That was an invalid command, try again
pause
goto c3map

:c3toilet
cls
echo Keegan: Well, glad I didn't hide here. I probably did in a different timeline, but, whatever.
pause
goto c3map

:c3plant
cls
echo Keegan: That's one mighty plant. I wonder why it's here.
pause
goto c3map

:c3stove
cls
echo Keegan: I'm supprised that this thing saved me. That guy was too dumb to check it.
pause
goto c3map

:c3uphouse
cls
echo Keegan: Now it would be a good time to go up.
pause
echo.
echo Keegan walks up the stairs.
pause
goto c3uphousemap

:c3uphousemap
cls
echo ---------------------
echo [                   ]
echo [         E         ]
echo [S K               .D
echo ---------------------
echo.
echo There is a door.
echo.
echo commands: east, examine
cmdMenuSel f870 "East" "Examine"
if %ERRORLEVEL% == 1 goto c3uphousemap2
if %ERRORLEVEL% == 2 goto c3uphouseexamine

:c3uphouseexamine
cls
echo Keegan: This room looks destroyed...
pause
goto c3uphousemap

:c3uphousemap2
cls
echo ---------------------
echo [                   ]
echo [         E         ]
echo [S                K D
echo ---------------------
echo.
echo Keegan nears the door.
echo.
echo commands: open, examine
cmdMenuSel f870 "Open" "Examine" "West"
if %ERRORLEVEL% == 1 goto exitc3toc4
if %ERRORLEVEL% == 2 goto c3uphouseexamine2
if %ERRORLEVEL% == 3 goto c3uphousemap

:c3uphouseexamine2
cls
echo Keegan: This room looks destroyed...
pause
goto c3uphousemap2

:exitc3toc4
cls
echo Keegan: Well, lets see whats behind this door.
pause
echo.
echo Keegan opens the door and looks in amazment at a glitching portal.
pause
echo.
echo Keegan: Uhhhhhhh, Kam?
pause
echo.
echo Kameron: *On Earpiece* Yeah, what's wrong?
pause
echo.
echo Keegan: There is a weird looking portal here.
pause
echo.
echo Kameron: You might want to be careful, it could suck you in and bring you to a different time and place. You might want to get away from that thing.
pause
echo.
echo Keegan: Got it.
pause
echo.
echo Keegan rushes to the door, but it's jammed.
pause
echo.
echo Keegan: Shit! It's jammed.
pause
echo.
echo The portal starts glitching even harder.
pause
echo.
echo Keegan: Kameron! I need you to get me out of here.
pause
echo.
echo Kameron: The systems are jammed!
pause
echo.
echo The portal then sucked Keegan in.
pause
cls
echo End of Chapter 3
echo.
echo Before continuing, please save this code somewhere so you can continue where you left off: 09315
pause

:c4start
cls
echo Chapter 4: Time Traveling Portals
echo.
echo Keegan flew out of the portal, along with a table and two chairs.
pause
echo.
echo Keegan: Ouch.
pause
echo.
echo Keegan heared static on his Earpiece. Keegan checked the static. Then, something connected.
pause
echo.
echo Kameron: God dang, it took a minute just to reconnect.
pause
echo.
echo Keegan: Oh, hey bro.
pause
echo.
echo Kameron: Hey, do you mind checking the screen on your glove? I just need to know where you are.
pause
echo.
echo Keegan checked the time on his glove, and it said 12:00 pm, August 13, 3999
pause
echo.
echo Keegan: Well, it looks like I'm in the year 3999
pause
echo.
echo Kameron: The what? Ok, you need to find another portal to get out of there.
pause
echo.
echo Keegan: Alright, I will try.
pause
echo.
echo Keegan enters the water.
pause

:c4startmap
cls
echo Area 1: Cave
echo.
echo -----------------
echo [ ~~~~~~~~~~~:  ]
echo [ ~~~~~~~~~~~:  ]
echo [ ~~~~~K~~~~~: .L
echo [ ~~~~~~~~~~~---]
echo [ ~~~~~~~~~~~   ]
echo -----------------
echo "~" means that there is water there. "L" means that you leave that area.
echo.
cmdMenuSel f870 "East" "Examine" ""
if %ERRORLEVEL% == 1 goto c4startmap2
if %ERRORLEVEL% == 2 goto c4startmape
if %ERRORLEVEL% == 3 goto c4secret

:c4secret
cls
echo ???: God dang it, I shouldn't have took that quiz, now I'm trapped down here.
pause
echo.
echo ??: Don't worry, I got trapped down here too.
pause
echo.
echo Keegan: Hello???
pause
echo.
echo ??: Did you hear that?
pause
echo.
echo ???: Yeah.
pause
echo.
echo ??: Might want to see who is talking.
pause
echo.
echo ???: First, who are you?
pause
echo.
echo Keegan: (I should get out of here.)
pause
goto c4startmap

:c4startmape
cls
echo Keegan: I should get out of this water.
pause
goto c4startmap

:c4startmap2
cls
echo Area 1: Cave
echo.
echo -----------------
echo [ ~~~~~~~~~~~:  ]
echo [ ~~~~~~~~~~~:  ]
echo [ ~~~~~.~~~~~:K L
echo [ ~~~~~~~~~~~---]
echo [ ~~~~~~~~~~~   ]
echo -----------------
echo.
cmdMenuSel f870 "West" "Exit Area"
if %ERRORLEVEL% == 1 goto c4startmap
if %ERRORLEVEL% == 2 goto c4cityenter

:c4cityenter
cls
echo Keegan exits the cave to find a big town.
pause
echo.
echo Keegan: Wow.
pause
goto c4citymapvaribles

:c4citymapvaribles
set c4ts=0
set c4h=0
set basekey=0
set ironamount=0
set pocketboyuse=0
goto c4citymap

:c4citymap
cls
echo Area 2: The Town
echo ---------                 ^                 ---------
echo [ Town  [                                   ]       ]
echo [ Shop  [                                   ] Hotel ]
echo [       [                                   ]       ]
echo [       [                                   ]       ]
echo [       [                                   ]       ]
echo [       [                                   ]       ]
echo [       [                                   ]       ]
echo [       [                                   ]       ]
echo [       [                                   ]       ]
echo [       [                                   ]       ]
echo [--------                                   --------]
echo [                     K                             ]
echo [                                                   ]
cmdMenuSel f870 "Town Shop" "Hotel" "Move Up"
if %ERRORLEVEL% == 1 goto c4townshopentermsg
if %ERRORLEVEL% == 2 goto c4hotelenter
if %ERRORLEVEL% == 3 goto c4citymap2

:c4townshopentermsg
cls
if "%c4ts%" == "1" goto c4townshopentermsg
echo Keegan opens the door to the Town Shop.
pause
echo.
echo Clerk: Sorry Outsider, shop's closed.
pause
echo.
echo Keegan: Sorry sir.
pause
echo.
echo Keegan was about to leave, when the clerk spoke again.
pause
echo.
echo Clerk: Wait, what's your name, Outsider?
pause
echo.
echo Keegan: Keegan, Keegan Miller.
pause
echo.
echo Clerk: Are you a friend or foe?
pause
echo.
echo Keegan: Friend.
pause
echo.
echo Clerk: Well, you don't look like you would hurt a civilian, so can you help me with something?
pause
echo.
echo Keegan: Yeah sure, why?
pause

:c4tsquestrequest
echo.
echo Clerk: There have been some people starting a ememy team. You look like you could take them down, can you help me?
pause
echo.
echo Do you help him?
cmdMenuSel f870 "Yes" "No"
if %ERRORLEVEL% == 1 goto c4townshopacceptquest
if %ERRORLEVEL% == 2 goto c4townshopdeclinequest

:c4townshopacceptquest
cls
set questaccept=1
echo Keegan: Yeah sure, let's do this.
pause
echo.
echo Clerk: Thank you. I stole a key from one of the guard's, it should help you reach the boss because it doesn't open the main door. Here you go.
set basekey=1
pause
echo.
echo Clerk: If you want anything, just ask, when you take care of the base. They are tracking the purchases.
pause
echo.
echo Keegan: Alright.
set c4ts=1
pause
goto c4townshopresponces

:c4townshopdeclinequest
cls
echo Keegan: Nope, sorry.
pause
echo.
echo Clerk: Oh, let me know if you change your mind. Is there any other way I can help you?
pause
set questaccept=0
goto c4townshopresponces0.5

:c4townshopentermsg
cls
echo Clerk: Hello, how can I help you?
goto c4townshopresponces

:c4townshopresponces0.5
cls
goto c4townshopresponces

:c4townshopresponces0.6
cls
echo Clerk: Is there any other way I can help you?

:c4townshopresponces
echo.
cmdMenuSel f870 "Buy" "Talk" "Leave"
if %ERRORLEVEL% == 1 goto c4tsmenu
if %ERRORLEVEL% == 2 goto c4tstalk
if %ERRORLEVEL% == 3 goto c4tsleave

:c4tsleave
cls
echo Clerk: Have a good day!
pause
goto c4citymap


:c4tsmenu
echo Which list do you want to view?
cmdMenuSel f870 "Main" "Tools" "Other" "Back"
if %ERRORLEVEL% == 1 goto c4tsmenumain
if %ERRORLEVEL% == 2 goto c4tsmenutools
if %ERRORLEVEL% == 3 goto c4tsmenuother
if %ERRORLEVEL% == 4 goto c4townshopresponces0.6

:c4tsmenumain
cls
echo Apple: A fruit that will be useless to you. Cost: O 2
echo Hammer (Quest Item): A weak iron hammer that will break in one use, but, can break certain walls. Cost: 5 -
echo Pocket Boy (Special Item): The Pocket Boy can be used for many different things, but you have to recode it to do things. You can plug it in to special terminals to automaticly recode it. Cost: Free
echo O = cyber coin  - = iron ingot
echo Clerk: When there are cheaper items, they will have different currency. Some will even cost some materials, because there is a shortage of materials here. So we have to hand craft items.
cmdMenuSel f870 "Apple" "Hammer" "Pocket Boy"
if %ERRORLEVEL% == 1 goto c4tsno1
if %ERRORLEVEL% == 2 goto c4tshammercheck
if %ERRORLEVEL% == 3 goto c4pocketboyunlock

:c4tsno1
echo.
echo You don't have enough cyber coins.
pause

:buycheck
cls
if "%quest1complete%" == "0" goto c4tsno2

:c4tshammercheck
cls
if "%quest1complete%" == "0" goto c4tsno2
if "%ironamount%" == "5" goto c4hammercraft
echo Clerk: Sorry, you don't have enough iron.

:c4tsno2
cls
echo Clerk: Sorry, I can't do that until you destroy the ememy base.
pause
goto c4tsmenu

:c4citymap2
cls
echo -------------------------------------------
echo [                                         ]
echo [                  Base                   ]
echo [                                         ]
echo [-----------------------------------------]
echo.
echo -                   ^                     -
echo.
echo.
echo.
echo                      K
echo                      /
cmdMenuSel f870 "Enter Base" "Move Left" "Move Right" "Move Back" "Examine"
if %ERRORLEVEL% == 1 goto c4enterbasecheck
if %ERRORLEVEL% == 2 goto c4citymap3
if %ERRORLEVEL% == 3 goto c4citymap4
if %ERRORLEVEL% == 4 goto c4citymap
if %ERRORLEVEL% == 5 goto c4citymap2examine

:c4enterbasecheck
cls
if "%pocketboyuse%" == 1 goto c4enterbase1
if "%pocketboy%" == 1 goto c4enterbaseno
echo Keegan: There is no reason to go in here.
pause
goto c4citymap2

:c4enterbaseno
cls
if "%questaccept%" == 1 goto c4enterbaseno2
if "%questaccept%" == 0 goto c4basenoboi
echo Keegan: There is no reason to go in here.
pause
goto c4citymap2

:c4basenoboi
cls
echo Keegan: There is no reason to go in here.

:c4enterbaseno2
cls
echo Keegan: There must be a way in.
pause
goto c4citymap2

:c4enterbase
cls
echo Keegan uses his Pocket Boy, and unlocks the door.
pause
echo.
echo The End for now. Thanks for playing! Chapter 4 will be out fully soon.
pause
exit

:c4citymap2examine
cls
echo Keegan: It looks like there is a terminal there.
cmdMenuSel f870 "Use" "Back"
if %ERRORLEVEL% == 1 goto c4terminal
if %ERRORLEVEL% == 2 goto c4citymap2

:c4tstalk
cls
echo Clerk: What do you want to talk about?
echo.
cmdMenuSel f870 "Shop" "Quest" "Exit"
if %ERRORLEVEL% == "1" goto c4tstalk1
if %ERRORLEVEL% == "2" goto c4tstalkquest
if %ERRORLEVEL% == "3" goto c4townshopresponces0.6

:c4tstalk1
cls
echo Keegan: What was your shop like before this happened?
pause
echo.
echo Clerk: Well, there was more people here. Now, people are hiding in their homes. They force them to stay in their houses, and when they run out of food, they are brought some food that they will let them have. Which for food, there is bread, or soup. For a drink, there would be water, or milk.
pause
echo.
echo Keegan: Oh, Ok.
pause
goto c4tstalk

:c4tstalkquest
cls
echo Keegan: About the quest?
pause
if "%questaccept%" == "0" goto c4tstalkquesttalk
echo.
echo Clerk: Well, I need you to take out the base.
pause
echo.
echo Keegan: Ok.
pause
goto c4tstalk

:c4tstalkquesttalk
echo.
echo Clerk: Well, I need you to accept the quest. Will you?
pause
echo.
cmdMenuSel f870 "Yes" "No" "About Quest"
if %ERRORLEVEL% == 1 goto c4townshopacceptquest
if %ERRORLEVEL% == 2 goto c4tstalkquestno
if %ERRORLEVEL% == 3 goto c4tstalkquestabout

:c4tstalkquestabout
cls
echo Keegan: What was the quest about again?
pause
echo.
echo Clerk: Ok, let me remind you. There have been some people starting a ememy team, and they are attacking other people.
pause
goto c4tstalkquesttalk

:c4tstalkquestno
cls
echo Keegan: No, sorry.
pause
echo.
echo Clerk: *Sighs* Alright, but until you take the quest and finish it, you won't be able to get anything from here.
pause
goto c4tstalk

:c4citymap3
cls
echo --------------------------
echo [
echo [
echo [
echo [ Cave -     E         K -
echo [
echo [
echo [
echo [
echo --------------------------
cmdMenuSel f870 "Enter Cave" "Move Back" "Examine"
if %ERRORLEVEL% == 1 goto c4cavemap
if %ERRORLEVEL% == 2 goto c4citymap2
if %ERRORLEVEL% == 3 goto c4citymap3examine

:c4citymap3examine
cls
echo Keegan: Look's like there is a dead soldier here. It looks like a bear killed him. Maybe I should take a closer look...?
cmdMenuSel f870 "Examine" "Go Back"
if %ERRORLEVEL% == 1 goto c4examinesoldier
if %ERRORLEVEL% == 2 goto c4citymap3

:c4examinesoldier
if %pocketboy% == 1 goto c4esnoget
set pocketboy = 1
cls
echo Keegan: Looks like he has a Pocket Boy, nice. Maybe it has access somewhere...
pause
echo.
echo Keegan: Ehhh, maybe not, but I will take it anyways.
pause
if %questaccept% == 1 goto c4changecodepb
goto c4citymap3

:c4esnoget
cls
echo Keegan: Damn, no weapon. I guess I'll search elsewhere.

:c4changecodepb
echo.
echo Keegan: Wait a minute! Maybe it will have access to the base. Let's link it up, because it's stuck.
pause
echo.
echo Keegan links it up to his Pocket Boy and assigns the code to his Pocket Boy.
set pocketboyuse=1
pause
echo.
echo Keegan: Nice.
pause
goto c4citymap3

:c4esinuse
echo.
echo Keegan: Oh wait. I already used my Pocket Boy on him. Nevermind.
pause
goto c4citymap3

:c4citymap4
cls
echo --------------------------------------
echo                                      ]
echo                                      ]
echo                                      ]
echo                                      ]
echo                                      ]
echo -    K                               ] Cracked Wall
echo                                      ]
echo                                      ]
echo                                      ]
echo                                      ]
echo --------------------------------------
cmdMenuSel f870 "Move Left" "Examine Wall"
if %ERRORLEVEL% == 1 goto c4citymap2
if %ERRORLEVEL% == 2 goto c4wall

:c4wall
cls
echo Keegan: Looks like I could break this wall with something strong.
pause
if "%hammer%" == "1" goto c4breakwall
echo.
echo Keegan punches the wall, and hurts his hand.
pause
echo.
echo Keegan: Ouch! Look's like if I had something stronger, I could break it.
pause
goto c4citymap4

:c4breakwall
echo.
echo Keegan hits the wall with the hammer, and an opening appears, and the hammer breaks.
pause
echo.
echo Keegan: Nice! Now I can find that portal.
pause
goto c4osmap

:c4hotelenter
cls
if "%hotelenter%" == 1 goto c4hotelactions
echo Manager: Welcome! I haven't seen you before. How did you get in anyway? The town has been blocked off by a wall.
pause
echo.
echo Keegan: It's a long story.
pause
echo.
echo Manager: Anyway, because of the people attacking others, and forcing them into their houses, you can sleep here for free, when you need to.
pause
echo.
echo Keegan: Alright.
pause
goto c4hotelactions

:c4hotelactions
cls
echo Manager: Is there anything I can help you with?
echo.
cmdMenuSel f870 "Sleep" "Talk" "Exit"
if %ERRORLEVEL% == 1 goto c4hotelsleepcheck
if %ERRORLEVEL% == 2 goto c4hoteltalk
if %ERRORLEVEL% == 3 goto c4hotelleave

:c4hotelsleepcheck
cls
if "%quest1sleep%" == "1" goto c4hotelsleep
echo Keegan: I'm not tired yet.
pause
goto c4hotelactions

:c4hotelsleep
cls
echo Keegan: Can I go sleep here?
pause
echo.
echo Manager: Sure, here's your room key.
pause
echo.
echo Keegan: Thanks.
pause
echo.
echo Keegan heads to his room, and falls asleep.
set quest1sleep=0
pause

:c4hoteltalk
cls
echo Manager: Sorry, I don't have time for talking.
pause
goto c4hotelactions

:c4cavemap
cls
echo -
echo []
echo []
echo []
echo []
echo []
echo []
echo -

:c4enterbase1
cls
echo Keegan: Time to use the computer! First, I should go examine the PC. (You can enter the base, if you examine the the Computer.)
pause
goto c4citymap2

:c4terminal
cls
if "%pocketboyuse%" == "0" goto c4terminalfail
echo Keegan: Time to let it start up.
echo --------------------------------
echo Loading...
ping localhost -n 5 >nul
cls
echo Keegan: It's taking a bit...
echo ----------------------------
echo Loading...
ping localhost -n 5 >nul
cls
echo Keegan: Finally!
echo ----------------
echo Complete. Press any key to continue.
pause >nul

:c4terminalmenu
cls
echo Welcome to the C4 Computer OS! This computer is currently programed to be a door locker. You can remove this message in the settings.
pause
cls
echo What do you want to do?
cmdMenuSel f870 "Unlock Door" "Message Owner" "Exit"
if %ERRORLEVEL% == 1 goto c4terminaldoor
if %ERRORLEVEL% == 2 goto c4messagefail
if %ERRORLEVEL% == 3 goto c4terminalexit

:c4terminalexit
cls
echo Bye! We hope to see you again soon.
pause >nul
goto c4citymap2

:c4terminalfail
cls
echo Keegan: It's not a good time to use this yet.
pause
goto c4citymap2

:c4messagefail
cls
echo The message system is not set up, please contact the owner to set up the messaging system.
pause
goto c4terminalmenu

:c4terminaldoor
cls
echo Please insert your Pocket Boy to unlock the door.
pause
if "%pocketboyuse%" == "1" goto c4terminaldooropen
echo.
echo Error E004: Incorrect Code, or Pocket Boy not inserted
pause
goto c4terminalmenu

:c4terminaldooropen
cls
echo Door opened!
pause
echo Keegan: Finally, lets get going!
pause
goto c4baseenter

:c4baseenter
cls
echo Keegan opens the base door and encounters a soldier. Keegan tackles the soldier to ask questions.
pause
echo.
echo Keegan: I need information.
pause
echo.
echo Soldier: Anything you want, man.
pause
echo.
echo Keegan: Where's a portal to get me out of here?
pause
echo.
echo Soldier: What portal?
pause
echo.
echo Keegan: Talk, dammit! There's no use in resisting.
pause
echo.
echo Soldier: I really don't know about any portal. I'm only a new recruit!
pause
echo.
echo Keegan: Where's a veteran?
pause
echo.
echo Soldier: Probably talking to the boss. He's at the top floor.
pause
echo.
echo Keegan: Thank you.
pause
echo.
echo Should you kill the soldier? (This choice won't affect the story.)
cmdMenuSel f870 "Kill" "Don't Kill"
if %ERRORLEVEL% == 1 goto c4baseenterk1
if %ERRORLEVEL% == 2 goto c4baseenterk2

:c4baseenterk1
cls
echo Keegan: I'm sorry, little one.
pause
echo.
echo (Keegan snaps the soldier's neck, and steals his gun.)
pause
echo.
echo Keegan: Nice. It's a M1911, err, a more modern version of it. Let's do this.
pause
goto c4base1

:c4baseenterk2
cls
echo Keegan: If you give me your pistol, I can guarantee your safety.
pause
echo.
echo Soldier: Okay.
pause
echo.
echo (The soldier hands the gun to Keegan.)
pause
echo.
echo Keegan: Nice M1911, now scram!
pause
echo.
echo (The soldier runs out of the building.)
pause
echo.
echo Keegan: Let's do this.
pause
goto c4base1

:c4base1
cls
echo Area 3: The Base
echo --------------------------
echo [   E               E    ]
echo [                        ]
echo [ K                     D]
echo [                        ]
echo [                        ]
echo --------------------------
cmdMenuSel f870 "Enter Door" "Examine" "Leave Area"

:c4hotelleave
cls
echo Manager: I hope you have a great day!
pause
goto c4citymap

:c4pocketboyunlock
cls
if "%pocketboy%" == "1" goto c4usekeypocketboy
echo Shop Clerk: Sorry, I can't sell you a Pocket Boy. The enemies would know.
pause
goto c4townshopresponces0.6

:c4usekeypocketboy
cls
if "%pocketboyuse%" == "1" goto c4nousekeypocketboy
if "%questaccept%" == "0" goto c4nousekeypocketboy2
if "%basekey%" == "0" goto c4nousekeypocketboy3
echo Shop Clerk: Since you have the Pocket Boy and the Base Key, I can apply the key to the Pocket Boy.
pause
echo.
echo Keegan: Thank you dude, you're the most helpful!
pause
echo.
echo Shop Clerk: No problem, just hand over the Pocket Boy, and the key.
pause
echo.
echo Keegan: Ok.
pause
echo.
echo (Keegan gives the Key and the Pocket Boy to the Shop Clerk.)
pause
echo.
echo Shop Clerk: Thanks, now give me a moment while I work on this.
pause
echo.
echo Keegan: Roger.
pause
echo.
echo (Keegan waits patiently...)
ping localhost -n 5 >nul
cls
echo (Keegan still waits patienly, while getting a little annoyed...)
ping localhost -n 5 >nul
cls
echo Shop Clerk: I'm done!
pause
echo.
echo Keegan: Thanks.
pause
echo.
echo Shop Clerk: No problem.
pause
goto c4townshopresponces0.6
