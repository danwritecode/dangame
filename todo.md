[] dynamic map system


[] add various game play states and create a system to go between them (home menu, gameplay, etc)
[] game debug window/ui to help with various state changes
    [] change gameplay state (home menu, gameplay, etc)
    [] change map
    [] change player
    [] add player
    [] remove player
    [] trigger damage

[] add damage system
[] add a "death" animation when out of HP

[] is hitting head on something detection

[] generic character system refactor #2

[] try to determine why jump is inconsistent


[x] add a "landing" animation by splitting off the end of the jump animation
[x] Flying kick
[x] see if we can make hitbox dynamic based on animation frames sequence
[x] build generic character system
[x] add second player
[x] Netcode
[x] Second remote player
[x] add input for IP address
[x] Refactor netcode to use separate thread - not possible
[x] Refactor netcode into struct
[x] Add server delay
[x] Add client delay (we don't need to send updates every frame)
[x] Add lerp
