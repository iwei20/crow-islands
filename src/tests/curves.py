top = 150
half_width = 125
half_top_dragger = 50
bottom = 200
leftx = 235
lefty = 160

FACTOR = 0.3
STEPS = 20
with open("vase", "w") as fout:
    for i in range(STEPS):
        fout.write(f"bezier {250 - half_width} {250 + top} {250 + half_top_dragger} {250 + top} {250 - leftx} {250 - lefty} 250 {250 - bottom} \n")
        fout.write(f"bezier {250 + half_width} {250 + top} {250 - half_top_dragger} {250 + top} {250 + leftx} {250 - lefty} 250 {250 - bottom} \n")
        fout.write(f"line {250 - half_width} {250 + top} 0 {250 + half_width} {250 + top} 0\n")
        top *= FACTOR
        half_width *= FACTOR
        half_top_dragger *= FACTOR
        bottom *= FACTOR
        leftx *= FACTOR
        lefty *= FACTOR
    fout.write("save vase.png\n")