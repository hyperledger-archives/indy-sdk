# Indy Python wrapper functional test

This is a Python wrapper functional test for Indy. The tests are not driven by any unit test framework but are standalone python scripts.

This Python wrapper functional test currently requires python 3.6, base58.

### How to run

After building successfully the Indy SDK for Python, you need to run the commands below so that could run the tests:

- Install base58 dependency with pip install: 
```
     python3.6 -m pip install base58
```
- Setup PYTHONPATH: 
```
    export PYTHONPATH=$PYTHONPATH:your_repo_location/Automation-Tests
```

#### Then run:
- Run one test case:
```
    python3.6 Automation-Tests/test_scripts/functional_tests/wallet/open_wallet.py
```
- Run a folder test case using test_runner.py:
```
    python3.6 Automation-Tests/test_runner.py -d Automation-Tests/test_scripts/functional_tests/wallet
```
- Run all test cases in the project using test_runner.py:
```    
    python3.6 Automation-Tests/test_runner.py -rd
```

##### This is the usage of test_runner.py
```
test_runner.py [-h] [-d [DIRECTORY]] [-rd [RECUR_DIRECTORY]]
                      [-t [TIMEOUT]] [-html] [-l]

optional arguments:
  -h, --help            show this help message and exit
  -d [DIRECTORY], --directory [DIRECTORY]
                        directory of test scenarios (not recursive)
  -rd [RECUR_DIRECTORY], --recur_directory [RECUR_DIRECTORY]
                        directory of test scenarios (recursive)
  -t [TIMEOUT], --timeout [TIMEOUT]
                        timeout for each scenario (default: 300s)
  -html, --html_report  if this flag is missing, html report would not be
                        generated
  -l, --keep_log        keep all log file
```
#### Generate the htlm report:
- Get the summary report for all the run
```
    python3.6 your_repo_location/functional_tests/reporter.py
```
- Get the summary report for a group of test cases.
```
    python3.6 Automation-Tests/reporter.py -n *wallet*
```
- Get the summary report on a giving date
```
    python3.6 Automation-Tests/reporter.py -n *2017-12-14*
``` 

##### This is the usage of reporter.py
```
reporter.py [-h] [-n [NAME]]

optional arguments:
  -h, --help            show this help message and exit
  -n [NAME], --name [NAME]
                        filter json file by name
```
