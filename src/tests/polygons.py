
with open("torusfractal", "w") as fout:
    def recursive(steps=0):
        if steps == 5:
            return
        points = 2**steps
        for i in range(points):
            fout.write(f"torus {250 / points * (1 + 2 * i) - 250} 0 0 10 {250 / points}\n")
        recursive(steps + 1)
    recursive()
    fout.write("rotate x 45\n")
    fout.write("rotate y 20\n")
    fout.write("move 250 250 0\n")
    fout.write("apply\n")
    fout.write("save torusfractal.png\n")