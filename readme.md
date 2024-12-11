Usage

`tasks.json` can be replaced with any other file to hold the tasks.


Create:

cargo run create tasks.json `id`


Read:

cargo run read tasks.json `id`


Update task description:

cargo run update tasks.json `id` `"new description"`


Update task status:

cargo run update-status tasks.json `id` `"new status"`


Delete:

cargo run delete tasks.json `id`


Create file:

cargo run create-file `file_name.json`


Read file:

cargo run read-file `file_name.json`


Delete file

cargo run delete-file `file_name.json`

