# Threading model

Libindy uses four threads during its lifecycle -- client thread, command thread, pool thread and thread for expensive operations. Beneath you can see diagrams with explanation of these threads' work.

![Threading model](threading_model.svg)

![Pool communication model](pool_communication_threading.svg)