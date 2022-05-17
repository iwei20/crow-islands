ID [a-zA-Z][a-zA-Z0-9_]*
%%
[ \t\n ]+ ;

\-?[0-9]+ |
\-?[0-9]+\. |
\-?[0-9]+\.[0-9]+ |
\-?\.[0-9]+ "DOUBLE"

"//".* "COMMENT"

light "LIGHT"
constants "CONSTANTS"
save_coord_system "SAVE_COORDS"
camera "CAMERA"
ambient "AMBIENT"

torus "TORUS"
sphere "SPHERE"
box "BOX"
line "LINE"
mesh "MESH"
texture "TEXTURE"

set "SET"
move "MOVE"
scale "SCALE"
rotate "ROTATE"
basename "BASENAME"
save_knobs "SAVE_KNOBS"
tween "TWEEN"
frames "FRAMES"
vary "VARY"

push "PUSH"
pop "POP"
save "SAVE"
generate_rayfiles "GENERATE_RAYFILES"

shading "SHADING"

phong|flat|gouraud|raytrace|wireframe {
strcpy(yylval.string, yytext); return SHADING_TYPE;}

setknobs "SETKNOBS"
focal "FOCAL"
display "DISPLAY"
web "WEB"

: "CO"

[a-zA-Z][\.a-zA-Z0-9_]* {
strcpy(yylval.string, yytext); return STRING;}


%%