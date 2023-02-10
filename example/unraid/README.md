# Example unRAID configuration

- `my-do-alerts-discord.xml` is an unRAID Docker container template. You can drop it in `/boot/config/plugins/dockerMan/templates-user/` (or just make a new docker container and configure it yourself, it's just like 5 variables and a custom icon URL).
- `user_script/*` is a script for use with the "User Scripts" unRAID plugin. Install that plugin via Community Applications, then make a new script and drop the files into the newly created script directory (which will be something like `/boot/config/plugins/user.scripts/scripts/my_new_script/`). You can have it run more frequently than hourly by setting a custom schedule from the UI, e.g. every 5 minutes with `*/5 * * * *`.
