cd ../../../samples/python && \
PYTHONPATH=.:../../wrappers/python/ python3 src/getting_started.py -t postgres_storage -l libindystrgpostgres.so -e postgresstorage_init -c '{"url":"localhost:5432"}' -s '{"account":"postgres","password":"mysecretpassword","admin_account":"postgres","admin_password":"mysecretpassword"}'

