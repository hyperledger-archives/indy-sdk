#! /usr/local/bin/python3
# findLargeFiles.py - given a folder name, walk through its entire hierarchy
#                   - print folders and files within each folder
# python add.trace.statements.to.src.py /Users/iosbuild1/forge/work/code/evernym/sdk/vcx/libvcx/src
# python add.trace.statements.to.src.py /Users/iosbuild1/forge/work/code/evernym/sdk/.macosbuild/vcx-indy-sdk/libindy/src

import os
import sys

# print(sys.argv)

def recursive_walk(folder, rustLogFunction):
    traceNumber = 0
    for folderName, subfolders, filenames in os.walk(folder):
        if subfolders:
            for subfolder in subfolders:
                recursive_walk(subfolder, rustLogFunction)
        print('\nFolder: ' + folderName)
        for filename in filenames:
            if (filename.endswith(".newrs")):
                print("Ignoring file: " + folderName + '/' + filename)
                continue

            print("Found file: " + folderName + '/' + filename)
            f = open(folderName + '/' + filename, "r")
            print("Copying to file: " + folderName + '/' + filename + ".newrs")
            copy = open(folderName + '/' + filename + ".newrs", "w")
            fileLineNumber = 0
            previousLine = ""
            previousTrimmedLine = ""
            insideExternCurly = 0
            atTopOfFile = 1
            waitForSemiColon = 0
            eatingCurlys = 0
            eatingLines = 0
            openCurlys = -1
            ignoreEnding = 0
            foundFirstOpeningCurl = 0
            insideUseStatement = 0
            for line in f:
                fileLineNumber += 1
                trimmedLine = line.strip()

                if (foundFirstOpeningCurl == 0 and trimmedLine.count('{') > 0):
                    foundFirstOpeningCurl = 1

                if (line.startswith("use") and not trimmedLine.endswith(';')):
                    insideUseStatement = 1

                if (trimmedLine == "extern {"):
                    insideExternCurly = 1
                if (trimmedLine.endswith(",")):
                    waitForSemiColon = 1
                if (
                    not line.startswith("use") and
                    not line.startswith("extern") and
                    not line.startswith("type ") and
                    waitForSemiColon == 0 and
                    len(trimmedLine) > 0
                ):
                    atTopOfFile = 0

                if (trimmedLine.count("\"") == 1 and trimmedLine.endswith("\"")):
                    eatingLines = 1

                if (line.startswith("pub type ") or line.startswith("pub const ") or line.startswith("const ")):
                    ignoreEnding = 1

                if (trimmedLine.startswith("macro_rules!") or trimmedLine.startswith("impl Node")):
                    eatingCurlys = 1

                if (eatingCurlys == 1):
                    if (openCurlys == -1):
                        openCurlys = trimmedLine.count('{')
                    else:
                        openCurlys += trimmedLine.count('{')
                    openCurlys -= trimmedLine.count('}')

                if (
                    trimmedLine.endswith(";") and
                    waitForSemiColon == 0 and
                    atTopOfFile == 0 and
                    eatingCurlys == 0 and
                    eatingLines == 0 and
                    not filename == "sodium_type.rs" and
                    not trimmedLine == "};" and
                    not trimmedLine == "})?;" and
                    not trimmedLine.startswith(".") and
                    not trimmedLine.startswith("{") and
                    not trimmedLine.startswith(").") and
                    not trimmedLine.startswith("}).") and
                    not trimmedLine.startswith(");") and
                    not trimmedLine.startswith("});") and
                    not trimmedLine.startswith("}));") and
                    not trimmedLine.startswith("static ref") and
                    not trimmedLine.startswith("pub static ref") and
                    not trimmedLine.startswith("pub type ") and
                    not trimmedLine.startswith("type ") and
                    not trimmedLine.startswith("fn ") and
                    not trimmedLine.startswith("extern") and
                    not trimmedLine.startswith("use") and
                    not trimmedLine.startswith("return") and
                    not trimmedLine.startswith("pub const") and
                    not line.startswith("pub mod") and
                    not line.startswith("sodium_type!") and
                    not line.startswith("const ") and
                    not line.startswith("pub static") and
                    not line.startswith("pub struct") and
                    not line.startswith("fn matches") and
                    not line.startswith("//") and
                    not line.startswith("}") and
                    not line.startswith(")") and
                    not line.startswith("/*") and
                    not line.startswith("static") and
                    not line.startswith("mod ") and
                    not line.startswith("#[macro_use") and
                    not previousTrimmedLine.endswith(",") and
                    not previousTrimmedLine.endswith(".") and
                    not previousTrimmedLine.endswith("=") and
                    not previousTrimmedLine.endswith("?") and
                    not previousTrimmedLine.endswith(")") and
                    not previousTrimmedLine.endswith("(") and
                    not previousTrimmedLine.endswith("|") and
                    not previousTrimmedLine.endswith("\\") and
                    not previousTrimmedLine.endswith("}") and
                    not previousTrimmedLine.startswith("retun") and
                    not previousTrimmedLine.startswith("#[cfg") and
                    not previousLine.startswith("#[cfg") and
                    not previousLine.startswith("pub trait") and
                    not previousLine.startswith("impl")
                ):
                    traceNumber += 1
                    copy.write(rustLogFunction + "!(\"TRACE[" + str(traceNumber) + "]: ABOVE LINE[" + str(fileLineNumber) + "]: " + trimmedLine.replace("\\","\\\\").replace("\"","\\\"").replace("{","{{").replace("}","}}") + " -- FILE: " + folderName + "/" + filename + "\");\n")
                    #copy.write("match std::env::var(\"MOBILE_TRACE\") {Ok(val) => {" + rustLogFunction + "!(\"TRACE[" + str(traceNumber) + "]: ABOVE LINE[" + str(fileLineNumber) + "]: " + trimmedLine.replace("\\","\\\\").replace("\"","\\\"").replace("{","{{").replace("}","}}") + " -- FILE: " + folderName + "/" + filename + "\")}, Err(e) => {},}\n")

                copy.write(line)

                if (
                    trimmedLine.endswith(";") and
                    atTopOfFile == 0 and
                    eatingCurlys == 0 and
                    eatingLines == 0 and
                    ignoreEnding == 0 and
                    not filename == "sodium_type.rs" and
                    not trimmedLine == "};" and
                    not trimmedLine == "})?;" and
                    not trimmedLine.startswith("static ref") and
                    not trimmedLine.startswith("pub static ref") and
                    not trimmedLine.startswith("pub type ") and
                    not trimmedLine.startswith("type ") and
                    not trimmedLine.startswith("r#\"{\"") and
                    not trimmedLine.startswith("fn ") and
                    not trimmedLine.startswith("return") and
                    not trimmedLine.startswith("break") and
                    not trimmedLine.startswith("continue") and
                    not trimmedLine.startswith("extern") and
                    not trimmedLine.startswith("use") and
                    not trimmedLine.startswith("pub const") and
                    not line.startswith("pub mod") and
                    not line.startswith("sodium_type!") and
                    not line.startswith("const ") and
                    not line.startswith("pub static") and
                    not line.startswith("pub struct") and
                    not line.startswith("fn matches") and
                    not line.startswith("//") and
                    not line.startswith("/*") and
                    not line.startswith("static") and
                    not line.startswith("mod ") and
                    not line.startswith("});") and
                    not line.startswith("#[macro_use") and
                    insideExternCurly == 0 and
                    not previousTrimmedLine.startswith("return") and
                    not previousTrimmedLine.startswith("break") and
                    not previousTrimmedLine.startswith("continue") and
                    not previousLine.startswith("use logic") and
                    foundFirstOpeningCurl == 1 and
                    insideUseStatement == 0 and
                    not previousLine.startswith("pub trait") and
                    not previousLine.startswith("impl")
                ):
                    traceNumber += 1
                    copy.write(rustLogFunction + "!(\"TRACE[" + str(traceNumber) + "]: BELOW LINE[" + str(fileLineNumber) + "]: " + trimmedLine.replace("\\","\\\\").replace("\"","\\\"").replace("{","{{").replace("}","}}") + " -- FILE: " + folderName + "/" + filename + "\");\n")
                    #copy.write("match std::env::var(\"MOBILE_TRACE\") {Ok(val) => {" + rustLogFunciton + "!(\"TRACE[" + str(traceNumber) + "]: BELOW LINE[" + str(fileLineNumber) + "]: " + trimmedLine.replace("\\","\\\\").replace("\"","\\\"").replace("{","{{").replace("}","}}") + " -- FILE: " + folderName + "/" + filename + "\")}, Err(e) => {},}\n")

                if ( insideExternCurly == 1 and trimmedLine == "}" ):
                    insideExternCurly = 0
                if (trimmedLine.endswith(";")):
                    waitForSemiColon = 0
                    ignoreEnding = 0
                if (openCurlys == 0):
                    eatingCurlys = 0
                    openCurlys = -1
                if (trimmedLine.startswith("\"") and eatingLines == 1):
                    eatingLines = 0
                if (insideUseStatement == 1 and trimmedLine.endswith(';')):
                    insideUseStatement = 0

                previousTrimmedLine = trimmedLine
                previousLine = line
            f.close()
            copy.close()
            os.rename(folderName + '/' + filename + ".newrs", folderName + '/' + filename)

recursive_walk(sys.argv[1], sys.argv[2])

# find vcx/libvcx/src -name "*.rs"|wc -l

# f = open("...", "r")
# copy = open("...", "w")
# for line in f:
#     copy.write(line)
# f.close()
# copy.close()