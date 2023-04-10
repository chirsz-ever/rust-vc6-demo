#!python

import sys
import subprocess

unused_libs = ["userenv.lib", "legacy_stdio_definitions.lib", "bcrypt.lib"]

args = []
is_first_obj = True
for x in sys.argv:
    if x in unused_libs:
        continue
    if x.endswith(".o") and is_first_obj:
        args.append("3rdparty\\YY_Thunks_for_WinXP.obj")
        # args.append("src\\stub.obj")
        is_first_obj = False
    elif x == "/NXCOMPAT":
        args.append("src\\stub.obj")
    args.append(x)

args[0] = "LINK.EXE"

print(args)

r = subprocess.run(args, capture_output=True)
# replace('\\r\\n', '\n')
print(r.stdout.decode(errors='ignore'))
print(r.stderr.decode(errors='ignore'), file=sys.stderr)
sys.exit(r.returncode)

# sys.exit(subprocess.run(args, encoding="raw_unicode_escape").returncode)
