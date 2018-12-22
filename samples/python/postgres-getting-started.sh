.:../../wrappers/python/ python3 src/getting_started.py -t postgres_storage -l ../../samples/storage/storage-postgres/target/debug/libindystrgpostgres.dylib -e postgresstorage_init -c '{"url":"localhost:5432"}' -s '{"account":"postgres","password":"mysecretpassword","admin_account":"postgres","admin_password":"mysecretpassword"}'

