MINUTE_LENGTH = 200
HOUR_LENGTH = 125
MARK_LENGTH = 25

for i in range(60):
    with open(f"clockscripts/clock{str(i).zfill(2)}", "w") as fout:
        fout.write("# Minute hand\n")
        fout.write(f"line 0 0 0 0 {MINUTE_LENGTH} 0\n")
        fout.write(f"rotate z {i * -6}\n")
        fout.write("apply\n")

        fout.write("# Hour hand\n")
        fout.write(f"line 0 0 0 0 {HOUR_LENGTH} 0\n")

        fout.write("# Edge markings\n")
        
        for j in range(12):
            fout.write("ident\n")
            fout.write(f"line 0 {245 - MARK_LENGTH} 0 0 245 0\n")
            fout.write("rotate z -30\n")
            fout.write("apply\n")

        fout.write("ident\n")
        fout.write("move 250 250 0\n")
        fout.write("apply\n")
        fout.write(f"save clockframes/clock{str(i).zfill(2)}.png\n")