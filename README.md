# BitBar Composite Plugin

An attempt to display multiple BitBar plugins in a single menu entry.

Below screenshot is the result of combining 2 other plugins: TeamCity BitBar and Sonarqube BitBar plugins.

![BitBar Composite Plugin](https://i.imgur.com/MzdpMS1.png)

## Installation

- Follow instructions at [BitBar Github](https://github.com/matryer/bitbar-plugins) to install BitBar and setup plugins folder
- Download pre-built binary from [BinTray](https://dl.bintray.com/hpcsc/bitbar-composite-plugin/)
- Rename downloaded binary to `bitbar-composite-plugin.{refresh}.crust` where `{refresh}` is refresh period like `10s`, `1h` etc and move it to BitBar plugins folder
- Create a file with name `.bitbar-composite-plugin.yaml` in BitBar plugins folder with content similar to below:

    ```
    plugins:
    - displayName: "Plugin: TeamCity"
      command: /path/to/teamcity/plugin/executable
      showInSubMenu: true
    - displayName: "Plugin: Sonarqube"
      command: /path/to/sonarqube/plugin/executable
      showInSubMenu: true
    - displayName: Echo
      command: bash
      args:
      - -c
      - echo -n 'my command output'
    ```

    Here we provide paths to 2 other BitBar plugins. Output from each of those plugins will be displayed in a submenu of this composite plugin since `showInSubMenu` is on. If you prefer seeing output not nested, set that to `false`.

    You can also specify arbitrary command to be run and included in this composite plugin as in 3rd configuration above
