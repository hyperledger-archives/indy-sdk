#!/usr/bin/env python
import subprocess
from subprocess import Popen, PIPE

# gets the revision number by a git commit hash
def get_revision_number():
    process = subprocess.Popen(['git','log','--pretty=format:\'%h\'','-n','1'], stdin=PIPE, stdout=PIPE)
    result = process.communicate()
    if len(result) > 1:
        result = result[0].decode("UTF-8").strip('\'')
        print("revision:\t%s" % result)
        return result
    else:
        print("Git not found, commit not found")
        return '0' 

